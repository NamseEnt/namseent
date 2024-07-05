mod dependencies;

use crate::*;
pub use dependencies::*;
use luda_rpc::rkyv;
use std::{collections::HashMap, fmt::Debug, sync::Arc};
use tokio::{
    sync::oneshot,
    task::{AbortHandle, JoinHandle},
};

type SigRef<'a, T> = Sig<'a, T, &'a T>;

type Serializer = rkyv::ser::serializers::AllocSerializer<1024>;

pub static SERVER_CONNECTION_ATOM: Atom<ServerConnection> = Atom::uninitialized();

pub fn server_rpc<
    'a,
    Req: rkyv::Serialize<Serializer> + Send + 'a,
    Deps: Dependencies + 'a,
    Response: rkyv::Archive + Send + 'static + Debug,
    Error: rkyv::Archive + Send + 'static + Debug,
>(
    ctx: &'a RenderCtx,
    request: impl FnOnce(Deps) -> Option<Req>,
    dependencies: Deps,
    api_index: u16,
) -> SigRef<'a, Option<Result<Response, Error>>>
where
    <Response as luda_rpc::rkyv::Archive>::Archived: luda_rpc::rkyv::Deserialize<
        Response,
        luda_rpc::rkyv::de::deserializers::SharedDeserializeMap,
    >,
    <Error as luda_rpc::rkyv::Archive>::Archived:
        luda_rpc::rkyv::Deserialize<Error, luda_rpc::rkyv::de::deserializers::SharedDeserializeMap>,
{
    let (server_connection, _) = ctx.atom(&SERVER_CONNECTION_ATOM);
    let server_connection = server_connection.clone_inner();
    let (response, set_response) = ctx.state(|| None);
    let dependencies_changed = dependencies.changed(ctx);
    let deps_sig = ctx.controlled_memo(|_| {
        if dependencies_changed {
            return ControlledMemo::Changed(());
        }
        ControlledMemo::Unchanged(())
    });

    ctx.effect("server get", || {
        deps_sig.record_as_used();

        let Some(req) = request(dependencies) else {
            return;
        };

        let bytes = rkyv::to_bytes(&req).unwrap().to_vec();

        ctx.spawn(async move {
            let response = server_connection.request(api_index, bytes).await;
            set_response.set(Some(response));
            Result::<()>::Ok(())
        });
    });

    response
}

struct Request {
    packet_id: u32,
    bytes: Vec<u8>,
    response_tx: oneshot::Sender<Vec<u8>>,
}

struct ConnectionKeeper {
    abort_handle: AbortHandle,
    request_tx: tokio::sync::mpsc::UnboundedSender<Request>,
}

impl ConnectionKeeper {
    fn new(url: impl ToString) -> Self {
        let url = url.to_string();
        let (request_tx, mut request_rx) = tokio::sync::mpsc::unbounded_channel::<Request>();

        let join_handle: JoinHandle<()> = tokio::spawn({
            async move {
                let mut requests = HashMap::<u32, Request>::new();
                loop {
                    let (sender, mut receiver) =
                        match namui::network::ws::connect(url.clone()).await {
                            Ok(ok) => ok,
                            Err(error) => {
                                eprintln!("NETWORK-LOG: Failed to connect to server: {}", error);
                                tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                continue;
                            }
                        };
                    println!("NETWORK-LOG: Server Connected");

                    // 밀린 요청 보내고
                    for request in requests.values() {
                        sender.send(&request.bytes);
                    }

                    loop {
                        tokio::select! {
                            request = request_rx.recv() => {
                                match request {
                                    Some(request) => {
                                        sender.send(&request.bytes);
                                        requests.insert(request.packet_id, request);
                                    },
                                    None => {
                                        // Connection Keeper Dropped
                                        return;
                                    },
                                }
                            },
                            response = receiver.recv() => {
                                match response {
                                    Some(response) => {
                                        let packet_id = u32::from_le_bytes(response[response.len() - 4..].try_into().unwrap());
                                        println!("packet_id: {packet_id}");
                                        let request = requests.remove(&packet_id).unwrap();
                                        request.response_tx.send(response.into_vec()).unwrap();
                                    },
                                    None => {
                                        // Disconnected
                                        println!("Server Connection Closed");
                                        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                                        continue;
                                    },
                                }
                            },
                        }
                    }
                }
            }
        });
        Self {
            abort_handle: join_handle.abort_handle(),
            request_tx,
        }
    }
    async fn request(&self, packet_id: u32, request_packet_bytes: Vec<u8>) -> Vec<u8> {
        let (response_tx, response_rx) = oneshot::channel();
        self.request_tx
            .send(Request {
                packet_id,
                bytes: request_packet_bytes,
                response_tx,
            })
            .unwrap();
        response_rx.await.unwrap()
    }
}
impl Drop for ConnectionKeeper {
    fn drop(&mut self) {
        self.abort_handle.abort();
    }
}

