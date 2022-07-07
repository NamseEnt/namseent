use super::PlaybackStatus;
use namui::prelude::*;

pub(super) struct ButtonsProps<'a> {
    pub rect: Rect<Px>,
    pub playback_status: &'a PlaybackStatus,
}

pub(super) enum ButtonsEvent {
    PlayButtonClicked,
    PauseButtonClicked,
}

pub(super) fn render_buttons(props: &ButtonsProps) -> RenderingTree {
    let is_toggle_button_play_button = match props.playback_status {
        PlaybackStatus::Paused(_) => true,
        PlaybackStatus::Loading => true,
        PlaybackStatus::Playing(_) => false,
    };

    let play_pause_toggle_button = match is_toggle_button_play_button {
        true => get_1x1_play_button(),
        false => get_1x1_pause_button(),
    };

    let outer_margin: Px = 0.05 * props.rect.height();
    let inner_margin: Px = 0.05 * props.rect.height();
    let button_size = props.rect.height() - 2.0 * (outer_margin + inner_margin);
    let button_center_xy = props.rect.center();

    let play_pause_toggle_button = play_pause_toggle_button
        .scale(button_size.as_f32(), button_size.as_f32())
        .translate(
            button_center_xy.x - button_size / 2.0,
            button_center_xy.y - button_size / 2.0,
        );

    let button_paint = PaintBuilder::new()
        .set_color(Color::BLACK)
        .set_style(PaintStyle::Fill)
        .set_anti_alias(true);

    render([
        rect(RectParam {
            rect: Rect::Xywh {
                x: button_center_xy.x - button_size / 2.0 - inner_margin,
                y: button_center_xy.y - button_size / 2.0 - inner_margin,
                width: button_size + 2.0 * inner_margin,
                height: button_size + 2.0 * inner_margin,
            },
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::BLACK,
                    width: px(1.0),
                    border_position: BorderPosition::Middle,
                }),
                ..Default::default()
            },
        })
        .attach_event(|event_builder| {
            event_builder.on_mouse_down_in(move |_| {
                namui::event::send(match is_toggle_button_play_button {
                    true => ButtonsEvent::PlayButtonClicked,
                    false => ButtonsEvent::PauseButtonClicked,
                })
            });
        }),
        path(play_pause_toggle_button, button_paint),
    ])
}

fn get_1x1_play_button() -> PathBuilder {
    PathBuilder::new()
        .move_to(px(0.0), px(0.0))
        .line_to(px(1.0), px(0.5))
        .line_to(px(0.0), px(1.0))
        .line_to(px(0.0), px(0.0))
        .close()
}

fn get_1x1_pause_button() -> PathBuilder {
    PathBuilder::new()
        .add_rect(Rect::Xywh {
            x: px(0.0),
            y: px(0.0),
            width: px(0.4),
            height: px(1.0),
        })
        .add_rect(Rect::Xywh {
            x: px(0.6),
            y: px(0.0),
            width: px(0.4),
            height: px(1.0),
        })
}
