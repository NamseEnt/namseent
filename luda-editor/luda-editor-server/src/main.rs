use bincode::deserialize;
use dashmap::DashMap;
use futures::{
    join,
    sink::WithFlatMap,
    stream::{self, SplitSink, SplitStream, StreamExt},
    Sink, SinkExt, Stream,
};
use luda_editor_rpc::async_trait;
use luda_editor_rpc2::ls;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::sync::mpsc::{self, unbounded_channel, UnboundedSender};
use tokio::sync::Notify;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};
#[tokio::main]
async fn main() {
    let routes = warp::path::end()
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
    let mut sink =
        sink.with_flat_map(|packet: Vec<u8>| stream::iter(vec![Ok(Message::binary(packet))]));

    let (tx, mut rx) = unbounded_channel();
    let tx2 = tx.clone();

    // let mut socket = Socket::new(tx, response_waiter.clone());
    // let socket2 = socket.clone();

    let handler = RpcHandler {};
    let stream = stream.map(|message| {
        message
            .map(|message| {
                message
                    .as_bytes()
                    .to_vec()
            })
            .map_err(|e| format!("websocket error: {}", e))
    });

    let loop_sending = async {
        while let Some(data) = rx
            .recv()
            .await
        {
            println!("sending: {:?}", data);
            sink.send(data)
                .await
                .unwrap();
        }
    };

    join!(loop_sending, loop_receiving(tx2, stream, handler, response_waiter));
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
pub struct Socket {
    sender: UnboundedSender<Vec<u8>>,
    response_waiter: ResponseWaiter,
}
impl Clone for Socket {
    fn clone(&self) -> Self {
        Self {
            sender: self
                .sender
                .clone(),
            response_waiter: self
                .response_waiter
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

impl Socket {
    pub fn new(sender: UnboundedSender<Vec<u8>>, response_waiter: ResponseWaiter) -> Self {
        Socket {
            sender,
            response_waiter,
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
        static mut ID_COUNTER: AtomicU64 = AtomicU64::new(0);
        let id = unsafe { ID_COUNTER.fetch_add(1, Ordering::SeqCst) };

        let request_packet = RpcRequestPacket {
            id,
            api,
            data: bincode::serialize(&request).unwrap(),
        };

        let notify = self
            .response_waiter
            .ready_to_wait(id);

        let data = bincode::serialize(&request_packet).unwrap();
        self.sender
            .send(data)
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

#[async_trait]
pub trait RpcHandle {
    async fn ls(
        &mut self,
        request: luda_editor_rpc2::ls::Request,
    ) -> luda_editor_rpc2::ls::Response;
}

#[derive(Clone)]
pub struct RpcHandler {}

#[async_trait]
impl RpcHandle for RpcHandler {
    async fn ls(
        &mut self,
        request: luda_editor_rpc2::ls::Request,
    ) -> luda_editor_rpc2::ls::Response {
        println!("ls: {}", request.path);
        luda_editor_rpc2::ls::Response {
            entries: vec![],
        }
    }
}

pub async fn loop_receiving<'a, TRpcHandle, TStream>(
    sender: UnboundedSender<Vec<u8>>,
    stream: TStream,
    handler: TRpcHandle,
    response_waiter: ResponseWaiter,
) where
    TRpcHandle: RpcHandle + Sized + Clone + std::marker::Unpin,
    TStream: Stream<Item = Result<Vec<u8>, String>> + Sized + std::marker::Unpin,
{
    stream
        .for_each_concurrent(None, |packet_result| {
            let mut handler = Box::new(handler.clone());
            let sender = Box::new(sender.clone());
            let response_waiter = Box::new(response_waiter.clone());
            async move {
                if let Err(e) = packet_result {
                    eprintln!("websocket error: {}", e);
                    return;
                }
                let packet_buffer = packet_result.unwrap();
                println!("receiving: {:?}", packet_buffer);
                let deserialize_result = bincode::deserialize::<RpcPacket>(&packet_buffer);
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
                                sender
                                    .send(response_packet_buffer)
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
