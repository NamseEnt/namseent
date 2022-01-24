pub use self::{
    image_browser::{ImageBrowser, ImageBrowserProps},
    wysiwyg_editor::{WysiwygEditor, WysiwygEditorProps},
};
use crate::app::{editor::job::Job, types::*};
use namui::prelude::*;
use preview::*;
pub mod image_browser;
pub mod preview;
pub mod wysiwyg_editor;

pub struct CameraClipEditor {
    image_browser: ImageBrowser,
    wysiwyg_editor: WysiwygEditor,
}

impl CameraClipEditor {
    pub fn new(character_pose_emotion: &CharacterPoseEmotion) -> Self {
        Self {
            image_browser: ImageBrowser::new(character_pose_emotion),
            wysiwyg_editor: WysiwygEditor::new(),
        }
    }
}

pub struct CameraClipEditorProps<'a> {
    pub camera_clip: &'a CameraClip,
    pub xywh: XywhRect<f32>,
    pub image_filename_objects: &'a Vec<ImageFilenameObject>,
    pub job: &'a Option<Job>,
}

impl CameraAngleImageLoader for CameraClipEditorProps<'_> {
    fn get_image_source(
        &self,
        character_pose_emotion: &CharacterPoseEmotion,
    ) -> Option<ImageSource> {
        self.image_filename_objects
            .iter()
            .find(|object| object.into_character_pose_emotion() == *character_pose_emotion)
            .map(|object| ImageSource::Url(object.url.clone()))
    }
}

impl CameraClipEditor {
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.image_browser.update(event);
    }
    pub fn render(&self, props: &CameraClipEditorProps) -> RenderingTree {
        let camera_angle = &match &props.job {
            // Some(Job::WysiwygMoveImage(job)) => {
            //     let mut camera_angle = props.camera_clip.camera_angle.clone();
            //     job.move_camera_angle(&mut camera_angle);
            //     camera_angle
            // }
            // Some(Job::WysiwygResizeImage(job)) => {
            //     let mut camera_angle = props.camera_clip.camera_angle.clone();
            //     job.resize_camera_angle(&mut camera_angle);
            //     camera_angle
            // }
            // Some(Job::WysiwygCropImage(job)) => {
            //     let mut camera_angle = props.camera_clip.camera_angle.clone();
            //     job.crop_camera_angle(&mut camera_angle);
            //     camera_angle
            // }
            _ => props.camera_clip.camera_angle.clone(),
        };

        let preview_rect = XywhRect {
            x: props.xywh.width / 2.0,
            y: 1.5 * (props.xywh.width / 2.0 / (1920.0 / 1080.0)),
            width: props.xywh.width / 2.0,
            height: props.xywh.width / 2.0 / (1920.0 / 1080.0),
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
                    namui::rect(namui::RectParam {
                        x: 0.0,
                        y: 0.0,
                        width: props.xywh.width,
                        height: props.xywh.height,
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
                    }),
                    self.image_browser.render(&ImageBrowserProps {
                        width: props.xywh.width / 2.0,
                        height: props.xywh.height,
                        image_filename_objects: props.image_filename_objects,
                    }),
                    clip(
                        PathBuilder::new().add_rect(&preview_rect.into_ltrb()),
                        ClipOp::Difference,
                        self.wysiwyg_editor.render(&WysiwygEditorProps {
                            xywh: XywhRect {
                                x: props.xywh.width / 2.0,
                                y: 0.0,
                                width: props.xywh.width / 2.0,
                                height: props.xywh.width / 2.0 / (1920.0 / 1080.0),
                            },
                            camera_angle: &camera_angle,
                            image_filename_objects: props.image_filename_objects,
                            job: &props.job,
                        }),
                    ),
                    Preview::new().render(&PreviewProps {
                        camera_angle: &camera_angle,
                        xywh: &preview_rect,
                        camera_angle_image_loader: props,
                    }),
                ],
            ),
        )
    }
}
