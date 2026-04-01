use crate::l10n::{self, Locale};
use crate::theme::palette;
use crate::theme::paper_container::{
    ArrowSide, PaperArrow, PaperContainerBackground, PaperTexture, PaperVariant,
};
use crate::theme::typography::memoized_text;
use namui::*;

pub struct RerollHealthCostWarningTooltip {
    pub health_cost: usize,
    pub locale: Locale,
}

impl Component for RerollHealthCostWarningTooltip {
    fn render(self, ctx: &RenderCtx) {
        let RerollHealthCostWarningTooltip {
            health_cost,
            locale,
        } = self;

        let detail_text = ctx.ghost_add(
            "reroll-tooltip-detail-text",
            memoized_text((&locale, &health_cost), |mut builder| {
                builder
                    .paragraph()
                    .color(palette::ON_SURFACE)
                    .icon(crate::icon::IconKind::Warning)
                    .space()
                    .l10n(
                        l10n::ui::RerollHealthCostDetailText::Damage(health_cost),
                        &locale,
                    )
                    .render_left_top()
            }),
        );

        let detail_wh = detail_text
            .bounding_box()
            .map(|rect| rect.wh())
            .unwrap_or(Wh::new(0.px(), 0.px()));

        let padding = 8.px();
        let content_width = detail_wh.width;
        let content_height = detail_wh.height;

        let container_wh = Wh::new(
            content_width + padding * 2.0,
            content_height + padding * 2.0,
        );

        ctx.translate(Xy::single(padding)).add(detail_text);

        ctx.add(PaperContainerBackground {
            width: container_wh.width,
            height: container_wh.height,
            texture: PaperTexture::Rough,
            variant: PaperVariant::Sticky,
            color: palette::SURFACE,
            shadow: true,
            arrow: Some(PaperArrow {
                side: ArrowSide::Left,
                width: 8.px(),
                height: 16.px(),
                offset: container_wh.height / 2.0,
            }),
        });
    }
}
