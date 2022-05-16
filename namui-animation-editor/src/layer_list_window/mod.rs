use namui::prelude::*;
use namui_prebuilt::rect_slice::{self, traits::RectSlice};
use std::sync::Arc;
mod body;
mod header;

pub(crate) struct LayerListWindow {
    header: header::Header,
    body: body::Body,
}

pub(crate) struct Props<'a> {
    pub wh: Wh<namui::types::PixelSize>,
    pub layers: &'a [Arc<namui::animation::Layer>],
}

impl LayerListWindow {
    pub(crate) fn new() -> Self {
        Self {
            header: header::Header::new(),
            body: body::Body::new(),
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        self.header.update(event);
        self.body.update(event);
    }
    pub(crate) fn render(&self, props: &Props) -> namui::RenderingTree {
        rect_slice::Slice::Top(&self.header, &self.body).render(
            Wh {
                width: props.wh.width.into(),
                height: props.wh.height.into(),
            },
            (
                header::Props(),
                body::Props {
                    layers: props.layers,
                },
            ),
        )
    }
}
