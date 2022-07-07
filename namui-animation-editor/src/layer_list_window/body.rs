use namui::{animation::Animate, prelude::*, types::Time};
use namui_prebuilt::{table::*, *};

pub struct Body {
    list_view: list_view::ListView,
}
pub struct Props<'a> {
    pub wh: Wh<Px>,
    pub layers: &'a [namui::animation::Layer],
    pub selected_layer_id: Option<String>,
}
impl Body {
    pub fn new() -> Self {
        Self {
            list_view: list_view::ListView::new(),
        }
    }
    pub fn update(&mut self, event: &dyn std::any::Any) {
        self.list_view.update(event);
    }
    pub fn render(&self, props: Props) -> RenderingTree {
        let now = Time::now();
        self.list_view.render(list_view::Props {
            xy: Xy::single(px(0.0)),
            height: props.wh.height,
            item_wh: Wh {
                width: props.wh.width,
                height: px(48.0),
            },
            scroll_bar_width: px(10.0),
            items: props.layers,
            item_render: |wh, layer| {
                let selected = props.selected_layer_id == Some(layer.id.clone());
                render_row(wh, &layer, selected, now)
            },
        })
    }
}

const MARGIN: Px = px(10.0);

fn render_shadowing_toggle_button_cell(wh: Wh<Px>) -> RenderingTree {
    namui::rect(RectParam {
        rect: Rect::Xywh {
            x: MARGIN,
            y: MARGIN,
            width: wh.width - MARGIN * 2.0,
            height: wh.height - MARGIN * 2.0,
        },
        style: RectStyle {
            stroke: Some(RectStroke {
                color: Color::BLACK,
                width: px(1.0),
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

fn render_label_cell(wh: Wh<Px>, layer: &namui::animation::Layer) -> RenderingTree {
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

fn render_row(wh: Wh<Px>, layer: &animation::Layer, is_selected: bool, now: Time) -> RenderingTree {
    let border = match is_selected {
        true => simple_rect(wh, Color::RED, px(2.0), Color::TRANSPARENT),
        false => simple_rect(wh, Color::BLACK, px(1.0), Color::TRANSPARENT),
    };
    render([
        horizontal([
            calculative(
                |parent_wh| parent_wh.height,
                |wh| render_shadowing_toggle_button_cell(wh),
            ),
            ratio(1.0, |wh| render_label_cell(wh, &layer)),
            calculative(
                |parent_wh| parent_wh.height / 1080.0 * 1920.0,
                |wh| render_preview_cell(wh, &layer, now),
            ),
        ])(wh),
        border,
    ])
    .attach_event(move |builder| {
        let layer = layer.clone();
        builder.on_mouse_up_in(move |_| {
            namui::event::send(super::Event::LayerSelected(layer.id.clone()))
        });
    })
}

fn render_preview_cell(wh: Wh<Px>, layer: &namui::animation::Layer, now: Time) -> RenderingTree {
    let border = simple_rect(wh, Color::BLACK, px(1.0), Color::WHITE);
    let preview = {
        let time_range = layer.image.get_visible_time_range();
        match time_range {
            Some((start_time, end_time)) => {
                let duration = end_time - start_time;

                const PLAY_SPEED: f32 = 3.0;
                let playback_time = if duration == Time::Ms(0.0) {
                    start_time
                } else {
                    (now % (duration / PLAY_SPEED)) * PLAY_SPEED
                };

                namui::scale(
                    wh.width / px(1920.0),
                    wh.height / px(1080.0),
                    layer.image.render(playback_time),
                )
            }
            None => RenderingTree::Empty,
        }
    };
    render([border, preview])
}
