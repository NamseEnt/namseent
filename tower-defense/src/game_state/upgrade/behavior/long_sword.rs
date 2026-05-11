use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct LongSwordUpgrade {
    pub damage_bonus_pct: f32,
}

impl UpgradeBehavior for LongSwordUpgrade {
    fn is_tower_damage_upgrade(&self) -> bool {
        true
    }

    fn on_upgrade_acquired(self, game_state: &mut GameState) -> UpgradeUpdateFlags
    where
        Self: Sized,
    {
        merge_for_acquire(game_state, self.into())
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        Some((
            TowerUpgradeTarget::Suit {
                suit: crate::card::Suit::Spades,
            },
            self.damage_bonus_pct,
        ))
    }

    fn on_upgrade_acquired_effect(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::TOWER_STATS
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Long Sword",
            crate::l10n::locale::Language::Korean => "롱소드",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => {
                builder.static_text("Spade tower ").with_icon_bold(
                    crate::icon::IconKind::Damage,
                    format!("+{:.0}%", self.damage_bonus_pct * 100.0),
                )
            }
            crate::l10n::locale::Language::Korean => {
                builder.static_text("스페이드 타워 ").with_icon_bold(
                    crate::icon::IconKind::Damage,
                    format!("+{:.0}%", self.damage_bonus_pct * 100.0),
                )
            }
        };
    }
}

impl LongSwordUpgrade {
    pub fn into_upgrade(damage_bonus_pct: f32) -> Upgrade {
        Upgrade::LongSword(LongSwordUpgrade { damage_bonus_pct })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, no_current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    LongSwordUpgrade::into_upgrade(0.5)
}
