use namui::*;
use namui_prebuilt::button::TextButton;
use std::cell::RefCell;

register_assets!();

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(MediaExample {});
    })
}

thread_local! {
    static PLAY_HANDLE: RefCell<Option<system::audio::PlayHandle>> = const { RefCell::new(None) };
}

struct MediaExample;

impl Component for MediaExample {
    fn render(self, ctx: &RenderCtx) {
        let (is_playing, set_is_playing) = ctx.state(|| false);

        let duration_text = format!("duration: {:.2}s", asset::AUDIO.duration().as_secs_f32());
        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 20.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: &duration_text,
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![],
            on_mouse_up_in: |_| {},
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 50.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: "play (one-shot)",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: |_| {
                PLAY_HANDLE.with(|h| *h.borrow_mut() = Some(asset::AUDIO.play()));
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 90.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: if *is_playing {
                "stop (drop PlayHandle)"
            } else {
                "play (with PlayHandle)"
            },
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: |_| {
                if *is_playing {
                    PLAY_HANDLE.with(|h| h.borrow_mut().take());
                    set_is_playing.set(false);
                } else {
                    PLAY_HANDLE.with(|h| *h.borrow_mut() = Some(asset::AUDIO.play()));
                    set_is_playing.set(true);
                }
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 10.px(),
                y: 130.px(),
                width: 200.px(),
                height: 20.px(),
            },
            text: "play repeat (click above to stop)",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: |_| {
                PLAY_HANDLE.with(|h| *h.borrow_mut() = Some(asset::AUDIO.play_repeat()));
                set_is_playing.set(true);
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 20.px(),
                y: 250.px(),
                width: 40.px(),
                height: 20.px(),
            },
            text: "vol -",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: |_| {
                system::audio::set_volume((system::audio::volume() - 0.05).max(0.0));
            },
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 60.px(),
                y: 250.px(),
                width: 40.px(),
                height: 20.px(),
            },
            text: &(system::audio::volume() * 100.0).floor().to_string(),
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![],
            on_mouse_up_in: |_| {},
        });

        ctx.add(TextButton {
            rect: Rect::Xywh {
                x: 100.px(),
                y: 250.px(),
                width: 40.px(),
                height: 20.px(),
            },
            text: "vol +",
            text_color: Color::BLACK,
            stroke_color: Color::BLACK,
            stroke_width: 1.px(),
            fill_color: Color::TRANSPARENT,
            mouse_buttons: vec![MouseButton::Left],
            on_mouse_up_in: |_| {
                system::audio::set_volume((system::audio::volume() + 0.05).min(1.0));
            },
        });
    }
}
