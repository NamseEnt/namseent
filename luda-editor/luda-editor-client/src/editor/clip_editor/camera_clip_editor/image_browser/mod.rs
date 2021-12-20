use crate::editor::types::*;
use async_trait::async_trait;
use dashmap::DashMap;
use futures::{stream, SinkExt, Stream, StreamExt};
use namui::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};
use tokio::sync::mpsc::{self, unbounded_channel, UnboundedSender};
use tokio::sync::Notify;
use tokio_stream::wrappers::UnboundedReceiverStream;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::{ErrorEvent, MessageEvent};

struct ImageFilenameObject {
    character: String,
    pose: String,
    emotion: String,
    extension: String,
}

pub struct ImageBrowser {
    directory_key: String,
    selected_key: Option<String>,
    image_filename_objects: Vec<ImageFilenameObject>,
    scroll_y: f32,
}

// struct WasmBindgenSocket {
//     url: String,
// }
// impl WasmBindgenSocket {
//     pub fn new(url: String) -> Self {
//         Self { url }
//     }
// }

impl ImageBrowser {
    pub fn new() -> Self {
        let response_waiter = ResponseWaiter::new();
        let (sending_sender, mut sending_receiver) = unbounded_channel();
        let mut socket = Socket::new(sending_sender.clone(), response_waiter.clone());
        let web_socket = web_sys::WebSocket::new("ws://localhost:3030").unwrap();
        web_socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let (receiving_sender, receiving_receiver) = unbounded_channel();
        let receiving_stream = UnboundedReceiverStream::new(receiving_receiver);
        let handler = RpcHandler {};
        spawn_local(loop_receiving(
            sending_sender.clone(),
            receiving_stream,
            handler,
            response_waiter.clone(),
        ));

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Handle difference Text/Binary,...
            if let Ok(array_buffer) = e
                .data()
                .dyn_into::<js_sys::ArrayBuffer>()
            {
                namui::log(format!("message event, received arraybuffer: {:?}", array_buffer));
                let u8_array = js_sys::Uint8Array::new(&array_buffer);
                let len = u8_array.byte_length() as usize;
                let packet = u8_array.to_vec();
                namui::log(format!("Arraybuffer received {}bytes: {:?}", len, packet));
                receiving_sender
                    .send(Ok(packet))
                    .unwrap();
            } else {
                namui::log(format!("message event, received Unknown: {:?}", e.data()));
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        web_socket.set_onmessage(Some(
            onmessage_callback
                .as_ref()
                .unchecked_ref(),
        ));
        onmessage_callback.forget();

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            namui::log(format!("error event: {:?}", e));
        }) as Box<dyn FnMut(ErrorEvent)>);
        web_socket.set_onerror(Some(
            onerror_callback
                .as_ref()
                .unchecked_ref(),
        ));
        onerror_callback.forget();

        namui::log(format!("socket created"));

        let cloned_web_socket = web_socket.clone();
        let onopen_callback = Closure::once(move || {
            namui::log(format!("socket opened"));
            spawn_local(async move {
                while let Some(packet) = sending_receiver
                    .recv()
                    .await
                {
                    namui::log(format!("sending packet: {:?}", packet));
                    cloned_web_socket
                        .send_with_u8_array(&packet)
                        .unwrap();
                }
            });
        });

        spawn_local(async move {
            let result = socket
                .ls(luda_editor_rpc2::ls::Request {
                    path: "/".to_string(),
                })
                .await;
            namui::log(format!("ls result: {:?}", result));
        });

        web_socket.set_onopen(Some(
            onopen_callback
                .as_ref()
                .unchecked_ref(),
        ));
        onopen_callback.forget();

        Self {
            directory_key: "".to_string(),
            selected_key: None,
            image_filename_objects: vec![],
            scroll_y: 0.0,
        }
    }
    // 처음 만들어지면 로딩을 시작하고
    // 그 로딩 결과를 가지고 이미지 브라우저의 image_filename_objects를 채워야 한다.
    // 어떻게 할 것인가?
}

pub struct ImageBrowserProps {}

impl Entity for ImageBrowser {
    type Props = ImageBrowserProps;

    fn update(&mut self, event: &dyn std::any::Any) {}

    fn render(&self, props: &Self::Props) -> RenderingTree {
        RenderingTree::Empty
    }
}

//

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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DirectoryEntry {}

    pub mod ls {
        use super::DirectoryEntry;
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize)]
        pub struct Request {
            pub path: String,
        }
        #[derive(Debug, Serialize, Deserialize)]
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

        let request_packet = RpcPacket::Request(RpcRequestPacket {
            id,
            api,
            data: bincode::serialize(&request).unwrap(),
        });

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
            let mut sender = Box::new(sender.clone());
            let response_waiter = Box::new(response_waiter.clone());
            async move {
                if let Err(e) = packet_result {
                    eprintln!("websocket error: {}", e);
                    return;
                }
                let packet_buffer = packet_result.unwrap();
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
#[derive(Debug, Serialize, Deserialize)]
enum RpcPacket {
    Request(RpcRequestPacket),
    Response(RpcResponsePacket),
}

#[derive(Debug, Serialize, Deserialize)]
enum RpcApi {
    ls,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcRequestPacket {
    id: u64,
    api: RpcApi,
    data: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RpcResponsePacket {
    id: u64,
    data: Vec<u8>,
}
