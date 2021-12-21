use crate::editor::types::*;
use async_trait::async_trait;
use luda_editor_rpc::{loop_receiving, response_waiter::ResponseWaiter, RpcHandle, Socket};
use namui::prelude::*;
use tokio::sync::mpsc::unbounded_channel;
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
        spawn_local(async move {
            loop_receiving(
                sending_sender.clone(),
                receiving_stream,
                handler,
                response_waiter.clone(),
            )
            .await;
        });

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
                .ls(luda_editor_rpc::ls::Request {
                    path: "/".to_string(),
                })
                .await;
            namui::log(format!("ls result: {:?}", result.is_ok()));
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

#[derive(Clone)]
pub struct RpcHandler {}

#[async_trait]
impl RpcHandle for RpcHandler {
    async fn ls(
        &mut self,
        request: luda_editor_rpc::ls::Request,
    ) -> Result<luda_editor_rpc::ls::Response, String> {
        println!("ls: {}", request.path);
        Ok(luda_editor_rpc::ls::Response {
            directory_entries: vec![],
        })
    }
}
