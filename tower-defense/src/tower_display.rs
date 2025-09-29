use crate::game_state::GameState;
use crate::game_state::tower::Tower;
use crate::game_state::upgrade::TowerUpgradeState;
use crate::icon::{Icon, IconKind};
use crate::theme::typography::{FontSize, TextAlign, headline, paragraph};
use namui::*;

/// 타워의 기본 스탯을 표시하는 공통 컴포넌트
pub struct TowerStatDisplay {
    pub wh: Wh<Px>,
    pub damage: f32,
    pub attack_speed: f32,
    pub range: f32,
    pub tower_name: String,
}

impl Component for TowerStatDisplay {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            damage,
            attack_speed,
            range,
            tower_name,
        } = self;

        const STAT_HEIGHT: Px = px(20.0);
        const PADDING: Px = px(8.0);

        ctx.compose(|ctx| {
            let mut y_offset = PADDING;

            // Title
            ctx.translate((PADDING, y_offset)).add(
                headline(tower_name)
                    .size(FontSize::Medium)
                    .align(TextAlign::LeftTop)
                    .max_width(wh.width - PADDING * 2.0)
                    .build_rich(),
            );
            y_offset += px(24.0);

            // Damage
            ctx.translate((PADDING, y_offset)).compose(|ctx| {
                ctx.add(
                    Icon::new(IconKind::AttackDamage)
                        .size(crate::icon::IconSize::Small)
                        .wh(Wh::single(crate::icon::IconSize::Small.px())),
                );
                ctx.translate((px(20.0), 0.px())).add(
                    paragraph(format!("데미지: {damage:.1}"))
                        .size(FontSize::Small)
                        .align(TextAlign::LeftTop)
                        .max_width(wh.width - PADDING * 2.0 - px(20.0))
                        .build(),
                );
            });
            y_offset += STAT_HEIGHT;

            // Attack Speed
            ctx.translate((PADDING, y_offset)).compose(|ctx| {
                ctx.add(
                    Icon::new(IconKind::AttackSpeed)
                        .size(crate::icon::IconSize::Small)
                        .wh(Wh::single(crate::icon::IconSize::Small.px())),
                );
                ctx.translate((px(20.0), 0.px())).add(
                    paragraph(format!("속도: {:.2}/s", attack_speed))
                        .size(FontSize::Small)
                        .align(TextAlign::LeftTop)
                        .max_width(wh.width - PADDING * 2.0 - px(20.0))
                        .build(),
                );
            });
            y_offset += STAT_HEIGHT;

            // Range
            ctx.translate((PADDING, y_offset)).compose(|ctx| {
                ctx.add(
                    Icon::new(IconKind::AttackRange)
                        .size(crate::icon::IconSize::Small)
                        .wh(Wh::single(crate::icon::IconSize::Small.px())),
                );
                ctx.translate((px(20.0), 0.px())).add(
                    paragraph(format!("사정거리: {range:.1}"))
                        .size(FontSize::Small)
                        .align(TextAlign::LeftTop)
                        .max_width(wh.width - PADDING * 2.0 - px(20.0))
                        .build(),
                );
            });
        });
    }
}

/// Tower 인스턴스로부터 TowerStatDisplay를 생성하는 헬퍼 함수
pub fn tower_stat_display_from_tower(
    tower: &Tower,
    tower_upgrades: &[TowerUpgradeState],
    game_state: &GameState,
    wh: Wh<Px>,
) -> TowerStatDisplay {
    let damage = tower.calculate_projectile_damage(tower_upgrades, 1.0);
    let attack_speed = 1.0 / tower.shoot_interval.as_secs_f32();
    let range = tower.attack_range_radius(
        tower_upgrades,
        game_state.stage_modifiers.get_range_multiplier(),
    );
    let tower_name = format!("{} {}", tower.suit, tower.rank);

    TowerStatDisplay {
        wh,
        damage,
        attack_speed,
        range,
        tower_name,
    }
}
