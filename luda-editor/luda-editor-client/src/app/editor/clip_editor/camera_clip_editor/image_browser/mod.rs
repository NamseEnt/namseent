use namui::prelude::*;
use std::collections::BTreeSet;
mod browser_item;
use browser_item::*;
mod back_button;
mod scroll;
use crate::app::{
    editor::events::EditorEvent,
    types::{CharacterPoseEmotion, ImageFilenameObject},
};
use scroll::*;
mod types;
pub use types::*;

#[derive(Debug)]
pub struct ImageBrowser {
    directory: ImageBrowserDirectory,
    selected_item: Option<ImageBrowserItem>,
    current_directory_label_layout: XywhRect<f32>,
    scroll: Scroll,
}

impl ImageBrowser {
    pub fn new(character_pose_emotion: &CharacterPoseEmotion) -> Self {
        Self {
            directory: ImageBrowserDirectory::CharacterPose(
                character_pose_emotion.0.clone(),
                character_pose_emotion.1.clone(),
            ),
            selected_item: Some(ImageBrowserItem::CharacterPoseEmotion(
                character_pose_emotion.0.clone(),
                character_pose_emotion.1.clone(),
                character_pose_emotion.2.clone(),
            )),
            scroll: Scroll::new(),
            current_directory_label_layout: XywhRect {
                x: 20.0,
                y: 20.0,
                width: 160.0,
                height: 40.0,
            },
        }
    }
}

pub struct ImageBrowserProps<'a> {
    pub width: f32,
    pub height: f32,
    pub image_filename_objects: &'a Vec<ImageFilenameObject>,
}

impl ImageBrowser {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::ImageBrowserSelectEvent { selected_item } => {
                    namui::log!("selected_item: {:?}", selected_item);
                    match selected_item {
                        ImageBrowserItem::Back => {
                            self.directory = self.directory.parent();
                            self.selected_item = match &self.directory {
                                ImageBrowserDirectory::Root => None,
                                ImageBrowserDirectory::Character(character) => {
                                    Some(ImageBrowserItem::Character(character.clone()))
                                }
                                ImageBrowserDirectory::CharacterPose(character, pose) => {
                                    Some(ImageBrowserItem::CharacterPose(
                                        character.clone(),
                                        pose.clone(),
                                    ))
                                }
                            };
                        }
                        ImageBrowserItem::Character(character) => {
                            self.directory = ImageBrowserDirectory::Character(character.clone());
                            self.selected_item = None;
                        }
                        ImageBrowserItem::CharacterPose(character, pose) => {
                            self.directory = ImageBrowserDirectory::CharacterPose(
                                character.clone(),
                                pose.clone(),
                            );
                            self.selected_item = None;
                        }
                        ImageBrowserItem::CharacterPoseEmotion(character, pose, emotion) => {
                            self.selected_item = Some(ImageBrowserItem::CharacterPoseEmotion(
                                character.clone(),
                                pose.clone(),
                                emotion.clone(),
                            ));
                        }
                    }
                }
                _ => {}
            }
        };
        self.scroll.update(event);
    }

    pub fn render(&self, props: &ImageBrowserProps) -> RenderingTree {
        let is_root = self.directory == ImageBrowserDirectory::Root;
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
            self.get_browser_item_props(item_size, thumbnail_rect, props.image_filename_objects)
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
            text: self.directory.to_string(),
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
        image_filename_objects: &Vec<ImageFilenameObject>,
    ) -> Vec<BrowserItemProps> {
        let under_directory_items = image_filename_objects
            .iter()
            .filter(|filename_object| filename_object.is_just_under_directory(&self.directory))
            .map(|filename_object| filename_object.extract_item_with_directory(&self.directory))
            .collect::<BTreeSet<_>>();

        under_directory_items
            .into_iter()
            .map(|item| {
                let filename_object = image_filename_objects
                    .iter()
                    .find(|filename_object| filename_object.contains(&item))
                    .unwrap();

                BrowserItemProps {
                    name: item.to_string(),
                    thumbnail_url: filename_object.url.clone(),
                    item: item.clone(),
                    is_selected: self.selected_item == Some(item),
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

impl ImageFilenameObject {
    fn is_just_under_directory(&self, directory: &ImageBrowserDirectory) -> bool {
        match directory {
            ImageBrowserDirectory::Root => true,
            ImageBrowserDirectory::Character(character) => self.character == *character,
            ImageBrowserDirectory::CharacterPose(character, pose) => {
                self.character == *character && self.pose == *pose
            }
        }
    }
    fn extract_item_with_directory(&self, directory: &ImageBrowserDirectory) -> ImageBrowserItem {
        match directory {
            ImageBrowserDirectory::Root => ImageBrowserItem::Character(self.character.clone()),
            ImageBrowserDirectory::Character(character) => {
                assert_eq!(self.character, *character);
                ImageBrowserItem::CharacterPose(self.character.clone(), self.pose.clone())
            }
            ImageBrowserDirectory::CharacterPose(character, pose) => {
                assert_eq!(self.character, *character);
                assert_eq!(self.pose, *pose);
                ImageBrowserItem::CharacterPoseEmotion(
                    self.character.clone(),
                    self.pose.clone(),
                    self.emotion.clone(),
                )
            }
        }
    }

    fn contains(&self, item: &ImageBrowserItem) -> bool {
        match item {
            ImageBrowserItem::Character(character) => self.character == *character,
            ImageBrowserItem::CharacterPose(character, pose) => {
                self.character == *character && self.pose == *pose
            }
            ImageBrowserItem::CharacterPoseEmotion(character, pose, emotion) => {
                self.character == *character && self.pose == *pose && self.emotion == *emotion
            }
            ImageBrowserItem::Back => false,
        }
    }
}
