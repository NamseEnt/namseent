use std::{
    borrow::Borrow,
    fmt::Debug,
    ops::Deref,
    pin::Pin,
    sync::{atomic::AtomicU64, Arc},
};
// use futures::*;
// use futures_util::{
//     stream::{SplitSink, SplitStream, StreamExt},
//     FutureExt, SinkExt, TryFutureExt,
// };
// use luda_editor_rpc::{
//     async_trait, recall_layer::RecallLayerReceiver, DirectoryEntry, RpcHandle, Socket,
//     TransportLayer,
// };
use bincode::deserialize;
use dashmap::DashMap;
use futures::{
    sink::WithFlatMap,
    stream::{self, SplitSink, SplitStream, StreamExt},
    Sink, SinkExt, Stream,
};
use luda_editor_rpc::async_trait;
use luda_editor_rpc2::ls;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tokio::sync::Notify;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

#[tokio::main]
async fn main() {
    let routes = warp::path("")
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(move |web_socket| on_connected(web_socket))
        });

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn on_connected(web_socket: WebSocket) {
    let response_waiter = ResponseWaiter::new();
    let (sink, stream) = web_socket.split();
    // let sink = sink.with_flat_map(|packet: Vec<u8>| stream::iter(vec![Message::binary(packet)]));
    let sink =
        sink.with_flat_map(|packet: Vec<u8>| stream::iter(vec![Ok(Message::binary(packet))]));

    let a = sink.clone();

    let mut socket = Socket::new(Arc::new(sink), response_waiter.clone());

    let result = socket
        .ls(ls::Request {
            path: "abc".to_string(),
        })
        .await;
    let socket2 = socket.clone();

    // while let Some(result) = rx
    //     .next()
    //     .await
    // {
    //     let message = match result {
    //         Ok(msg) => msg,
    //         Err(e) => {
    //             eprintln!("websocket error: {}", e);
    //             break;
    //         }
    //     };
    //     if !message.is_binary() {
    //         continue;
    //     }

    //     let packet = message.into_bytes();
    // }
}

#[derive(Clone)]
pub struct ResponseWaiter {
    notification_map: Arc<DashMap<u64, Arc<Notify>>>,
    response_data_map: Arc<DashMap<u64, Vec<u8>>>,
}
impl ResponseWaiter {
    pub fn new() -> Self {
        Self {
            notification_map: Arc::new(DashMap::new()),
            response_data_map: Arc::new(DashMap::new()),
        }
    }
}

impl ResponseWaiter {
    fn ready_to_wait(&self, id: u64) -> Arc<Notify> {
        let notify = Arc::new(Notify::new());
        self.notification_map
            .insert(id, notify.clone());
        notify
    }
    async fn wait(&self, id: u64, notify: Arc<Notify>) -> Result<Vec<u8>, String> {
        notify
            .notified()
            .await;

        let (_, data) = self
            .response_data_map
            .remove(&id)
            .ok_or_else(|| format!("no response for id {}", id))?;

        Ok(data)
    }
    fn on_response(&self, id: u64, response: Vec<u8>) {
        self.response_data_map
            .insert(id, response);

        let (_, notify) = self
            .notification_map
            .remove(&id)
            .unwrap();

        notify.notify_waiters();
    }
}
pub struct Socket<TError> {
    sink: Box<dyn Sink<Vec<u8>, Error = TError> + std::marker::Unpin>,
    response_waiter: ResponseWaiter,
    id: AtomicU64,
}
impl Clone for Socket<()> {
    fn clone(&self) -> Self {
        Self {
            sink: self
                .sink
                .clone(),
            response_waiter: self
                .response_waiter
                .clone(),
            id: self
                .id
                .clone(),
        }
    }
}
pub mod luda_editor_rpc2 {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct DirectoryEntry {}

    pub mod ls {
        use super::DirectoryEntry;
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize)]
        pub struct Request {
            pub path: String,
        }
        #[derive(Serialize, Deserialize)]
        pub struct Response {
            pub entries: Vec<DirectoryEntry>,
        }
    }
}

