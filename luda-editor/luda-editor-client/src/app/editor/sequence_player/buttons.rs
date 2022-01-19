use super::PlaybackStatus;
use namui::prelude::*;

pub(super) struct ButtonsProps<'a> {
    pub xywh: &'a XywhRect<f32>,
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

    let outer_margin = 0.05 * props.xywh.height;
    let inner_margin = 0.05 * props.xywh.height;
    let button_size = props.xywh.height - 2.0 * (outer_margin + inner_margin);
    let button_center_xy = props.xywh.center();

    let play_pause_toggle_button = play_pause_toggle_button
        .scale(button_size, button_size)
        .translate(
            button_center_xy.x - button_size / 2.0,
            button_center_xy.y - button_size / 2.0,
        );

    let button_paint = PaintBuilder::new()
        .set_color(Color::BLACK)
        .set_style(PaintStyle::Fill)
        .set_anti_alias(true);

    render![
        rect(RectParam {
            x: button_center_xy.x - button_size / 2.0 - inner_margin,
            y: button_center_xy.y - button_size / 2.0 - inner_margin,
            width: button_size + 2.0 * inner_margin,
            height: button_size + 2.0 * inner_margin,
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::BLACK,
                    width: 1.0,
                    border_position: BorderPosition::Middle,
                }),
                ..Default::default()
            },
        })
        .attach_event(|event_builder| {
            event_builder.on_mouse_down(Box::new(move |_| {
                namui::event::send(match is_toggle_button_play_button {
                    true => ButtonsEvent::PlayButtonClicked,
                    false => ButtonsEvent::PauseButtonClicked,
                })
            }))
        }),
        path(play_pause_toggle_button, button_paint)
    ]
}

fn get_1x1_play_button() -> PathBuilder {
    PathBuilder::new()
        .move_to(0.0, 0.0)
        .line_to(1.0, 0.5)
        .line_to(0.0, 1.0)
        .line_to(0.0, 0.0)
        .close()
}

fn get_1x1_pause_button() -> PathBuilder {
    PathBuilder::new()
        .add_rect(
            &XywhRect {
                x: 0.0,
                y: 0.0,
                width: 0.4,
                height: 1.0,
            }
            .into_ltrb(),
        )
        .add_rect(
            &XywhRect {
                x: 0.6,
                y: 0.0,
                width: 0.4,
                height: 1.0,
            }
            .into_ltrb(),
        )
}
