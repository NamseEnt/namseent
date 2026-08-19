use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PerfectPotteryUpgrade {
    pub damage_bonus_pct: f32,
}

impl UpgradeBehavior for PerfectPotteryUpgrade {
    fn key(&self) -> &'static str {
        "perfect_pottery"
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::PERFECT_POTTERY)
    }

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        vec![crate::thumbnail::ThumbnailOverlay::right_bottom(
            format!("{:.0}%", self.damage_bonus_pct * 100.0),
            crate::theme::palette::RED,
        )]
    }

    fn is_applicable(&self, context: &SelectedTowerContext) -> bool {
        context.rerolled_count == Some(0)
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::PerfectPottery(upgrade) = &mut upgrade.upgrade {
                upgrade.damage_bonus_pct += self.damage_bonus_pct;
                return UpgradeUpdateFlags::TOWER_STATS | UpgradeUpdateFlags::REVISION;
            }
        }

        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::TOWER_STATS | UpgradeUpdateFlags::REVISION
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        Some((TowerUpgradeTarget::NoRerollTower, self.damage_bonus_pct))
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Perfect Pottery",
            crate::l10n::locale::Language::Korean => "완벽한 도자기",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("No-reroll tower ")
                .with_bold(format!("damage +{:.0}%", self.damage_bonus_pct * 100.0)),
            crate::l10n::locale::Language::Korean => builder
                .static_text("리롤 안한 타워 ")
                .with_bold(format!("데미지 +{:.0}%", self.damage_bonus_pct * 100.0)),
        };
    }
}

impl PerfectPotteryUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::PerfectPottery(PerfectPotteryUpgrade { damage_bonus_pct })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    no_current_and_max,
    UpgradeDefinition::rarity_common,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PerfectPotteryUpgrade::into_upgrade(0.5)
}
