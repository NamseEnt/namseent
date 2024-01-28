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

        ctx.compose(|ctx| {
            vertical([
                ratio(1, |_, _| {}),
                fixed(
                    BUTTON_WH.height,
                    horizontal([
                        ratio(1, |_, _| {}),
                        fit(
                            FitAlign::CenterMiddle,
                            FilledButton {
                                wh: BUTTON_WH,
                                text: "Ok".to_string(),
                                on_click: &|| {
                                    close_setting_overlay();
                                    resume_game(now);
                                },
                            },
                        ),
                        ratio(1, |_, _| {}),
                    ]),
                ),
                ratio(1, |_, _| {}),
            ])(wh, ctx);
        });

        ctx.done()
    }
}
