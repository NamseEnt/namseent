use self::{image_browser::*, wysiwyg_editor::*};
use crate::app::{
    editor::{events::EditorEvent, job::Job},
    types::*,
};
use namui::prelude::*;
use preview::*;
use std::{collections::BTreeSet, sync::Arc};
mod button;
pub mod image_browser;
pub mod preview;
pub mod wysiwyg_editor;
use button::*;
mod tab;
use tab::*;

pub struct CameraClipEditor {
    character_image_browser: ImageBrowser,
    background_image_browser: ImageBrowser,
    character_wysiwyg_editor: CharacterWysiwygEditor,
    background_wysiwyg_editor: BackgroundWysiwygEditor,
    selected_tab: Tab,
    clip_id: String,
}

#[derive(Debug, Clone)]
pub enum WysiwygTarget {
    Character,
    Background,
}

pub struct CameraClipEditorProps<'a> {
    pub camera_clip: &'a CameraClip,
    pub rect: Rect<Px>,
    pub character_image_files: &'a BTreeSet<ImageBrowserFile>,
    pub background_image_files: &'a BTreeSet<ImageBrowserFile>,
    pub job: &'a Option<Job>,
}

impl CameraClipEditor {
    pub fn new(clip: &CameraClip) -> Self {
        let character_image_directory = get_character_image_directory(clip);
        let character_image_item = get_character_image_item(clip);
        let background_image_directory = get_background_image_directory(clip);
        let background_image_item = get_background_image_item(clip);
        Self {
            character_image_browser: ImageBrowser::new(
                "character",
                character_image_directory,
                character_image_item,
                "http://localhost:3030/resources/characterImages",
            ),
            background_image_browser: ImageBrowser::new(
                "background",
                background_image_directory,
                background_image_item,
                "http://localhost:3030/resources/backgrounds",
            ),
            character_wysiwyg_editor: CharacterWysiwygEditor::new(),
            background_wysiwyg_editor: BackgroundWysiwygEditor::new(),
            selected_tab: Tab::CharacterImage,
            clip_id: clip.id.clone(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.character_image_browser.update(event);

        if let Some(event) = event.downcast_ref::<TabEvent>() {
            match event {
                TabEvent::ClickTabButton(tab) => self.selected_tab = *tab,
            }
        } else if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::SequenceUpdateEvent { sequence } => {
                    sequence.find_clip(&self.clip_id).map(|clip| {
                        let character_image_item = get_character_image_item(clip);
                        self.character_image_browser.select(character_image_item);
                        let background_image_item = get_background_image_item(clip);
                        self.background_image_browser.select(background_image_item);
                    });
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<ImageBrowserEvent>() {
            match event {
                ImageBrowserEvent::Select { browser_id, item } => {
                    if self.character_image_browser.get_id().eq(browser_id) {
                        let character_pose_emotion = match item {
                            ImageBrowserItem::Empty => Some(None),
                            ImageBrowserItem::File(file) => {
                                Some(Some(convert_file_to_character_pose_emotion(file)))
                            }
                            _ => None,
                        };
                        character_pose_emotion.map(|character_pose_emotion| {
                            namui::event::send(EditorEvent::CharacterImageBrowserSelectEvent {
                                character_pose_emotion,
                            });
                        });
                    } else if self.background_image_browser.get_id().eq(browser_id) {
                        let background_name = match item {
                            ImageBrowserItem::Empty => Some(None),
                            ImageBrowserItem::File(file) => {
                                Some(Some(convert_file_to_background(file)))
                            }
                            _ => None,
                        };
                        background_name.map(|background_name| {
                            namui::event::send(EditorEvent::BackgroundImageBrowserSelectEvent {
                                background_name,
                            });
                        });
                    }
                }
            }
        } else if let Some(event) = event.downcast_ref::<WysiwygEvent>() {
            match event {
                WysiwygEvent::ResizerHandleMouseDownEvent {
                    target_id,
                    mouse_xy,
                    handle,
                    center_xy,
                    container_size,
                    image_size_ratio,
                } => {
                    let target = self.get_wysiwyg_target(target_id);
                    namui::event::send(EditorEvent::WysiwygEditorResizerHandleMouseDownEvent {
                        target,
                        mouse_xy: mouse_xy.clone(),
                        handle: handle.clone(),
                        center_xy: center_xy.clone(),
                        container_size: container_size.clone(),
                        image_size_ratio: image_size_ratio.clone(),
                    });
                }
                WysiwygEvent::InnerImageMouseDownEvent {
                    container_size,
                    mouse_xy,
                    target_id,
                } => {
                    let target = self.get_wysiwyg_target(target_id);
                    namui::event::send(EditorEvent::WysiwygEditorInnerImageMouseDownEvent {
                        target,
                        container_size: container_size.clone(),
                        mouse_xy: mouse_xy.clone(),
                    });
                }
                _ => {}
            }
        }
    }
    pub fn render(&self, props: &CameraClipEditorProps) -> RenderingTree {
        let left_box_wh = Wh {
            width: props.rect.width() * 0.25,
            height: props.rect.height(),
        };
        let right_box_wh = Wh {
            width: props.rect.width() - left_box_wh.width,
            height: props.rect.height(),
        };

        let camera_angle = &match &props.job {
            Some(Job::WysiwygMoveImage(job)) => {
                let mut camera_angle = props.camera_clip.camera_angle.clone();
                job.move_camera_angle(&mut camera_angle);
                camera_angle
            }
            Some(Job::WysiwygResizeImage(job)) => {
                let mut camera_angle = props.camera_clip.camera_angle.clone();
                job.resize_camera_angle(&mut camera_angle);
                camera_angle
            }
            Some(Job::WysiwygCropImage(job)) => {
                let mut camera_angle = props.camera_clip.camera_angle.clone();
                job.crop_camera_angle(&mut camera_angle);
                camera_angle
            }
            _ => props.camera_clip.camera_angle.clone(),
        };

        namui::translate(
            props.rect.x(),
            props.rect.y(),
            namui::clip(
                namui::PathBuilder::new().add_rect(namui::Rect::Ltrb {
                    left: px(0.0),
                    top: px(0.0),
                    right: props.rect.width(),
                    bottom: props.rect.height(),
                }),
                namui::ClipOp::Intersect,
                namui::render([
                    self.render_left_box(
                        left_box_wh,
                        &camera_angle,
                        &LudaEditorServerCameraAngleImageLoader {},
                    ),
                    namui::translate(
                        left_box_wh.width,
                        px(0.0),
                        self.render_right_box(right_box_wh, &camera_angle, &props),
                    ),
                ]),
            ),
        )
    }

    fn render_left_box(
        &self,
        wh: Wh<Px>,
        camera_angle: &CameraAngle,
        camera_angle_image_loader: &dyn CameraAngleImageLoader,
    ) -> RenderingTree {
        let tab_button_wh = Wh {
            width: wh.width,
            height: wh.width * 0.2,
        };
        let tab_buttons: Vec<_> = ALL_TABS
            .iter()
            .enumerate()
            .map(|(index, tab)| {
                render_button(
                    &ButtonProps {
                        rect: Rect::Xywh {
                            x: px(0.0),
                            y: index * tab_button_wh.height,
                            width: tab_button_wh.width,
                            height: tab_button_wh.height,
                        },
                        text: tab.get_name(),
                        selected: self.selected_tab == *tab,
                    },
                    Arc::new(|| namui::event::send(TabEvent::ClickTabButton(*tab))),
                )
            })
            .collect();

        let preview_rect = Rect::Xywh {
            x: px(0.0),
            y: tab_buttons.len() as f32 * tab_button_wh.height,
            width: wh.width,
            height: wh.width / (1920.0 / 1080.0),
        };

        let preview = Preview::new().render(&PreviewProps {
            camera_angle: &camera_angle,
            rect: preview_rect,
            camera_angle_image_loader,
        });

        RenderingTree::Children(
            tab_buttons
                .into_iter()
                .chain([preview].into_iter())
                .collect(),
        )
    }

    fn render_right_box(
        &self,
        wh: Wh<Px>,
        camera_angle: &CameraAngle,
        props: &CameraClipEditorProps,
    ) -> RenderingTree {
        let border = namui::rect(namui::RectParam {
            rect: Rect::Xywh {
                x: px(0.0),
                y: px(0.0),
                width: wh.width,
                height: wh.height,
            },
            style: namui::RectStyle {
                stroke: Some(namui::RectStroke {
                    color: namui::Color::BLACK,
                    width: px(1.0),
                    border_position: namui::BorderPosition::Inside,
                }),
                fill: Some(namui::RectFill {
                    color: namui::Color::WHITE,
                }),
                ..Default::default()
            },
            ..Default::default()
        });
        let content = match self.selected_tab {
            Tab::CharacterImage => self.character_image_browser.render(&ImageBrowserProps {
                width: wh.width,
                height: wh.height,
                files: props.character_image_files,
            }),
            Tab::CharacterPosition => {
                self.character_wysiwyg_editor
                    .render(&CharacterWysiwygEditorProps {
                        rect: Rect::Xywh {
                            x: px(0.0),
                            y: px(0.0),
                            width: wh.width,
                            height: wh.width / (1920.0 / 1080.0),
                        },
                        camera_angle,
                    })
            }
            Tab::BackgroundImage => self.background_image_browser.render(&ImageBrowserProps {
                width: wh.width,
                height: wh.height,
                files: props.background_image_files,
            }),
            Tab::BackgroundPosition => {
                self.background_wysiwyg_editor
                    .render(&BackgroundWysiwygEditorProps {
                        rect: Rect::Xywh {
                            x: px(0.0),
                            y: px(0.0),
                            width: wh.width,
                            height: wh.height,
                        },
                        camera_angle,
                    })
            }
        };
        namui::clip(
            namui::PathBuilder::new().add_rect(namui::Rect::Ltrb {
                left: px(0.0),
                top: px(0.0),
                right: wh.width,
                bottom: wh.height,
            }),
            namui::ClipOp::Intersect,
            namui::render([border, content]),
        )
    }

    fn get_wysiwyg_target(&self, target_id: &str) -> WysiwygTarget {
        if self.character_wysiwyg_editor.get_id() == target_id {
            WysiwygTarget::Character
        } else if self.background_wysiwyg_editor.get_id() == target_id {
            WysiwygTarget::Background
        } else {
            unreachable!()
        }
    }
}
fn convert_file_to_character_pose_emotion(file: &ImageBrowserFile) -> CharacterPoseEmotion {
    let url = file.get_url();
    // remove only extension but keep dot in middle of name.
    let last_dot_index = url.rfind('.').unwrap();

    let mut splits = url.split_at(last_dot_index).0.split("/");

    let _ = splits.next().unwrap();
    let character = splits.next().unwrap();
    let pose = splits.next().unwrap();
    let emotion = splits.next().unwrap();

    CharacterPoseEmotion(character.to_string(), pose.to_string(), emotion.to_string())
}
fn convert_file_to_background(file: &ImageBrowserFile) -> String {
    let url = file.get_url();
    // remove only extension but keep dot in middle of name.
    let last_dot_index = url.rfind('.').unwrap();

    url.split_at(last_dot_index).0[1..].to_string()
}
fn get_character_image_item(clip: &CameraClip) -> Option<ImageBrowserItem> {
    match clip.camera_angle.character.as_ref() {
        Some(character) => Some(ImageBrowserItem::File(ImageBrowserFile::new(
            character.character_pose_emotion.to_url(),
        ))),
        None => Some(ImageBrowserItem::Empty),
    }
}
fn get_character_image_directory(clip: &CameraClip) -> ImageBrowserDirectory {
    match clip.camera_angle.character.as_ref() {
        Some(character) => {
            ImageBrowserFile::new(character.character_pose_emotion.to_url()).get_directory()
        }
        None => ImageBrowserDirectory::root(),
    }
}
fn get_background_image_item(clip: &CameraClip) -> Option<ImageBrowserItem> {
    match clip.camera_angle.background.as_ref() {
        Some(background) => Some(ImageBrowserItem::File(ImageBrowserFile::new(format!(
            "/{}.jpeg",
            background.name
        )))),
        None => Some(ImageBrowserItem::Empty),
    }
}
fn get_background_image_directory(clip: &CameraClip) -> ImageBrowserDirectory {
    match clip.camera_angle.background.as_ref() {
        Some(background) => {
            ImageBrowserFile::new(format!("/{}.jpeg", background.name)).get_directory()
        }
        None => ImageBrowserDirectory::root(),
    }
}
