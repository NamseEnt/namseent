use super::{
    canvas::{Canvas, CanvasEvent},
    event::CropperEvent,
    job::{
        Job, PolySelectionCreate, RectSelectionCreate, RectSelectionResize,
        RectSelectionResizeDirection,
    },
    render_app_bar::render_app_bar,
    save_image::save_image,
    selection::Selection,
};
use crate::app::cropper::{canvas::CanvasProps, selection::SelectionEvent};
use namui::prelude::*;
use std::sync::Arc;

pub struct CropperProps {
    pub rect: Rect<Px>,
}

pub struct Cropper {
    image_url: String,
    image_name: String,
    canvas: Canvas,
    selection_list: Vec<Selection>,
    job: Option<Job>,
}
impl Cropper {
    pub fn new(image: Arc<Image>, url: String, name: String) -> Self {
        Self {
            image_url: url,
            image_name: name,
            canvas: Canvas::new(image.clone()),
            selection_list: Vec::new(),
            job: None,
        }
    }

    pub fn update(&mut self, event: &namui::Event) {
        event
            .is::<CropperEvent>(|event| match &event {
                CropperEvent::SaveButtonClicked => save_image(
                    self.image_url.clone(),
                    self.image_name.clone(),
                    self.selection_list.clone(),
                ),
                CropperEvent::MouseUp => {
                    if let Some(job) = &self.job {
                        match job {
                            Job::RectSelectionResize(_) | Job::RectSelectionCreate(_) => {
                                self.execute_job();
                            }
                            Job::PolySelectionCreate(job) => {
                                if job.is_done() {
                                    self.execute_job();
                                }
                            }
                        }
                    }
                }
            })
            .is::<CanvasEvent>(|event| match *event {
                CanvasEvent::LeftMouseDownInCanvas {
                    position,
                    tool_type,
                } => match tool_type {
                    super::canvas::ToolType::RectSelection => {
                        self.create_rect_selection_create_job(position);
                    }
                    super::canvas::ToolType::PolySelection => {
                        self.create_poly_selection_create_job(position);
                        if let Some(Job::PolySelectionCreate(job)) = &mut self.job {
                            job.add_point(position.clone());
                        }
                    }
                    _ => (),
                },
                CanvasEvent::MouseMoveInCanvas(position) => {
                    if let Some(job) = &mut self.job {
                        match job {
                            Job::RectSelectionResize(job) => job.update_position(position.clone()),
                            Job::RectSelectionCreate(job) => job.update_position(position.clone()),
                            Job::PolySelectionCreate(job) => job.update_position(position.clone()),
                        }
                    }
                }
                _ => (),
            })
            .is::<SelectionEvent>(|event| match *event {
                SelectionEvent::RectSelectionResizeHandleClicked {
                    selection_id,
                    direction,
                } => {
                    self.create_rect_selection_resize_job(selection_id, direction);
                }
                SelectionEvent::PolySelectionCreateButtonClicked => {
                    if let Some(Job::PolySelectionCreate(job)) = &mut self.job {
                        job.done();
                    }
                }
                SelectionEvent::SelectionRightClicked { target_id } => {
                    self.remove_selection(target_id)
                }
            });
        self.canvas.update(event);
    }

    pub fn render(&self, props: CropperProps) -> RenderingTree {
        const APP_BAR_HEIGHT: Px = px(48.0);

        let job_preview_selection_list =
            get_job_preview_selection_list(&self.selection_list, &self.job);
        let selection_list = match job_preview_selection_list {
            Some(ref selection_list) => selection_list,
            None => &self.selection_list,
        };

        render([
            render_app_bar(Wh {
                width: props.rect.width(),
                height: APP_BAR_HEIGHT,
            }),
            translate(
                px(0.0),
                APP_BAR_HEIGHT,
                self.canvas.render(CanvasProps {
                    wh: Wh {
                        width: props.rect.width(),
                        height: props.rect.height() - APP_BAR_HEIGHT,
                    },
                    selection_list,
                }),
            ),
        ])
        .attach_event(|build| {
            build.on_mouse(|event| {
                if event.event_type == MouseEventType::Up {
                    namui::event::send(CropperEvent::MouseUp)
                }
            });
        })
    }

    fn create_rect_selection_create_job(&mut self, position: Xy<Px>) {
        if self.job.is_none() {
            self.job = Some(Job::RectSelectionCreate(RectSelectionCreate::new(position)))
        }
    }

    fn create_rect_selection_resize_job(
        &mut self,
        selection_id: Uuid,
        direction: RectSelectionResizeDirection,
    ) {
        if self.job.is_none() {
            self.job = Some(Job::RectSelectionResize(RectSelectionResize::new(
                selection_id,
                direction,
            )))
        }
    }

    fn create_poly_selection_create_job(&mut self, position: Xy<Px>) {
        if self.job.is_none() {
            self.job = Some(Job::PolySelectionCreate(PolySelectionCreate::new(position)))
        }
    }

    fn remove_selection(&mut self, target_id: Uuid) {
        self.selection_list
            .retain(|selection| selection.get_id() != target_id)
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
