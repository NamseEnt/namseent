use namui::prelude::*;
use namui_prebuilt::dubu;
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
        let header_row = dubu::Row {
            height: dubu::Size::Fixed(props.wh.height.min(20.0.into()).into()),
            dubu: &self.header,
        };

        let container = dubu::Container {
            wh: Wh {
                width: props.wh.width.into(),
                height: props.wh.height.into(),
            },
            dubu: &dubu::Slice::Top(header_row, &self.body),
        };

        container.render((
            header::Props(),
            body::Props {
                layers: props.layers,
            },
        ))
    }
}
