use super::{
    router::RouterProps,
    types::{
        meta::{self, *},
        AppContext,
    },
    Router,
};
use async_trait::async_trait;
use luda_editor_rpc::Socket;
use namui::Wh;
use std::sync::Arc;
use wasm_bindgen_futures::spawn_local;

pub struct App {
    router: Router,
    meta_container: Arc<MetaContainer>,
}

impl namui::Entity for App {
    type Props = ();
    fn render(&self, _: &Self::Props) -> namui::RenderingTree {
        match self.meta_container.get_meta() {
            Some(meta) => {
                let screen_size = namui::system::screen::size();
                self.router.render(&RouterProps {
                    screen_wh: Wh {
                        width: screen_size.width as f32,
                        height: screen_size.height as f32,
                    },
                    meta: &meta,
                })
            }
            None => namui::RenderingTree::Empty,
        }
    }
    fn update(&mut self, event: &dyn std::any::Any) {
        self.meta_container.update(event);
        self.router.update(event);
    }
}

impl App {
    pub fn new() -> Self {
        let socket = App::create_socket();
        let meta_container = Arc::new(MetaContainer::new(None, Arc::new(socket.clone())));
        meta_container.start_reloading();
        Self {
            router: Router::new(AppContext {
                socket,
                meta_container: meta_container.clone(),
            }),
            meta_container: meta_container.clone(),
        }
    }
    fn create_socket() -> luda_editor_rpc::Socket {
        use luda_editor_rpc::{response_waiter::*, RpcHandle, *};
        use tokio::sync::mpsc::unbounded_channel;
        use tokio_stream::wrappers::UnboundedReceiverStream;
        use wasm_bindgen::{closure::Closure, JsCast};
        use web_sys::{ErrorEvent, MessageEvent};

        #[derive(Clone)]
        pub struct RpcHandler {}

        #[async_trait]
        impl RpcHandle for RpcHandler {
            async fn get_character_image_urls(
                &mut self,
                _: luda_editor_rpc::get_character_image_urls::Request,
            ) -> Result<luda_editor_rpc::get_character_image_urls::Response, String> {
                unreachable!()
            }
            async fn get_background_image_urls(
                &mut self,
                _: luda_editor_rpc::get_background_image_urls::Request,
            ) -> Result<luda_editor_rpc::get_background_image_urls::Response, String> {
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
            async fn get_sequences(
                &mut self,
                _: luda_editor_rpc::get_sequences::Request,
            ) -> Result<luda_editor_rpc::get_sequences::Response, String> {
                unreachable!()
            }
            async fn put_sequences(
                &mut self,
                _: luda_editor_rpc::put_sequences::Request,
            ) -> Result<luda_editor_rpc::put_sequences::Response, String> {
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
                let u8_array = js_sys::Uint8Array::new(&array_buffer);
                let packet = u8_array.to_vec();
                receiving_sender.send(Ok(packet)).unwrap();
            } else {
                namui::log!("message event, received Unknown: {:?}", e.data());
            }
        }) as Box<dyn FnMut(MessageEvent)>);

        web_socket.set_onmessage(Some(onmessage_callback.as_ref().unchecked_ref()));
        onmessage_callback.forget();

        let onerror_callback = Closure::wrap(Box::new(move |e: ErrorEvent| {
            namui::log!("error event: {:?}", e);
        }) as Box<dyn FnMut(ErrorEvent)>);
        web_socket.set_onerror(Some(onerror_callback.as_ref().unchecked_ref()));
        onerror_callback.forget();

        namui::log!("socket created");

        let cloned_web_socket = web_socket.clone();
        let onopen_callback = Closure::once(move || {
            namui::log!("socket opened");
            spawn_local(async move {
                while let Some(packet) = sending_receiver.recv().await {
                    cloned_web_socket.send_with_u8_array(&packet).unwrap();
                }
            });
        });

        web_socket.set_onopen(Some(onopen_callback.as_ref().unchecked_ref()));
        onopen_callback.forget();

        socket
    }
}

#[async_trait]
impl MetaLoad for Socket {
    async fn load_meta(&self) -> Result<Meta, String> {
        meta::get_meta(&self).await
    }
}
