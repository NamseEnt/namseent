use crate::l10n::rich_text_helpers::{additive_value, multiplier_value};
use crate::{
    icon::{Icon, IconKind, IconSize},
    theme::{
        palette,
        typography::{FontSize, TextAlign, paragraph},
    },
};
use namui::*;
use namui_prebuilt::simple_rect;
use std::fmt;

const TOOLTIP_MAX_WIDTH: Px = px(256.);
const PADDING: Px = px(8.);

#[derive(Clone, PartialEq)]
pub(super) struct StatPreview<'a> {
    pub stat_icon_kind: IconKind,
    pub default_stat: f32,
    pub plus_stat: f32,
    pub multiplier: f32,
    pub wh: Wh<Px>,
    pub upgrade_texts: &'a [String],
}
impl Component for StatPreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            stat_icon_kind,
            default_stat,
            plus_stat,
            multiplier,
            wh,
            upgrade_texts,
        } = self;

        let (mouse_hovering, set_mouse_hovering) = ctx.state::<bool>(|| false);

        ctx.compose(|ctx| {
            if !*mouse_hovering {
                return;
            }
            let tooltip = ctx.ghost_add(
                "tooltip",
                Tooltip {
                    stat_detail: format_stat_detail(default_stat, plus_stat, multiplier),
                    upgrade_texts,
                    max_width: TOOLTIP_MAX_WIDTH,
                },
            );
            let Some(tooltip_wh) = bounding_box(&tooltip).map(|rect| rect.wh()) else {
                return;
            };
            if tooltip_wh.height == 0.px() {
                return;
            }
            ctx.translate((wh.width, wh.height - tooltip_wh.height))
                .on_top()
                .add(tooltip);
        });

        ctx.add(
            Icon::new(stat_icon_kind)
                .size(IconSize::Small)
                .wh(Wh::new(16.px(), wh.height)),
        );
        ctx.add(
            paragraph(format_stat_final(default_stat, plus_stat, multiplier))
                .size(FontSize::Medium)
                .align(TextAlign::RightTop { width: wh.width })
                .build_rich(),
        );

        ctx.add(
            simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::TRANSPARENT).attach_event(|event| {
                let Event::MouseMove { event } = event else {
                    return;
                };

                if event.is_local_xy_in() {
                    set_mouse_hovering.set(true);
                } else {
                    set_mouse_hovering.set(false);
                }
            }),
        );
    }
}

fn format_stat_final(base: f32, plus: f32, multiplier: f32) -> String {
    let final_value = calculate_final_stat(base, plus, multiplier);
    format!("{final_value:.1}")
}

fn format_stat_detail(base: f32, plus: f32, multiplier: f32) -> String {
    let has_plus = plus != 0.0;
    let has_multiplier = multiplier != 1.0;

    match (has_plus, has_multiplier) {
        (true, true) => format!(
            "({:.1} {}) {} = {:.1}",
            base,
            additive_value(OneDecimal(plus)),
            multiplier_value(OneDecimal(multiplier)),
            calculate_final_stat(base, plus, multiplier)
        ),
        (true, false) => format!(
            "{:.1} {} = {:.1}",
            base,
            additive_value(OneDecimal(plus)),
            base + plus
        ),
        (false, true) => format!(
            "{:.1} {} = {:.1}",
            base,
            multiplier_value(OneDecimal(multiplier)),
            base * multiplier
        ),
        (false, false) => format!("{base:.1}"),
    }
}

struct OneDecimal(f32);
impl fmt::Display for OneDecimal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}", self.0)
    }
}

fn calculate_final_stat(base: f32, plus: f32, multiplier: f32) -> f32 {
    (base + plus) * multiplier
}

struct Tooltip<'a> {
    stat_detail: String,
    upgrade_texts: &'a [String],
    max_width: Px,
}
impl Component for Tooltip<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Tooltip {
            stat_detail,
            upgrade_texts,
            max_width,
        } = self;

        let text_max_width = max_width - (PADDING * 2.0);
        let content = ctx.ghost_compose("tooltip-contents", |mut ctx| {
            // 통계 상세 정보 렌더링
            let stat_text = ctx.ghost_add(
                "stat-detail",
                paragraph(stat_detail)
                    .size(FontSize::Medium)
                    .align(TextAlign::LeftTop)
                    .max_width(text_max_width)
                    .build_rich(),
            );
            let stat_text_height = bounding_box(&stat_text)
                .map(|rect| rect.height())
                .unwrap_or_default();
            ctx.add(stat_text);
            ctx = ctx.translate((0.px(), PADDING + stat_text_height));

            // 구분선 추가 (업그레이드 텍스트가 있는 경우)
            if !upgrade_texts.is_empty() {
                ctx = ctx.translate((0.px(), PADDING));
            }

            // 업그레이드 텍스트들 렌더링
            for (index, upgrade_text) in upgrade_texts.iter().enumerate() {
                let rendered_text = ctx.ghost_add(
                    format!("tooltip-content-{index}"),
                    paragraph(upgrade_text.clone())
                        .size(FontSize::Medium)
                        .align(TextAlign::LeftTop)
                        .max_width(text_max_width)
                        .build_rich(),
                );
                let text_height = bounding_box(&rendered_text)
                    .map(|rect| rect.height())
                    .unwrap_or_default();
                ctx.add(rendered_text);
                ctx = ctx.translate((0.px(), PADDING + text_height))
            }
        });

        let Some(content_wh) = bounding_box(&content).map(|rect| rect.wh()) else {
            return;
        };
        if content_wh.height == 0.px() {
            return;
        }
        let container_wh = Wh::new(
            content_wh.width + (PADDING * 2.0),
            content_wh.height + (PADDING * 2.0),
        );

        ctx.translate((PADDING, PADDING)).add(content);

        ctx.add(rect(RectParam {
            rect: container_wh.to_rect(),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: palette::OUTLINE,
                    width: 1.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill {
                    color: palette::SURFACE,
                }),
                round: Some(RectRound {
                    radius: palette::ROUND,
                }),
            },
        }));
    }
}
