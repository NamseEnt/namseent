use namui::{prelude::*, types::PixelSize};
use namui_prebuilt::*;
use std::sync::Arc;

pub(crate) struct Body {
    list_view: list_view::ListView,
}

pub(crate) struct Props<'a> {
    pub layers: &'a [Arc<namui::animation::Layer>],
}

impl table::CellRender<Props<'_>> for Body {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        self.list_view.render(list_view::Props {
            x: 0.0,
            y: 0.0,
            height: wh.height.into(),
            item_wh: Wh {
                width: wh.width.into(),
                height: 48.0,
            },
            scroll_bar_width: 10.0,
            items: props.layers,
            item_render: |wh, layer| render_row(wh, &layer),
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

const MARGIN: f32 = 10.0;

fn render_shadowing_toggle_button_cell(wh: Wh<f32>) -> RenderingTree {
    namui::rect(RectParam {
        x: MARGIN,
        y: MARGIN,
        width: wh.width - MARGIN * 2.0,
        height: wh.height - MARGIN * 2.0,
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::BLACK,
                width: 1.0,
                border_position: BorderPosition::Inside,
            }),
            fill: Some(RectFill {
                color: Color::WHITE,
            }),
            ..Default::default()
        },
        ..Default::default()
    })
}

fn get_font_size(height: f32) -> i16 {
    // 0, 4, 8, 16, 20, ...
    let mut font_size = (height * 0.8) as i16;
    if font_size % 4 != 0 {
        font_size += 4 - font_size % 4;
    }
    font_size
}

fn render_label_cell(wh: Wh<f32>, layer: &namui::animation::Layer) -> RenderingTree {
    render![namui::text(TextParam {
        x: MARGIN,
        y: wh.height / 2.0,
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
            size: get_font_size(wh.height - MARGIN * 2.0),
        }
    }),]
}

fn render_preview_cell(wh: Wh<f32>, layer: &namui::animation::Layer) -> RenderingTree {
    // TODO: Preview Animation

    simple_rect(wh, Color::BLACK, 1.0, Color::WHITE)
}

fn render_row(wh: Wh<f32>, layer: &animation::Layer) -> RenderingTree {
    render![
        simple_rect(wh, Color::BLACK, 1.0, Color::WHITE),
        horizontal![
            calculative!(|parent_wh| parent_wh.height, |wh| {
                render_shadowing_toggle_button_cell(wh)
            }),
            ratio!(1.0, |wh| render_label_cell(wh, layer)),
            calculative!(|parent_wh| { parent_wh.height / 1080.0 * 1920.0 }, |wh| {
                render_preview_cell(wh, layer)
            }),
        ](wh),
    ]
}
