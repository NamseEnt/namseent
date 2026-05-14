use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EraserUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for EraserUpgrade {
    fn removed_number_rank_count(&self) -> usize {
        self.add
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Eraser",
            crate::l10n::locale::Language::Korean => "지우개",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Remove ")
                .with_positive_effect(format!("{} rank", self.add))
                .static_text(" from the deck"),
            crate::l10n::locale::Language::Korean => {
                builder.text(format!("덱에서 {}개 숫자카드를 제거합니다", self.add))
            }
        };
    }
}

impl EraserUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Eraser(EraserUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    EraserUpgrade::into_upgrade(1)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.removed_number_rank_count(),
        super::MAX_REMOVE_NUMBER_RANKS,
    ))
}
