mod slider;
mod volume_setting;

use self::volume_setting::VolumeSetting;
use super::{components::Backdrop, play_state::resume_game};
use crate::app::components::FilledButton;
use namui::{prelude::*, time::since_start};
use namui_prebuilt::table::hooks::*;

pub static SETTING_OVERLAY_OPEN_ATOM: Atom<bool> = Atom::uninitialized_new();

#[component]
pub struct SettingOverlay {
    pub wh: Wh<Px>,
}
impl Component for SettingOverlay {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;

        let (open, _) = ctx.atom_init(&SETTING_OVERLAY_OPEN_ATOM, || false);

        ctx.compose(|ctx| {
            if !*open {
                return;
            }
            ctx.add(Opened { wh });
        });

        ctx.done()
    }
}

#[component]
struct Opened {
    wh: Wh<Px>,
}
impl Component for Opened {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
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

        ctx.component(Backdrop { wh });

        ctx.done()
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
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, now } = self;

        const BUTTON_WH: Wh<Px> = Wh {
            width: px(192.0),
            height: px(96.0),
        };

        let (ok_button_focused, set_ok_button_focused) = ctx.state(|| false);

        ctx.compose(|ctx| {
            vertical([
                ratio_no_clip(1, |_, _| {}),
                fixed_no_clip(
                    BUTTON_WH.height,
                    horizontal([
                        ratio_no_clip(1, |_, _| {}),
                        fixed_no_clip(BUTTON_WH.width, |_, ctx| {
                            ctx.add(
                                FilledButton {
                                    wh: BUTTON_WH,
                                    text: "Ok".to_string(),
                                    on_click: &|| {
                                        close_setting_overlay();
                                        resume_game(now);
                                    },
                                    focused: *ok_button_focused,
                                }
                                .attach_event(|event| {
                                    let Event::MouseMove { event } = event else {
                                        return;
                                    };
                                    let should_focus = event.is_local_xy_in();
                                    if *ok_button_focused == should_focus {
                                        return;
                                    }
                                    set_ok_button_focused.set(should_focus);
                                }),
                            );
                        }),
                        ratio(1, |_, _| {}),
                    ]),
                ),
                ratio(1, |_, _| {}),
            ])(wh, ctx);
        });

        ctx.done()
    }
}
