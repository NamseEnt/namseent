use namui::{prelude::*, types::PixelSize};
use namui_prebuilt::*;
use std::sync::Arc;

pub(crate) struct Body {
    list_view: list_view::ListView,
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Arc<namui::animation::Layer>],
}

impl rect_slice::traits::Fill<Props<'_>> for Body {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        self.list_view.render(list_view::Props {
            x: 0.0,
            y: 0.0,
            height: wh.height.into(),
            item_wh: Wh {
                width: wh.width.into(),
                height: ROW_HEIGHT.into(),
            },
            scroll_bar_width: 10.0,
            items: props.layers,
            item_render: |layer| render_row(&layer, wh.width.into()),
        })
    }
}

impl Body {
    pub(crate) fn new() -> Self {
        Self {
            list_view: list_view::ListView::new(),
        }
    }
    pub(crate) fn update(&mut self, event: &dyn std::any::Any) {
        self.list_view.update(event);
    }
}

const ROW_HEIGHT: PixelSize = PixelSize(48.0);
lazy_static! {
    static ref ROW_LEFT_CELL_WIDTH: PixelSize = ROW_HEIGHT / 1080.0 * 1920.0;
}
const ROW_UNIT_MARGIN: PixelSize = PixelSize(10.0);

fn render_left_cell(layer: &namui::animation::Layer, width: PixelSize) -> RenderingTree {
    // TODO: Preview Animation

    namui::rect(RectParam {
        x: 0.0,
        y: 0.0,
        width: width.into(),
        height: ROW_HEIGHT.into(),
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::BLACK,
                width: 1.0,
                border_position: BorderPosition::Middle,
            }),
            fill: Some(RectFill {
                color: Color::WHITE,
            }),
            ..Default::default()
        },
        ..Default::default()
    })
}

fn render_right_cell(layer: &namui::animation::Layer, width: PixelSize) -> RenderingTree {
    // TODO: Make toggle button, prebuilt in namui_prebuilt too.
    let button_width: PixelSize = ROW_HEIGHT - ROW_UNIT_MARGIN * 2.0;
    let shadowing_toggle_button = namui::rect(RectParam {
        x: 10.0,
        y: 10.0,
        width: button_width.into(),
        height: button_width.into(),
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::BLACK,
                width: 1.0,
                border_position: BorderPosition::Middle,
            }),
            fill: Some(RectFill {
                color: Color::WHITE,
            }),
            ..Default::default()
        },
        ..Default::default()
    });

    let x_next_to_shadowing_toggle_button_margin = ROW_HEIGHT;

    render![
        namui::rect(RectParam {
            x: 0.0,
            y: 0.0,
            width: width.into(),
            height: ROW_HEIGHT.into(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::BLACK,
                    width: 1.0,
                    border_position: BorderPosition::Middle,
                }),
                fill: Some(RectFill {
                    color: Color::WHITE,
                }),
                ..Default::default()
            },
            ..Default::default()
        }),
        shadowing_toggle_button,
        namui::text(TextParam {
            x: (x_next_to_shadowing_toggle_button_margin + ROW_UNIT_MARGIN).into(),
            y: (ROW_HEIGHT / 2.0).into(),
            text: layer.name.clone(),
            style: TextStyle {
                color: Color::BLACK,
                ..Default::default()
            },
            align: TextAlign::Left,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                font_weight: FontWeight::REGULAR,
                language: Language::Ko,
                serif: false,
                size: ((ROW_HEIGHT - ROW_UNIT_MARGIN * 2) / 0.8).into(),
            }
        }),
    ]
}

fn render_row(layer: &animation::Layer, width: PixelSize) -> RenderingTree {
    // TODO

    render![
        render_left_cell(&layer, *ROW_LEFT_CELL_WIDTH),
        namui::translate(
            (*ROW_LEFT_CELL_WIDTH).into(),
            0.0,
            render_right_cell(&layer, width - *ROW_LEFT_CELL_WIDTH)
        )
    ]
}
