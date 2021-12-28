mod main;
pub use main::main;
mod timeline;
use namui::prelude::*;
pub use timeline::*;
use wasm_bindgen_futures::spawn_local;
mod types;
use crate::editor::clip_editor::ClipEditorProps;

use self::{
    clip_editor::ClipEditor,
    events::*,
    job::{Job, MoveCameraClipJob, WysiwygMoveImageJob, WysiwygResizeImageJob},
};
use types::*;
mod clip_editor;
mod events;
mod job;

struct Editor {
    job: Option<Job>,
    timeline: Timeline,
    clip_editor: ClipEditor,
    playback_time: chrono::Duration,
    socket: luda_editor_rpc::Socket,
    screen_wh: namui::Wh<f32>,
    image_filename_objects: Vec<ImageFilenameObject>,
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
                    if self.job.is_none() {
                        self.job = Some(Job::MoveCameraClip(MoveCameraClipJob {
                            clip_id: clip_id.clone(),
                            click_anchor_in_global: *global_mouse_xy,
                            last_global_mouse_xy: *global_mouse_xy,
                        }));
                    }
                    self.timeline.selected_clip_id = Some(clip_id.clone());
                }
                EditorEvent::ImageFilenameObjectsUpdatedEvent {
                    image_filename_objects,
                } => {
                    self.image_filename_objects = image_filename_objects.to_vec();
                }
                EditorEvent::WysiwygEditorInnerImageMouseDownEvent {
                    mouse_xy,
                    container_size,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygMoveImage(WysiwygMoveImageJob {
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            container_size: *container_size,
                        }));
                    };
                }
                EditorEvent::WysiwygEditorResizerHandleMouseDownEvent {
                    mouse_xy,
                    handle,
                    center_xy,
                    container_size,
                    image_size_ratio,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::WysiwygResizeImage(WysiwygResizeImageJob {
                            start_global_mouse_xy: *mouse_xy,
                            last_global_mouse_xy: *mouse_xy,
                            handle: *handle,
                            center_xy: *center_xy,
                            container_size: *container_size,
                            image_size_ratio: *image_size_ratio,
                        }));
                    };
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match event {
                NamuiEvent::MouseMove(global_xy) => match self.job {
                    Some(Job::MoveCameraClip(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    Some(Job::WysiwygMoveImage(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    Some(Job::WysiwygResizeImage(ref mut job)) => {
                        job.last_global_mouse_xy = *global_xy;
                    }
                    _ => {}
                },
                NamuiEvent::MouseUp(global_xy) => {
                    let job = self.job.clone();
                    match job {
                        Some(Job::MoveCameraClip(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        Some(Job::WysiwygMoveImage(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        Some(Job::WysiwygResizeImage(mut job)) => {
                            job.last_global_mouse_xy = *global_xy;
                            job.execute(&mut self.timeline);
                            self.job = None;
                        }
                        _ => {}
                    }
                }
                &namui::NamuiEvent::ScreenResize(wh) => {
                    self.screen_wh = namui::Wh {
                        width: wh.width as f32,
                        height: wh.height as f32,
                    };
                }
                _ => {}
            }
        };
        self.clip_editor.update(event);
    }
    fn render(&self, props: &Self::Props) -> namui::RenderingTree {
        let selected_clip = self
            .timeline
            .selected_clip_id
            .as_ref()
            .and_then(|id| self.timeline.sequence.get_clip(&id));
        render![
            self.timeline.render(&TimelineProps {
                playback_time: self.playback_time,
                xywh: self.calculate_timeline_xywh(),
                job: &self.job,
            }),
            self.clip_editor.render(&ClipEditorProps {
                selected_clip,
                xywh: XywhRect {
                    x: 0.0,
                    y: 0.0,
                    width: 800.0,
                    height: self.screen_wh.height - 200.0,
                },
                image_filename_objects: &self.image_filename_objects,
                job: &self.job,
            }),
        ]
    }
}

impl Editor {
    fn new(screen_wh: namui::Wh<f32>) -> Self {
        let socket = Editor::create_socket();
        spawn_local({
            let socket = socket.clone();
            async move {
                let result = socket
                    .get_camera_shot_urls(luda_editor_rpc::get_camera_shot_urls::Request {})
                    .await;
                match result {
                    Ok(response) => {
                        let image_filename_objects = response
                            .camera_shot_urls
                            .iter()
                            .map(|url| ImageFilenameObject::new(url))
                            .collect();

                        namui::event::send(Box::new(
                            EditorEvent::ImageFilenameObjectsUpdatedEvent {
                                image_filename_objects,
                            },
                        ))
                    }
                    Err(error) => namui::log(format!("error on get_camera_shot_urls: {:?}", error)),
                }
            }
        });
        Self {
            timeline: Timeline::new(get_sample_sequence()),
            clip_editor: ClipEditor::new(),
            playback_time: chrono::Duration::zero(),
            socket,
            screen_wh,
            image_filename_objects: vec![],
            job: None,
        }
    }
    fn calculate_timeline_xywh(&self) -> XywhRect<f32> {
        XywhRect {
            x: 0.0,
            y: self.screen_wh.height - 200.0,
            width: self.screen_wh.width,
            height: 200.0,
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
                todo!()
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
