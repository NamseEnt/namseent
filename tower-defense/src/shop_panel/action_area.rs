use crate::game_state::use_game_state;
use crate::shop_panel::constants::*;
use crate::theme::button::{Button, ButtonColor, ButtonVariant};
use crate::theme::palette;
use crate::theme::paper_container::{PaperContainerBackground, PaperTexture, PaperVariant};
use crate::theme::typography::memoized_text;
use namui::*;
use namui_prebuilt::table;

pub(super) struct ShopActionArea {
    pub wh: Wh<Px>,
}

impl Component for ShopActionArea {
    fn render(self, ctx: &RenderCtx) {
        let Self { wh } = self;
        let _game_state = use_game_state(ctx);

        let start_selecting = || {
            crate::mutate_game_state(|state| {
                state.action(crate::game_state::GameStateAction::StartSelectingTower);
            });
        };

        ctx.compose(|ctx| {
            table::padding_no_clip(
                INNER_PADDING,
                table::vertical([
                    table::fixed_no_clip(ACTION_HEIGHT, |wh, ctx| {
                        ctx.add(super::refresh_button::RefreshButton::new(wh));
                    }),
                    table::fixed_no_clip(ACTION_GAP, |_, _| {}),
                    table::fixed_no_clip(ACTION_HEIGHT, |wh, ctx| {
                        ctx.add(
                            Button::new(wh, &start_selecting, &|wh, _text_color, ctx| {
                                ctx.add(memoized_text(&(), |mut builder| {
                                    builder
                                        .headline()
                                        .bold()
                                        .color(palette::WHITE)
                                        .stroke(2.px(), palette::DARK_CHARCOAL)
                                        .text("START")
                                        .render_center(wh)
                                }));
                            })
                            .variant(ButtonVariant::Contained)
                            .color(ButtonColor::Primary),
                        );
                    }),
                ]),
            )(wh, ctx);
        });

        ctx.add(PaperContainerBackground {
            width: wh.width,
            height: wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE_CONTAINER_LOW,
            outline_color: None,
            shadow: true,
            arrow: None,
        });
    }
}
