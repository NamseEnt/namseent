use super::{
    canvas::Canvas,
    event::CropperEvent,
    job::{Job, RectSelectionResize},
    render_app_bar::render_app_bar,
    selection::Selection,
};
use crate::app::cropper::canvas::CanvasProps;
use namui::{render, translate, Image, NamuiEvent, RenderingTree, Wh, XywhRect};
use std::sync::Arc;

pub struct CropperProps {
    pub xywh: XywhRect<f32>,
}

pub struct Cropper {
    canvas: Canvas,
    selection_list: Vec<Selection>,
    job: Option<Job>,
}
impl Cropper {
    pub fn new(image: Arc<Image>) -> Self {
        Self {
            canvas: Canvas::new(image.clone()),
            selection_list: Vec::new(),
            job: None,
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<CropperEvent>() {
            match &event {
                CropperEvent::SelectionCreate(selection) => {
                    self.selection_list.push(selection.clone())
                }
                CropperEvent::RectSelectionResizeHandleClicked {
                    selection_id,
                    direction,
                } => {
                    if self.job.is_none() {
                        self.job = Some(Job::RectSelectionResize(RectSelectionResize::new(
                            selection_id.clone(),
                            direction.clone(),
                        )))
                    }
                }
                CropperEvent::MouseMoveInCanvas(position) => {
                    if let Some(job) = &mut self.job {
                        match job {
                            Job::RectSelectionResize(job) => job.update_position(position.clone()),
                        }
                    }
                }
            }
        }
        if let Some(event) = event.downcast_ref::<NamuiEvent>() {
            match &event {
                NamuiEvent::MouseUp(_) => {
                    if let Some(job) = &self.job {
                        match job {
                            Job::RectSelectionResize(_) => {
                                self.execute_job();
                            }
                        }
                    }
                }
                _ => (),
            }
        }
        self.canvas.update(event);
    }

    pub fn render(&self, props: CropperProps) -> RenderingTree {
        const APP_BAR_HEIGHT: f32 = 48.0;

        let job_preview_selection_list =
            get_job_preview_selection_list(&self.selection_list, &self.job);
        let selection_list = match job_preview_selection_list {
            Some(ref selection_list) => selection_list,
            None => &self.selection_list,
        };

        render([
            render_app_bar(Wh {
                width: props.xywh.width,
                height: APP_BAR_HEIGHT,
            }),
            translate(
                0.0,
                APP_BAR_HEIGHT,
                self.canvas.render(CanvasProps {
                    wh: Wh {
                        width: props.xywh.width,
                        height: props.xywh.height - APP_BAR_HEIGHT,
                    },
                    selection_list,
                }),
            ),
        ])
    }

    fn execute_job(&mut self) {
        if let Some(job) = &self.job {
            self.selection_list = job.execute(self.selection_list.clone());
            self.job = None;
        }
    }
}

fn get_job_preview_selection_list(
    selection_list: &Vec<Selection>,
    job: &Option<Job>,
) -> Option<Vec<Selection>> {
    match job {
        Some(job) => Some(job.execute(selection_list.clone())),
        None => None,
    }
}
