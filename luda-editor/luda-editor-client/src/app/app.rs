use super::{router::RouterProps, types::AppContext, Router};
use namui::Wh;
use wasm_bindgen_futures::spawn_local;

pub struct App {
    router: Router,
}

impl namui::Entity for App {
    type Props = ();
    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        let screen_size = namui::screen::size();
        self.router.render(&RouterProps {
            screen_wh: Wh {
                width: screen_size.width as f32,
                height: screen_size.height as f32,
            },
        })
    }
    fn update(&mut self, event: &dyn std::any::Any) {
        self.router.update(event);
    }
}

impl App {
    pub fn new() -> Self {
        let socket = App::create_socket();
        let screen_size = namui::screen::size();
        Self {
            router: Router::new(AppContext {
                screen_size: namui::Wh {
                    width: screen_size.width as f32,
                    height: screen_size.height as f32,
                },
                socket,
            }),
        }
    }
    fn create_socket() -> luda_editor_rpc::Socket {
        use luda_editor_rpc::{async_trait::async_trait, response_waiter::*, RpcHandle, *};
        use tokio::sync::mpsc::unbounded_channel;
        use tokio_stream::wrappers::UnboundedReceiverStream;
        use wasm_bindgen::{closure::Closure, JsCast};
        use web_sys::{ErrorEvent, MessageEvent};

        #[derive(Clone)]
        pub struct RpcHandler {}

        #[async_trait]
        impl RpcHandle for RpcHandler {
            async fn get_camera_shot_urls(
                &mut self,
                _: luda_editor_rpc::get_camera_shot_urls::Request,
            ) -> Result<luda_editor_rpc::get_camera_shot_urls::Response, String> {
                unreachable!()
            }
            async fn read_file(
                &mut self,
                _: luda_editor_rpc::read_file::Request,
            ) -> Result<luda_editor_rpc::read_file::Response, String> {
                unreachable!()
            }
            async fn read_dir(
                &mut self,
                _: luda_editor_rpc::read_dir::Request,
            ) -> Result<luda_editor_rpc::read_dir::Response, String> {
                unreachable!()
            }
            async fn write_file(
                &mut self,
                _: luda_editor_rpc::write_file::Request,
            ) -> Result<luda_editor_rpc::write_file::Response, String> {
                unreachable!()
            }
        }

        let response_waiter = ResponseWaiter::new();
        let (sending_sender, mut sending_receiver) = unbounded_channel();
        let socket = Socket::new(sending_sender.clone(), response_waiter.clone());
        let web_socket = web_sys::WebSocket::new("ws://localhost:3030").unwrap();
        web_socket.set_binary_type(web_sys::BinaryType::Arraybuffer);

        let (receiving_sender, receiving_receiver) = unbounded_channel();
        let receiving_stream = UnboundedReceiverStream::new(receiving_receiver);
        let handler = RpcHandler {};
        spawn_local(async move {
            let _ = loop_receiving(
                sending_sender.clone(),
                receiving_stream,
                handler,
                response_waiter.clone(),
            )
            .await;
        });

        let onmessage_callback = Closure::wrap(Box::new(move |e: MessageEvent| {
            // Handle difference Text/Binary,...
            if let Ok(array_buffer) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                namui::log(format!(
                    "message event, received arraybuffer: {:?}",
                    array_buffer
                ));
                let u8_array = js_sys::Uint8Array::new(&array_buffer);
                let len = u8_array.byte_length() as usize;
                let packet = u8_array.to_vec();
                namui::log(format!("Arraybuffer received {}bytes: {:?}", len, packet));
                receiving_sender.send(Ok(packet)).unwrap();
            } else {
                namui::log(format!("message event, received Unknown: {:?}", e.data()));
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        web_socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            namui::log(format!("error event: {:?}", e));
        }) as Box<dyn FnMut(ErrorEvent)>);
        web_socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        namui::log(format!("socket created"));

        let cloned_web_socket = web_socket.clone();
        let onopen_callback = Closure::once(move || {
            namui::log(format!("socket opened"));
            spawn_local(async move {
                while let Some(packet) = sending_receiver.recv().await {
                    namui::log(format!("sending packet: {:?}", packet));
                    cloned_web_socket.send_with_u8_array(&packet).unwrap();
                }
            });
        });

        web_socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        socket
    }
}
