use crate::editor::{events::EditorEvent, types::*};
use namui::prelude::*;

pub struct ScrollProps {
    pub scroll_y: f32,
    pub x: f32,
    pub y: f32,
    pub scroll_bar_width: f32,
    pub inner_width: f32,
    pub inner_height: f32,
    pub height: f32,
    pub inner_rendering_tree: RenderingTree,
}

pub fn render_scroll(
    ScrollProps {
        scroll_y,
        x,
        y,
        scroll_bar_width,
        inner_width,
        inner_height,
        height,
        inner_rendering_tree,
    }: ScrollProps,
) -> RenderingTree {
    let scroll_y = num::clamp(scroll_y, 0.0, (0.0_f32).max(inner_height - height));

    let inner = namui::clip(
        namui::Path::new().add_rect(namui::LtrbRect {
            left: 0.0,
            top: 0.0,
            right: inner_width,
            bottom: height,
        }),
        namui::ClipOp::Intersect,
        namui::translate(0.0, -scroll_y, inner_rendering_tree),
    );

    let scroll_bar_handle_weight = height.powf(2.0) / inner_height;
    let scroll_bar = match inner_height > height {
        true => render![
            rect(RectParam {
                x: inner_width,
                y: 0.0,
                width: scroll_bar_width,
                height,
                style: RectStyle {
                    stroke: Some(RectStroke {
                        width: 1.0,
                        border_position: BorderPosition::Inside,
                        color: Color::BLACK,
                    }),
                    fill: Some(RectFill {
                        color: Color::WHITE,
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
            rect(RectParam {
                x: inner_width,
                y: scroll_y,
                width: scroll_bar_width,
                height: scroll_bar_handle_weight,
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::gary_scale_f01(0.5),
                    }),
                    ..Default::default()
                },
                ..Default::default()
            }),
        ],
        false => RenderingTree::Empty,
    };

    let whole_rect = rect(RectParam {
        x: 0.0,
        y: 0.0,
        width: inner_width + scroll_bar_width,
        height,
        style: RectStyle {
            stroke: Some(RectStroke {
                width: 1.0,
                border_position: BorderPosition::Middle,
                color: Color::BLACK,
            }),
            ..Default::default()
        },
        // TODO : OnAfterDraw - Wheel rust/luda-ts/editor/client/src/cameraAngleEditor/imageBrowser/Scroll.ts:97
        ..Default::default()
    });

    namui::translate(x, y, render![whole_rect, inner, scroll_bar])
}
