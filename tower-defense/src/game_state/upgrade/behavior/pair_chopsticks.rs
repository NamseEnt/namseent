use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PairChopsticksUpgrade {
    pub damage_bonus_pct: f32,
}

impl UpgradeBehavior for PairChopsticksUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::PAIR_CHOPSTICK,
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
        Some(crate::thumbnail::render_right_bottom_overlay(
            width_height,
            &format!("+{:.0}%", self.damage_bonus_pct * 100.0),
            crate::theme::palette::RED,
        ))
    }

    fn is_applicable(&self, context: &SelectedTowerContext) -> bool {
        context.rank.is_some_and(|rank| rank.is_even())
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::PairChopsticks(upgrade) = &mut upgrade.upgrade {
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

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::EvenOdd { even: true },
            self.damage_bonus_pct,
        ))
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Pair Chopsticks",
            crate::l10n::locale::Language::Korean => "젓가락 세트",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Even-card tower ").with_icon_bold(
                    crate::icon::IconKind::Damage,
                    format!("+{:.0}%", self.damage_bonus_pct * 100.0),
                )
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("짝수 카드 타워 ").with_icon_bold(
                    crate::icon::IconKind::Damage,
                    format!("+{:.0}%", self.damage_bonus_pct * 100.0),
                )
            }
        };
    }
}

impl PairChopsticksUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::PairChopsticks(PairChopsticksUpgrade { damage_bonus_pct })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    PairChopsticksUpgrade::into_upgrade(0.4)
}
