use namui::{animation::Animate, prelude::*, types::Time};
use namui_prebuilt::*;

pub(crate) struct Body {
    list_view: list_view::ListView,
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
pub(crate) struct Props<'a> {
    pub layers: &'a [namui::animation::Layer],
    pub selected_layer_id: Option<String>,
}
impl table::CellRender<Props<'_>> for Body {
    fn render(&self, wh: Wh<f32>, props: Props) -> RenderingTree {
        let now = Time::now();
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
            item_render: |wh, layer| {
                let selected = props.selected_layer_id == Some(layer.id.clone());
                render_row(wh, &layer, selected, now)
            },
        })
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
            size: crate::adjust_font_size(wh.height - MARGIN * 2.0),
        }
    }),]
}

fn render_row(
    wh: Wh<f32>,
    layer: &animation::Layer,
    is_selected: bool,
    now: Time,
) -> RenderingTree {
    let border = match is_selected {
        true => simple_rect(wh, Color::RED, 2.0, Color::TRANSPARENT),
        false => simple_rect(wh, Color::BLACK, 1.0, Color::TRANSPARENT),
    };
    (render![
        horizontal![
            calculative!(|parent_wh| parent_wh.height, |wh| {
                render_shadowing_toggle_button_cell(wh)
            }),
            ratio!(1.0, |wh| render_label_cell(wh, &layer)),
            calculative!(|parent_wh| { parent_wh.height / 1080.0 * 1920.0 }, |wh| {
                render_preview_cell(wh, &layer, now)
            }),
        ](wh),
        border,
    ])
    .attach_event(move |builder| {
        let layer = layer.clone();
        builder.on_mouse_up(move |_| {
            namui::event::send(super::Event::LayerSelected(layer.id.clone()))
        });
    })
}

fn render_preview_cell(wh: Wh<f32>, layer: &namui::animation::Layer, now: Time) -> RenderingTree {
    let border = simple_rect(wh, Color::BLACK, 1.0, Color::WHITE);
    let preview = {
        let time_range = layer.image.get_visible_time_range();
        match time_range {
            Some((start_time, end_time)) => {
                let duration = end_time - start_time;

                const PLAY_SPEED: f32 = 3.0;
                let playback_time = if duration == Time::zero() {
                    start_time
                } else {
                    (now % (duration / PLAY_SPEED)) * PLAY_SPEED
                };

                namui::scale(
                    wh.width / 1920.0,
                    wh.height / 1080.0,
                    layer.image.render(playback_time),
                )
            }
            None => RenderingTree::Empty,
        }
    };
    render([border, preview])
}
