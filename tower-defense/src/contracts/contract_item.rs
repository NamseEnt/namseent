use crate::{
    game_state::contract::{Contract, ContractId},
    hand::xy_with_spring,
    icon::{Icon, IconKind, IconSize},
    l10n::{TextManager, contract::ContractText},
    palette,
    theme::typography::{FontSize, HEADLINE_FONT_SIZE_LARGE, headline, paragraph},
};
use namui::*;
use namui_prebuilt::table;

const PADDING: Px = px(4.);

pub struct ContractItemContent<'a> {
    pub contract: &'a Contract,
    pub text_manager: TextManager,
    pub content_width: Px,
    pub evaluating_contract_id: Option<ContractId>,
}

impl Component for ContractItemContent<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            contract,
            text_manager,
            content_width,
            evaluating_contract_id,
        } = self;

        let eval_offset_x = if Some(contract.id) == evaluating_contract_id {
            px(32.0)
        } else {
            0.px()
        };

        let target_xy = Xy::new(eval_offset_x, 0.px());
        let animated_xy = xy_with_spring(ctx, target_xy, Xy::zero());

        let content = ctx.ghost_compose("inner", |ctx| {
            table::vertical([
                table::fixed(
                    HEADLINE_FONT_SIZE_LARGE.into_px(),
                    table::horizontal([
                        table::fixed(HEADLINE_FONT_SIZE_LARGE.into_px(), |wh, ctx| {
                            ctx.add(
                                Icon::new(IconKind::Rarity {
                                    rarity: contract.rarity,
                                })
                                .size(IconSize::Custom { size: wh.width })
                                .wh(wh),
                            );
                        }),
                        table::ratio(1, move |_, _| {}),
                        table::fixed(HEADLINE_FONT_SIZE_LARGE.into_px() * 2.0, |wh, ctx| {
                            ctx.add(
                                headline()
                                    .size(FontSize::Small)
                                    .text(contract.status.to_string())
                                    .render_center(wh),
                            );
                        }),
                    ]),
                ),
                table::fixed(PADDING * 2.0, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, move |compose_ctx| {
                    let text = text_manager.contract(ContractText::Risk(&contract.risk));
                    compose_ctx.add(
                        paragraph()
                            .size(FontSize::Medium)
                            .max_width(content_width)
                            .text(&text)
                            .render_left_top(),
                    );
                }),
                table::fixed(PADDING, |_, _| {}),
                table::fit(table::FitAlign::LeftTop, move |compose_ctx| {
                    let text = text_manager.contract(ContractText::Reward(&contract.reward));
                    compose_ctx.add(
                        paragraph()
                            .size(FontSize::Medium)
                            .max_width(content_width)
                            .text(&text)
                            .render_left_top(),
                    );
                }),
            ])(Wh::new(content_width, f32::MAX.px()), ctx);
        });
        let Some(content_wh) = content.bounding_box().map(|rect| rect.wh()) else {
            return;
        };
        let container_wh = content_wh + Wh::single(PADDING * 2.);

        ctx.translate(animated_xy + Xy::single(PADDING))
            .add(content);
        ctx.translate(animated_xy).add(rect(RectParam {
            rect: container_wh.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE_CONTAINER,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}
