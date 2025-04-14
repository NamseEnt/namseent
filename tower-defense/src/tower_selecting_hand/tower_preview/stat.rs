use crate::theme::{
    palette,
    typography::{FontSize, Paragraph, TextAlign},
};
use namui::*;
use namui_prebuilt::simple_rect;

const TOOLTIP_MAX_WIDTH: Px = px(256.);
const PADDING: Px = px(8.);

#[derive(Clone, PartialEq)]
pub(super) struct StatPreview<'a> {
    pub stat_name: &'static str,
    pub default_stat: f32,
    pub plus_stat: f32,
    pub multiplier: f32,
    pub wh: Wh<Px>,
    pub upgrade_texts: &'a [String],
}
impl Component for StatPreview<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            stat_name,
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

        ctx.add(Paragraph {
            text: format!("{stat_name}: "),
            font_size: FontSize::Medium,
            text_align: TextAlign::LeftTop,
            max_width: None,
        });
        ctx.add(Paragraph {
            text: format_stat(default_stat, plus_stat, multiplier),
            font_size: FontSize::Medium,
            text_align: TextAlign::RightTop { width: wh.width },
            max_width: None,
        });

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

fn format_stat(base: f32, plus: f32, multiplier: f32) -> String {
    let has_plus = plus != 0.0;
    let has_multiplier = multiplier != 1.0;

    match (has_plus, has_multiplier) {
        (true, true) => format!(
            "{:.1} (({:.1}+{:.1})*{})",
            base * multiplier + plus,
            base,
            plus,
            multiplier
        ),
        (true, false) => format!("{:.1} ({:.1}+{:.1})", base + plus, base, plus),
        (false, true) => format!("{:.1} ({:.1}*{:.1})", base * multiplier, base, multiplier),
        (false, false) => format!("{:.1}", base),
    }
}

struct Tooltip<'a> {
    upgrade_texts: &'a [String],
    max_width: Px,
}
impl Component for Tooltip<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Tooltip {
            upgrade_texts,
            max_width,
        } = self;

        let text_max_width = max_width - (PADDING * 2.0);
        let content = ctx.ghost_compose("tooltip-contents", |mut ctx| {
            for (index, upgrade_text) in upgrade_texts.iter().enumerate() {
                let rendered_text = ctx.ghost_add(
                    format!("tooltip-content-{index}"),
                    Paragraph {
                        text: upgrade_text.clone(),
                        font_size: FontSize::Medium,
                        text_align: TextAlign::LeftTop,
                        max_width: Some(text_max_width),
                    },
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
