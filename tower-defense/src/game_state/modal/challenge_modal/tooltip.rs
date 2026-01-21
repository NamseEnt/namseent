use crate::game_state::use_game_state;
use crate::icon::{Icon, IconKind, IconSize};
use crate::theme::{palette, typography};
use namui::*;

pub struct ChallengeMonsterTooltip<'a> {
    pub template: &'a crate::game_state::monster::MonsterTemplate,
}

impl<'a> Component for ChallengeMonsterTooltip<'a> {
    fn render(self, ctx: &RenderCtx) {
        let Self { template } = self;
        let game_state = use_game_state(ctx);
        let monster_kind = template.kind;

        const PADDING: Px = px(8.);
        const IMAGE_SIZE: Px = px(96.);
        const TEXT_PADDING: Px = px(4.);
        const TOOLTIP_WIDTH: Px = px(200.);
        const CONTENT_WIDTH: Px = px(184.); // TOOLTIP_WIDTH - (PADDING * 2)

        let content = ctx.ghost_compose("tooltip-contents", |mut ctx| {
            // 생김새 - 이미지만 96px 크기로 상단 중앙에 표시
            let image = monster_kind.image();
            let image_wh = Wh::single(IMAGE_SIZE);
            let image_offset = (CONTENT_WIDTH - IMAGE_SIZE) / 2.0;
            ctx.translate((image_offset, 0.px()))
                .add(namui::image(ImageParam {
                    rect: image_wh.to_rect(),
                    image,
                    style: ImageStyle {
                        fit: ImageFit::Contain,
                        paint: None,
                    },
                }));
            ctx = ctx.translate((0.px(), IMAGE_SIZE + TEXT_PADDING));

            // 체력 - 아이콘과 함께 표시
            let hp_text = ctx.ghost_add(
                "hp",
                typography::paragraph(format!("{:.0}", template.max_hp))
                    .size(typography::FontSize::Medium)
                    .align(typography::TextAlign::LeftTop)
                    .max_width(CONTENT_WIDTH)
                    .build_rich(),
            );
            let hp_height = hp_text
                .bounding_box()
                .map(|rect| rect.height())
                .unwrap_or_default();

            // HP 아이콘
            ctx.add(
                Icon::new(IconKind::Health)
                    .size(IconSize::Small)
                    .wh(Wh::new(16.px(), hp_height)),
            );
            ctx = ctx.translate((20.px(), 0.px()));
            ctx.add(hp_text);
            ctx = ctx.translate((-20.px(), hp_height + TEXT_PADDING));

            // 스킬
            let skill_descriptions = template.skill_descriptions(&game_state.locale);
            if !skill_descriptions.is_empty() {
                for skill_desc in skill_descriptions.iter() {
                    let skill_text = ctx.ghost_add(
                        "skill",
                        typography::paragraph(skill_desc.clone())
                            .size(typography::FontSize::Medium)
                            .align(typography::TextAlign::LeftTop)
                            .max_width(CONTENT_WIDTH)
                            .build_rich(),
                    );
                    let skill_height = skill_text
                        .bounding_box()
                        .map(|rect| rect.height())
                        .unwrap_or_default();

                    // 스킬 아이콘 (Invincible)
                    ctx.add(
                        Icon::new(IconKind::Invincible)
                            .size(IconSize::Small)
                            .wh(Wh::new(16.px(), skill_height)),
                    );
                    ctx = ctx.translate((20.px(), 0.px()));
                    ctx.add(skill_text);
                    ctx = ctx.translate((-20.px(), skill_height + TEXT_PADDING));
                }
            }

            // 리워드 - 골드 아이콘과 함께 표시
            let reward_text = ctx.ghost_add(
                "reward",
                typography::paragraph(format!("{}", template.reward))
                    .size(typography::FontSize::Medium)
                    .align(typography::TextAlign::LeftTop)
                    .max_width(CONTENT_WIDTH)
                    .build_rich(),
            );
            let reward_height = reward_text
                .bounding_box()
                .map(|rect| rect.height())
                .unwrap_or_default();

            // 골드 아이콘
            ctx.add(
                Icon::new(IconKind::Gold)
                    .size(IconSize::Small)
                    .wh(Wh::new(16.px(), reward_height)),
            );
            ctx.translate((20.px(), 0.px())).add(reward_text);
        });

        let Some(content_wh) = content.bounding_box().map(|rect| rect.wh()) else {
            return;
        };

        if content_wh.height == 0.px() {
            return;
        }

        let container_wh = Wh::new(TOOLTIP_WIDTH, content_wh.height + (PADDING * 2.0));

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
