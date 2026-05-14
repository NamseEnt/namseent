use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct RabbitUpgrade;

impl UpgradeBehavior for RabbitUpgrade {
    fn skip_rank_for_straight(&self) -> bool {
        true
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Rabbit",
            crate::l10n::locale::Language::Korean => "토끼",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Skip one rank in a straight",
            crate::l10n::locale::Language::Korean => {
                "스트레이트를 만들 때 하나를 건너뛸 수 있습니다"
            }
        });
    }
}

impl RabbitUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Rabbit(RabbitUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    RabbitUpgrade::into_upgrade()
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.skip_rank_for_straight() as usize, 1))
}
