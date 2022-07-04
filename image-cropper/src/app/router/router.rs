use super::{Page, RouterEvent};
use crate::app::{
    cropper::CropperProps,
    file_selector::{FileSelector, FileSelectorProps},
};
use namui::prelude::*;

pub struct RouterProps {
    pub screen_wh: Wh<Px>,
}

pub struct Router {
    page: Page,
}

impl Router {
    pub fn new() -> Self {
        Self {
            page: Page::FileSelector(FileSelector::new()),
        }
    }

    pub fn update(&mut self, event: &dyn std::any::Any) {
        if let Some(event) = event.downcast_ref::<RouterEvent>() {
            match &event {
                RouterEvent::PageChangeRequestedToFileSelectorEvent(initializer) => {
                    self.page = Page::FileSelector(initializer());
                }
                RouterEvent::PageChangeRequestedToCropperEvent(initializer) => {
                    self.page = Page::Cropper(initializer());
                }
            }
        }
        match &mut self.page {
            Page::FileSelector(file_selector) => file_selector.update(event),
            Page::Cropper(cropper) => cropper.update(event),
        }
    }

    pub fn render(&self, props: &RouterProps) -> namui::RenderingTree {
        match &self.page {
            Page::FileSelector(file_selector) => file_selector.render(&FileSelectorProps {
                screen_wh: props.screen_wh.clone(),
            }),
            Page::Cropper(cropper) => cropper.render(CropperProps {
                rect: Rect::Xywh {
                    x: px(0.0),
                    y: px(0.0),
                    width: props.screen_wh.width,
                    height: props.screen_wh.height,
                },
            }),
        }
    }
}
