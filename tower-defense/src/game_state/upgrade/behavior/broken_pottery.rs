use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrokenPotteryUpgrade {
    pub damage_bonus_pct: f32,
}

impl UpgradeBehavior for BrokenPotteryUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::BROKEN_POTTERY,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn thumbnail_overlay(
        &self,
        width_height: Wh<Px>,
        _game_state: &GameState,
    ) -> Option<RenderingTree> {
        let text = format!("{:.0}%", self.damage_bonus_pct * 100.0);

        Some(crate::thumbnail::render_right_bottom_overlay(
            width_height,
            &text,
            crate::theme::palette::RED,
        ))
    }

    fn is_applicable(&self, context: &SelectedTowerContext) -> bool {
        context.rerolled_count.is_some_and(|count| count > 0)
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        Some((TowerUpgradeTarget::RerolledTower, self.damage_bonus_pct))
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::BrokenPottery(upgrade) = &mut upgrade.upgrade {
                upgrade.damage_bonus_pct += self.damage_bonus_pct;
                return UpgradeUpdateFlags::TOWER_STATS;
            }
        }

        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Broken Pottery",
            crate::l10n::locale::Language::Korean => "깨진 도자기",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Every card rerolled tower ")
                .with_damage_value(format!("damage +{:.0}%", self.damage_bonus_pct * 100.0)),
            crate::l10n::locale::Language::Korean => builder
                .static_text("카드 리롤 시 타워 ")
                .with_damage_value(format!("데미지 +{:.0}%", self.damage_bonus_pct * 100.0)),
        };
    }
}

impl BrokenPotteryUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::BrokenPottery(BrokenPotteryUpgrade { damage_bonus_pct })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::common_rarity,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BrokenPotteryUpgrade::into_upgrade(0.25)
}