#[derive(Clone)]
pub struct ServerConnection {
    connection_keeper: Arc<ConnectionKeeper>,
}

impl ServerConnection {
    pub fn new(url: impl ToString) -> Self {
        Self {
            connection_keeper: ConnectionKeeper::new(url).into(),
        }
    }

    pub async fn request<
        Response: rkyv::Archive + Send + 'static + Debug,
        Error: rkyv::Archive + Send + 'static + Debug,
    >(
        &self,
        api_index: u16,
        request_bytes: Vec<u8>,
    ) -> Result<Response, Error>
    where
        <Response as luda_rpc::rkyv::Archive>::Archived: luda_rpc::rkyv::Deserialize<
            Response,
            luda_rpc::rkyv::de::deserializers::SharedDeserializeMap,
        >,
        <Error as luda_rpc::rkyv::Archive>::Archived: luda_rpc::rkyv::Deserialize<
            Error,
            luda_rpc::rkyv::de::deserializers::SharedDeserializeMap,
        >,
    {
        println!("NETWORK-LOG: request: {:?}", api_index);
        let request_packet = RequestPacket::new(api_index, request_bytes);

        let response_packet = self.request_raw(request_packet).await;

        let response = match response_packet.status {
            ResponseStatus::Response => {
                let response = unsafe {
                    rkyv::from_bytes_unchecked(&response_packet.response_payload).unwrap()
                };
                Ok(response)
            }
            ResponseStatus::Error => {
                let error = unsafe {
                    rkyv::from_bytes_unchecked(&response_packet.response_payload).unwrap()
                };
                Err(error)
            }
        };
        println!("NETWORK-LOG: response: {:?}", response);
        response
    }

    async fn request_raw(&self, request_packet: RequestPacket) -> ResponsePacket {
        let packet_id = request_packet.packet_id;
        let request_packet_bytes = request_packet.into_bytes();

        let response_packet_bytes = self
            .connection_keeper
            .request(packet_id, request_packet_bytes)
            .await;
        assert!(response_packet_bytes.len() >= 5);

        let (response_payload, header) = {
            let mut response_packet_bytes = response_packet_bytes;
            let header = response_packet_bytes.split_off(response_packet_bytes.len() - 5);
            (response_packet_bytes, header)
        };

        let status = match header[0] {
            0 => ResponseStatus::Response,
            1 => ResponseStatus::Error,
            _ => unreachable!("Invalid status: {}", header[0]),
        };

        ResponsePacket {
            status,
            response_payload,
        }
    }
}

struct RequestPacket {
    packet_id: u32,
    api_index: u16,
    in_payload: Vec<u8>,
}

#[repr(u8)]
enum ResponseStatus {
    Response = 0,
    Error = 1,
}

struct ResponsePacket {
    status: ResponseStatus,
    response_payload: Vec<u8>,
}

impl RequestPacket {
    fn new(api_index: u16, in_payload: Vec<u8>) -> Self {
        let packet_id = {
            static PACKET_ID: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
            PACKET_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        };
        Self {
            packet_id,
            api_index,
            in_payload,
        }
    }
    fn into_bytes(self) -> Vec<u8> {
        let mut bytes = self.in_payload;
        bytes.extend_from_slice(&self.packet_id.to_le_bytes());
        bytes.extend_from_slice(&self.api_index.to_le_bytes());
        bytes
    }
}
