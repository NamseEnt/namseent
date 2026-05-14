use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FourLeafCloverUpgrade;

impl UpgradeBehavior for FourLeafCloverUpgrade {
    fn shorten_straight_flush_to_4_cards(&self) -> bool {
        true
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Four Leaf Clover",
            crate::l10n::locale::Language::Korean => "네잎클로버",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Enable straight flush with 4 cards",
            crate::l10n::locale::Language::Korean => {
                "스트레이트와 플러시를 4장으로 만들 수 있습니다"
            }
        });
    }
}

impl FourLeafCloverUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::FourLeafClover(FourLeafCloverUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    FourLeafCloverUpgrade::into_upgrade()
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.shorten_straight_flush_to_4_cards() as usize,
        1,
    ))
}
