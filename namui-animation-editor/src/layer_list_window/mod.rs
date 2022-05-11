use namui::prelude::*;
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
        let header_height = namui::types::PixelSize::new(20.0).min(props.wh.height);
        namui::render![
            self.header.render(&header::Props {
                wh: Wh {
                    width: props.wh.width,
                    height: header_height,
                },
            }),
            translate(
                0.0,
                header_height.into(),
                self.body.render(&body::Props {
                    wh: Wh {
                        width: props.wh.width,
                        height: props.wh.height - header_height,
                    },
                    layers: props.layers,
                })
            ),
        ]
    }
}
