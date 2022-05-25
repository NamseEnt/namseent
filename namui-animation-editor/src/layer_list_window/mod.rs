use namui::{animation::Layer, prelude::*};
use namui_prebuilt::*;
use std::sync::Arc;
mod body;
mod header;

pub(crate) struct LayerListWindow {
    header: header::Header,
    body: body::Body,
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Arc<Layer>],
}

pub(crate) enum Event {
    LayerSelected(Arc<Layer>),
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
}

impl table::CellRender<Props<'_>> for LayerListWindow {
    fn render(&self, wh: Wh<f32>, props: Props<'_>) -> RenderingTree {
        render![
            simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
            vertical![
                fixed!(20.0, &self.header, header::Props()),
                ratio!(
                    1.0,
                    &self.body,
                    body::Props {
                        layers: props.layers
                    }
                ),
            ](Wh {
                width: wh.width.into(),
                height: wh.height.into(),
            })
        ]
    }
}
