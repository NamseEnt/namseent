mod slider;
mod volume_setting;

use self::volume_setting::VolumeSetting;
use super::{components::Backdrop, play_state::resume_game};
use crate::app::{
    components::FilledButton,
    play_state::{restart_game, PlayState, PLAY_STATE_ATOM},
};
use namui::{prelude::*, time::since_start};
use namui_prebuilt::table::hooks::*;

pub static SETTING_OVERLAY_OPEN_ATOM: Atom<bool> = Atom::uninitialized();

#[component]
pub struct SettingOverlay {
    pub wh: Wh<Px>,
}
impl Component for SettingOverlay {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let (open, _) = ctx.init_atom(&SETTING_OVERLAY_OPEN_ATOM, || false);

        ctx.compose(|ctx| {
            if !*open {
                return;
            }
            ctx.add(Opened { wh });
        });
    }
}

#[component]
struct Opened {
    wh: Wh<Px>,
}
impl Component for Opened {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;

        let now = since_start();

        ctx.compose(|ctx| {
            vertical([
                ratio(1, |_, _| {}),
                ratio(3, |wh, ctx| {
                    ctx.add(VolumeSetting { wh });
                }),
                ratio(2, |wh, ctx| {
                    ctx.add(Buttons { wh, now });
                }),
                ratio(1, |_, _| {}),
            ])(wh, ctx);
        });

        ctx.add(Backdrop { wh });
    }
}

pub fn open_setting_overlay() {
    SETTING_OVERLAY_OPEN_ATOM.set(true);
}

pub fn close_setting_overlay() {
    SETTING_OVERLAY_OPEN_ATOM.set(false);
}

#[component]
struct Buttons {
    wh: Wh<Px>,
    now: Duration,
}
impl Component for Buttons {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh, now } = self;

        const BUTTON_WH: Wh<Px> = Wh {
            width: px(192.0),
            height: px(96.0),
        };
        const PADDING: Px = px(64.0);

        let (state, _set_state) = ctx.atom(&PLAY_STATE_ATOM);
        let (focusing_button, set_focusing_button) = ctx.state(|| FocusingButton::Ok);

        let playing = matches!(*state, PlayState::Loaded { .. });

        let on_music_button_clicked = &|| {
            close_setting_overlay();
            PLAY_STATE_ATOM.set(PlayState::Idle);
        };
        let on_ok_button_clicked = &|| {
            close_setting_overlay();
            resume_game(now);
        };
        let on_retry_button_clicked = &|| {
            close_setting_overlay();
            restart_game();
        };

        ctx.compose(|ctx| {
            vertical([
                ratio_no_clip(1, |_, _| {}),
                fixed_no_clip(
                    BUTTON_WH.height,
                    horizontal([
                        ratio_no_clip(1, |_, _| {}),
                        fixed_no_clip(BUTTON_WH.width, |wh, ctx| {
                            if !playing {
                                return;
                            }
                            ctx.add(FilledButton {
                                wh,
                                text: "Music".to_string(),
                                on_click: &on_music_button_clicked,
                                on_mouse_enter: &|| {
                                    set_focusing_button.set(FocusingButton::Music);
                                },
                                focused: *focusing_button == FocusingButton::Music,
                            });
                        }),
                        fixed_no_clip(PADDING, |_, _| {}),
                        fixed_no_clip(BUTTON_WH.width, |wh, ctx| {
                            ctx.add(FilledButton {
                                wh,
                                text: "Ok".to_string(),
                                on_click: &on_ok_button_clicked,
                                on_mouse_enter: &|| {
                                    set_focusing_button.set(FocusingButton::Ok);
                                },
                                focused: *focusing_button == FocusingButton::Ok,
                            });
                        }),
                        fixed_no_clip(PADDING, |_, _| {}),
                        fixed_no_clip(BUTTON_WH.width, |wh, ctx| {
                            if !playing {
                                return;
                            }
                            ctx.add(FilledButton {
                                wh,
                                text: "Retry".to_string(),
                                on_click: &on_retry_button_clicked,
                                on_mouse_enter: &|| {
                                    set_focusing_button.set(FocusingButton::Retry);
                                },
                                focused: *focusing_button == FocusingButton::Retry,
                            });
                        }),
                        ratio(1, |_, _| {}),
                    ]),
                ),
                ratio(1, |_, _| {}),
            ])(wh, ctx);
        });

        ctx.on_raw_event(|event| {
            let RawEvent::KeyDown { event } = event else {
                return;
            };
            match event.code {
                Code::Enter => match *focusing_button {
                    FocusingButton::Music => {
                        on_music_button_clicked();
                    }
                    FocusingButton::Ok => {
                        on_ok_button_clicked();
                    }
                    FocusingButton::Retry => {
                        on_retry_button_clicked();
                    }
                },
                Code::ArrowLeft => {
                    set_focusing_button.set(focusing_button.prev());
                }
                Code::ArrowRight => {
                    set_focusing_button.set(focusing_button.next());
                }
                _ => {}
            }
        });
    }
}

#[derive(Debug, PartialEq)]
enum FocusingButton {
    Music,
    Ok,
    Retry,
}
impl FocusingButton {
    fn next(&self) -> Self {
        match self {
            Self::Music => Self::Ok,
            Self::Ok => Self::Retry,
            Self::Retry => Self::Retry,
        }
    }
    fn prev(&self) -> Self {
        match self {
            Self::Music => Self::Music,
            Self::Ok => Self::Music,
            Self::Retry => Self::Ok,
        }
    }
}
