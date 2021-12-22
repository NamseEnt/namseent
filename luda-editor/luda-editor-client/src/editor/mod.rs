mod main;
pub use main::main;
mod timeline;
use namui::prelude::*;
pub use timeline::*;
mod types;
use crate::editor::clip_editor::ClipEditorProps;

use self::{
    clip_editor::ClipEditor,
    events::*,
    job::{Job, MoveCameraClipJob},
};
use types::*;
mod clip_editor;
mod events;
mod job;

struct Editor {
    timeline: Timeline,
    clip_editor: ClipEditor,
    playback_time: chrono::Duration,
    socket: luda_editor_rpc::Socket,
}

impl namui::Entity for Editor {
    type Props = ();
    fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::CameraClipBodyMouseDownEvent {
                    clip_id,
                    global_mouse_xy,
                    ..
                } => {
                    if self
                        .timeline
                        .job
                        .is_none()
                    {
                        self.timeline
                            .job = Some(Job::MoveCameraClip(MoveCameraClipJob {
                            clip_id: clip_id.clone(),
                            click_anchor_in_global: *global_mouse_xy,
                            last_global_mouse_xy: *global_mouse_xy,
                        }));
                    }
                    self.timeline
                        .selected_clip_id = Some(clip_id.clone());
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(global_xy) => match self
                    .timeline
                    .job
                {
                    Some(Job::MoveCameraClip(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    _ => {}
                },
                NamuiEvent::MouseUp(global_xy) => {
                    let job = self
                        .timeline
                        .job
                        .clone();
                    match job {
                        Some(Job::MoveCameraClip(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.timeline
                                .job = None;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };
        self.clip_editor
            .update(event);
    }
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let selected_clip = self
            .timeline
            .selected_clip_id
            .as_ref()
            .and_then(|id| {
                self.timeline
                    .sequence
                    .get_clip(&id)
            });
        render![
            self.timeline
                .render(&TimelineProps {
                    playback_time: self.playback_time,
                }),
            self.clip_editor
                .render(&ClipEditorProps {
                    selected_clip
                }),
        ]
    }
}

impl Editor {
    fn new(screen_wh: namui::Wh<f32>) -> Self {
        let socket = Editor::create_socket();
        Self {
            timeline: Timeline::new(
                Editor::calculate_timeline_xywh(screen_wh),
                get_sample_sequence(),
            ),
            clip_editor: ClipEditor::new(&socket),
            playback_time: chrono::Duration::zero(),
            socket,
        }
    }
    fn resize(&mut self, wh: namui::Wh<f32>) {
        self.timeline
            .resize(Editor::calculate_timeline_xywh(wh));
    }
    fn calculate_timeline_xywh(wh: namui::Wh<f32>) -> XywhRect<f32> {
        XywhRect {
            x: 0.0,
            y: wh.height - 200.0,
            width: wh.width,
            height: 200.0,
        }
    }
    fn create_socket() -> luda_editor_rpc::Socket {
        use luda_editor_rpc::{async_trait::async_trait, response_waiter::*, RpcHandle, *};
        use tokio::sync::mpsc::unbounded_channel;
        use tokio_stream::wrappers::UnboundedReceiverStream;
        use wasm_bindgen::{closure::Closure, JsCast};
        use wasm_bindgen_futures::spawn_local;
        use web_sys::{ErrorEvent, MessageEvent};

        #[derive(Clone)]
        pub struct RpcHandler {}

        #[async_trait]
        impl RpcHandle for RpcHandler {
            async fn get_camera_shot_urls(
                &mut self,
                request: luda_editor_rpc::get_camera_shot_urls::Request,
            ) -> Result<luda_editor_rpc::get_camera_shot_urls::Response, String> {
                todo!()
            }
        }

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

        web_socket.set_onopen(Some(
            onopen_callback
                .as_ref()
                .unchecked_ref(),
        ));
        onopen_callback.forget();

        socket
    }
}
