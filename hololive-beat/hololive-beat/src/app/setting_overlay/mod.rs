mod slider;
mod volume_setting;

use self::volume_setting::VolumeSetting;
use super::play_state::resume_game;
use namui::{prelude::*, time::since_start};
use namui_prebuilt::{button::TextButtonFit, simple_rect, table::hooks::*};

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
                ratio(3, |wh, ctx| {
                    ctx.add(VolumeSetting { wh });
                }),
                ratio(2, |wh, ctx| {
                    ctx.add(Buttons {
                        wh,
                        on_click: &|_| {
                            close_setting_overlay();
                            resume_game(now);
                        },
                    });
                }),
            ])(wh, ctx);
        });

        ctx.component(simple_rect(
            wh,
            Color::TRANSPARENT,
            0.px(),
            Color::from_u8(0, 0, 0, 128),
        ));

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
struct Buttons<'a> {
    wh: Wh<Px>,
    on_click: &'a dyn Fn(MouseEvent),
}
impl Component for Buttons<'_> {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh, on_click } = self;
        const BUTTON_HEIGHT: Px = px(160.0);
        const PADDING: Px = px(16.0);

        ctx.compose(|ctx| {
            vertical([
                ratio(1, |_, _| {}),
                fixed(
                    BUTTON_HEIGHT,
                    horizontal([
                        ratio(1, |_, _| {}),
                        fit(
                            FitAlign::CenterMiddle,
                            TextButtonFit {
                                height: BUTTON_HEIGHT,
                                text: "Ok",
                                text_color: Color::BLACK,
                                stroke_color: Color::TRANSPARENT,
                                stroke_width: 0.px(),
                                fill_color: Color::grayscale_u8(128),
                                side_padding: PADDING,
                                mouse_buttons: [MouseButton::Left].to_vec(),
                                on_mouse_up_in: &on_click,
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