impl<TError> Socket<TError>
where
    TError: Debug,
{
    pub fn new(
        sink: Box<dyn Sink<Vec<u8>, Error = TError> + std::marker::Unpin>,
        response_waiter: ResponseWaiter,
    ) -> Self {
        Socket {
            sink,
            response_waiter,
            id: AtomicU64::new(0),
        }
    }
    async fn send<'de, TRequest, TResponse>(
        &mut self,
        request: TRequest,
        api: RpcApi,
    ) -> Result<TResponse, String>
    where
        TRequest: Serialize,
        TResponse: DeserializeOwned,
    {
        let id = self
            .id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let request_packet = RpcRequestPacket {
            id,
            api,
            data: bincode::serialize(&request).unwrap(),
        };

        let notify = self
            .response_waiter
            .ready_to_wait(id);

        Pin::new(&mut self.sink)
            .send(bincode::serialize(&request_packet).unwrap())
            .await
            .unwrap();

        let response_data = self
            .response_waiter
            .wait(id, notify)
            .await?;

        let response: TResponse = bincode::deserialize(&response_data).unwrap();

        Ok(response)
    }
    pub async fn ls(
        &mut self,
        request: luda_editor_rpc2::ls::Request,
    ) -> Result<luda_editor_rpc2::ls::Response, String> {
        self.send(request, RpcApi::ls)
            .await
    }
}

// impl Clone for Socket {}

#[async_trait]
pub trait RpcHandle {
    async fn ls(
        &mut self,
        request: luda_editor_rpc2::ls::Request,
    ) -> luda_editor_rpc2::ls::Response;
}

pub async fn loop_receiving<TRpcHandle, TCloneSink>(
    // sink: &mut (dyn Sink<Vec<u8>, Error = ()> + Clone),
    sink: &mut TCloneSink,
    stream: Pin<&mut dyn Stream<Item = Result<&[u8], String>>>,
    handler: &mut TRpcHandle,
    response_waiter: ResponseWaiter,
) where
    TRpcHandle: RpcHandle + Sized + Clone + std::marker::Unpin,
    TCloneSink: Sink<Vec<u8>, Error = ()> + Clone + std::marker::Unpin,
{
    stream
        .for_each_concurrent(None, |packet_result| {
            let mut sink = Box::new(sink.clone());
            let mut handler = Box::new(handler.clone());
            let response_waiter = Box::new(response_waiter.clone());
            async move {
                if let Err(e) = packet_result {
                    eprintln!("websocket error: {}", e);
                    return;
                }
                let packet_buffer = packet_result.unwrap();
                let deserialize_result = bincode::deserialize::<RpcPacket>(packet_buffer);
                match deserialize_result {
                    Ok(packet) => match packet {
                        RpcPacket::Request(request_packet) => match request_packet.api {
                            RpcApi::ls => {
                                let request =
                                    bincode::deserialize::<luda_editor_rpc2::ls::Request>(
                                        &request_packet.data,
                                    )
                                    .unwrap();
                                let response = handler
                                    .ls(request)
                                    .await;
                                let response_packet = RpcPacket::Response(RpcResponsePacket {
                                    id: request_packet.id,
                                    data: bincode::serialize(&response).unwrap(),
                                });
                                let response_packet_buffer =
                                    bincode::serialize(&response_packet).unwrap();
                                sink.send(response_packet_buffer)
                                    .await
                                    .unwrap();
                            }
                        },
                        RpcPacket::Response(response) => {
                            response_waiter.on_response(response.id, response.data);
                        }
                    },
                    Err(error) => {
                        eprintln!("{}", error);
                    }
                }
            }
        })
        .await;
}
#[derive(Serialize, Deserialize)]
enum RpcPacket {
    Request(RpcRequestPacket),
    Response(RpcResponsePacket),
}

#[derive(Serialize, Deserialize)]
enum RpcApi {
    ls,
}

#[derive(Serialize, Deserialize)]
struct RpcRequestPacket {
    id: u64,
    api: RpcApi,
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
struct RpcResponsePacket {
    id: u64,
    data: Vec<u8>,
}
