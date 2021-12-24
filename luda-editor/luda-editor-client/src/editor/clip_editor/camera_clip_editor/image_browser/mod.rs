use crate::editor::{events::EditorEvent, types::*};
use namui::prelude::*;
use std::{collections::BTreeSet, sync::Arc};
use wasm_bindgen_futures::spawn_local;
mod browser_item;
use browser_item::*;
mod back_button;
use back_button::*;
mod scroll;
use scroll::*;

#[derive(Debug)]
pub struct ImageBrowser {
    directory_key: String,
    selected_key: Option<String>,
    image_filename_objects: Vec<ImageFilenameObject>,
    current_directory_label_layout: XywhRect<f32>,
    scroll: Scroll,
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
            scroll: Scroll::new(),
            current_directory_label_layout: XywhRect {
                x: 20.0,
                y: 20.0,
                width: 160.0,
                height: 40.0,
            },
        }
    }
    // 처음 만들어지면 로딩을 시작하고
    // 그 로딩 결과를 가지고 이미지 브라우저의 image_filename_objects를 채워야 한다.
    // 어떻게 할 것인가?
}

pub struct ImageBrowserProps {
    pub width: f32,
    pub height: f32,
}

impl ImageBrowser {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::ImageFilenameObjectsUpdatedEvent {
                    image_filename_objects,
                } => {
                    self.image_filename_objects = image_filename_objects.to_vec();
                }
                EditorEvent::ImageBrowserSelectEvent { selected_key } => {
                    namui::log(format!("selected_key: {}", selected_key));
                    if selected_key == "back" {
                        let directory_splitted = self.directory_key.split("-").collect::<Vec<_>>();
                        let length = directory_splitted.len();
                        let back_key = directory_splitted
                            .into_iter()
                            .take(length - 1)
                            .collect::<Vec<_>>()
                            .join("-");
                        self.directory_key = back_key;
                        self.selected_key = if self.directory_key == "" {
                            None
                        } else {
                            Some(self.directory_key.clone())
                        };
                    } else {
                        let element_count = selected_key.matches("-").count();
                        if element_count <= 2 {
                            self.directory_key = selected_key.clone();
                            self.selected_key = Some("back".to_string());
                        } else {
                            self.selected_key = Some(selected_key.clone());
                            // TODO : send event to editor to change image
                        }
                    }
                }
                _ => {}
            }
        };
        self.scroll.update(event);
    }

    pub fn render(&self, props: &ImageBrowserProps) -> RenderingTree {
        let is_root = self.directory_key == "";
        namui::log(format!("is_root: {}", is_root));
        let item_margin = 10.0;
        let item_width = props.width / 2.0 - item_margin;
        let item_size = namui::Wh {
            width: item_width,
            height: item_width,
        };

        let thumbnail_rect = namui::XywhRect {
            x: 10.0,
            y: 5.0,
            width: item_size.width - 20.0,
            height: item_size.height - 20.0,
        };

        let get_browser_item_y =
            |index: usize| item_margin + (index / 2) as f32 * (item_size.height + item_margin);

        let mut browser_items = vec![];
        if !is_root {
            browser_items.push(self.render_back_button(item_size, thumbnail_rect));
        }
        browser_items.extend(
            self.get_browser_item_props(item_size, thumbnail_rect)
                .iter()
                .map(|props| BrowserItem::new().render(props)),
        );
        let browser_items = browser_items
            .into_iter()
            .enumerate()
            .map(|(index, browser_item)| {
                namui::translate(
                    (index % 2) as f32 * (item_size.width + item_margin),
                    get_browser_item_y(index),
                    browser_item,
                )
            })
            .collect::<Vec<_>>();

        let browser_item_scroll_height =
            get_browser_item_y(browser_items.len() - 1) + item_size.height + item_margin;

        let scroll_bar_width = 10.0;

        namui::render![
            self.render_current_directory_label(),
            namui::translate(
                0.0,
                self.current_directory_label_layout.y,
                self.scroll.render(ScrollProps {
                    x: 0.0,
                    y: 0.0,
                    inner_width: props.width - scroll_bar_width,
                    inner_height: browser_item_scroll_height,
                    scroll_bar_width,
                    height: props.height
                        - (self.current_directory_label_layout.y
                            + self.current_directory_label_layout.height),
                    inner_rendering_tree: RenderingTree::Children(browser_items),
                }),
            )
        ]
    }

    fn render_current_directory_label(&self) -> RenderingTree {
        namui::text(namui::TextParam {
            text: self.directory_key.clone(),
            x: self.current_directory_label_layout.x,
            y: self.current_directory_label_layout.y,
            align: namui::TextAlign::Left,
            baseline: namui::TextBaseline::Bottom,
            font_type: namui::FontType {
                size: 16,
                serif: false,
                language: namui::Language::Ko,
                font_weight: namui::FontWeight::REGULAR,
            },
            style: namui::TextStyle {
                color: namui::Color::BLACK,
                ..Default::default()
            },
        })
    }

    fn get_browser_item_props(
        &self,
        item_size: Wh<f32>,
        thumbnail_rect: XywhRect<f32>,
    ) -> Vec<BrowserItemProps> {
        let mut iter = self.directory_key.split("-").filter(|s| !s.is_empty());
        let character = iter.next();
        let pose = iter.next();

        namui::log(format!("self.directory_key: {:?}", self.directory_key));
        namui::log(format!("character: {:?}, pose: {:?}", character, pose));

        if character.is_none() {
            let mut characters = BTreeSet::new();
            for filename_object in &self.image_filename_objects {
                characters.insert(&filename_object.character);
            }
            return characters
                .into_iter()
                .map(|character| {
                    let filename_object = self
                        .image_filename_objects
                        .iter()
                        .find(|filename_object| filename_object.character == *character)
                        .unwrap();
                    let key = format!("{}", character);
                    BrowserItemProps {
                        name: character.to_string(),
                        thumbnail_url: filename_object.url.clone(),
                        key: key.clone(),
                        is_selected: self.selected_key == Some(key.clone()),
                        item_size,
                        thumbnail_rect,
                    }
                })
                .collect();
        };
        let character = character.unwrap();

        if pose.is_none() {
            let mut poses = BTreeSet::new();
            for filename_object in self
                .image_filename_objects
                .iter()
                .filter(|filename_object| filename_object.character == character)
            {
                poses.insert(&filename_object.pose);
            }
            return poses
                .into_iter()
                .map(|pose| {
                    let filename_object = self
                        .image_filename_objects
                        .iter()
                        .find(|filename_object| {
                            filename_object.character == character && filename_object.pose == *pose
                        })
                        .unwrap();
                    let key = format!("{}-{}", character, pose);
                    BrowserItemProps {
                        name: pose.to_string(),
                        thumbnail_url: filename_object.url.clone(),
                        key: key.clone(),
                        is_selected: self.selected_key == Some(key),
                        item_size,
                        thumbnail_rect,
                    }
                })
                .collect();
        };
        let pose = pose.unwrap();

        let mut emotions = BTreeSet::new();
        for filename_object in self
            .image_filename_objects
            .iter()
            .filter(|filename_object| {
                filename_object.character == character && filename_object.pose == pose
            })
        {
            emotions.insert(&filename_object.emotion);
        }

        emotions
            .into_iter()
            .map(|emotion| {
                let filename_object = self
                    .image_filename_objects
                    .iter()
                    .find(|filename_object| {
                        filename_object.character == character
                            && filename_object.pose == pose
                            && filename_object.emotion == *emotion
                    })
                    .unwrap();
                let key = format!("{}-{}-{}", character, pose, emotion);
                BrowserItemProps {
                    name: emotion.to_string(),
                    thumbnail_url: filename_object.url.clone(),
                    key: key.clone(),
                    is_selected: self.selected_key == Some(key),
                    item_size,
                    thumbnail_rect,
                }
            })
            .collect()
    }
}

impl ImageFilenameObject {
    pub fn new(camera_shot_url: &String) -> Self {
        let file_name_with_extension = camera_shot_url.split("/").last().unwrap();
        // remove only extension but keep dot in middle of name.
        let last_dot_index = file_name_with_extension.rfind('.').unwrap();
        let file_name = file_name_with_extension.split_at(last_dot_index).0;

        let mut splits = file_name.split("-");

        let character = splits.next().unwrap();
        let pose = splits.next().unwrap();
        let emotion = splits.collect::<Vec<&str>>().join("-");

        Self {
            character: character.to_string(),
            pose: pose.to_string(),
            emotion,
            url: camera_shot_url.to_string(),
        }
    }
}
