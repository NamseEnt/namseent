use crate::editor::{events::EditorEvent, types::*};
use namui::prelude::*;
use wasm_bindgen_futures::spawn_local;

pub struct ImageBrowser {
    directory_key: String,
    selected_key: Option<String>,
    image_filename_objects: Vec<ImageFilenameObject>,
    scroll_y: f32,
}

impl ImageBrowser {
    pub fn new(socket: &luda_editor_rpc::Socket) -> Self {
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

impl ImageBrowser {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::ImageFilenameObjectsUpdatedEvent {
                    image_filename_objects,
                } => {
                    self.image_filename_objects = image_filename_objects.to_vec();
                }
                _ => {}
            }
        };
    }

    pub fn render(&self, props: &ImageBrowserProps) -> RenderingTree {
        // namui::log(format!("rendering image browser {:?}", self.image_filename_objects));
        RenderingTree::Empty
    }
}

impl ImageFilenameObject {
    pub fn new(camera_shot_url: &String) -> Self {
        let file_name_with_extension = camera_shot_url
            .split("/")
            .last()
            .unwrap();
        // remove only extension but keep dot in middle of name.
        let last_dot_index = file_name_with_extension
            .rfind('.')
            .unwrap();
        let file_name = file_name_with_extension
            .split_at(last_dot_index)
            .0;

        let mut splits = file_name.split("-");

        let character = splits
            .next()
            .unwrap();
        let emotion = splits
            .next()
            .unwrap();
        let pose = splits
            .collect::<Vec<&str>>()
            .join("-");

        Self {
            character: character.to_string(),
            emotion: emotion.to_string(),
            pose,
            url: camera_shot_url.to_string(),
        }
    }
}
