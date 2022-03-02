use self::image_browser::{
    ImageBrowserDirectory, ImageBrowserEvent, ImageBrowserFile, ImageBrowserItem,
};
pub use self::{
    image_browser::{ImageBrowser, ImageBrowserProps},
    wysiwyg_editor::{WysiwygEditor, WysiwygEditorProps},
};
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
    image_browser: ImageBrowser,
    wysiwyg_editor: WysiwygEditor,
    selected_tab: Tab,
    clip_id: String,
}

pub struct CameraClipEditorProps<'a> {
    pub camera_clip: &'a CameraClip,
    pub xywh: XywhRect<f32>,
    pub character_image_files: &'a BTreeSet<ImageBrowserFile>,
    pub job: &'a Option<Job>,
}

impl CameraClipEditor {
    pub fn new(clip: &CameraClip) -> Self {
        let character_image_directory = get_character_image_directory(clip);
        let character_image_item = get_character_image_item(clip);
        Self {
            image_browser: ImageBrowser::new(
                character_image_directory,
                character_image_item,
                "http://localhost:3030/resources/characterImages",
            ),
            wysiwyg_editor: WysiwygEditor::new(),
            selected_tab: Tab::CharacterImage,
            clip_id: clip.id.clone(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.image_browser.update(event);

        if let Some(event) = event.downcast_ref::<TabEvent>() {
            match event {
                TabEvent::ClickTabButton(tab) => self.selected_tab = *tab,
            }
        } else if let Some(event) = event.downcast_ref::<EditorEvent>() {
            match event {
                EditorEvent::SequenceUpdateEvent { sequence } => {
                    sequence.find_clip(&self.clip_id).map(|clip| {
                        let item = get_character_image_item(clip);
                        self.image_browser.select(item);
                    });
                }
                _ => {}
            }
        } else if let Some(event) = event.downcast_ref::<ImageBrowserEvent>() {
            match event {
                ImageBrowserEvent::Select(item) => {
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
                }
                _ => {}
            }
        }
    }
    pub fn render(&self, props: &CameraClipEditorProps) -> RenderingTree {
        let left_box_wh = Wh {
            width: props.xywh.width * 0.25,
            height: props.xywh.height,
        };
        let right_box_wh = Wh {
            width: props.xywh.width - left_box_wh.width,
            height: props.xywh.height,
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
            props.xywh.x,
            props.xywh.y,
            namui::clip(
                namui::PathBuilder::new().add_rect(&namui::LtrbRect {
                    left: 0.0,
                    top: 0.0,
                    right: props.xywh.width,
                    bottom: props.xywh.height,
                }),
                namui::ClipOp::Intersect,
                namui::render![
                    self.render_left_box(
                        &left_box_wh,
                        &camera_angle,
                        &LudaEditorServerCameraAngleImageLoader {}
                    ),
                    namui::translate(
                        left_box_wh.width,
                        0.0,
                        self.render_right_box(
                            &right_box_wh,
                            &camera_angle,
                            props.character_image_files,
                            props.job,
                        )
                    ),
                ],
            ),
        )
    }

    fn render_left_box(
        &self,
        wh: &Wh<f32>,
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
                        xywh: &XywhRect {
                            x: 0.0,
                            y: index as f32 * tab_button_wh.height,
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

        let preview_rect = XywhRect {
            x: 0.0,
            y: tab_buttons.len() as f32 * tab_button_wh.height,
            width: wh.width,
            height: wh.width / (1920.0 / 1080.0),
        };

        let preview = Preview::new().render(&PreviewProps {
            camera_angle: &camera_angle,
            xywh: &preview_rect,
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
        wh: &Wh<f32>,
        camera_angle: &CameraAngle,
        character_image_files: &BTreeSet<ImageBrowserFile>,
        job: &Option<Job>,
    ) -> RenderingTree {
        let border = namui::rect(namui::RectParam {
            x: 0.0,
            y: 0.0,
            width: wh.width,
            height: wh.height,
            style: namui::RectStyle {
                stroke: Some(namui::RectStroke {
                    color: namui::Color::BLACK,
                    width: 1.0,
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
            Tab::CharacterImage => self.image_browser.render(&ImageBrowserProps {
                width: wh.width,
                height: wh.height,
                files: character_image_files,
            }),
            Tab::CharacterPosition => self.wysiwyg_editor.render(&WysiwygEditorProps {
                xywh: XywhRect {
                    x: 0.0,
                    y: 0.0,
                    width: wh.width,
                    height: wh.width / (1920.0 / 1080.0),
                },
                camera_angle,
                character_image_files,
                job,
            }),
            Tab::BackgroundImage => todo!(),
            Tab::BackgroundPosition => todo!(),
        };
        namui::clip(
            namui::PathBuilder::new().add_rect(&namui::LtrbRect {
                left: 0.0,
                top: 0.0,
                right: wh.width,
                bottom: wh.height,
            }),
            namui::ClipOp::Intersect,
            namui::render![border, content],
        )
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

fn get_character_image_item(clip: &CameraClip) -> Option<ImageBrowserItem> {
    match clip.camera_angle.character_pose_emotion.as_ref() {
        Some(character_pose_emotion) => Some(ImageBrowserItem::File(ImageBrowserFile::new(
            character_pose_emotion.to_url(),
        ))),
        None => Some(ImageBrowserItem::Empty),
    }
}

fn get_character_image_directory(clip: &CameraClip) -> ImageBrowserDirectory {
    match clip.camera_angle.character_pose_emotion.as_ref() {
        Some(character_pose_emotion) => {
            ImageBrowserFile::new(character_pose_emotion.to_url()).get_directory()
        }
        None => ImageBrowserDirectory::root(),
    }
}
