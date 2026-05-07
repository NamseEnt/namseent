use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CatUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for CatUpgrade {
    fn gold_earn_plus(&self) -> usize {
        self.add
    }

    fn l10n_name<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Cat",
            crate::l10n::locale::Language::Korean => "고양이",
        });
    }

    fn l10n_description<'a>(&self, builder: &mut crate::theme::typography::TypographyBuilder<'a>, locale: &crate::l10n::Locale) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Gain ")
                .with_icon_bold(crate::icon::IconKind::Gold, format!("+{}", self.add))
                .static_text(" on monster kills"),
            crate::l10n::locale::Language::Korean => builder
                .static_text("몬스터 처치 시 ")
                .with_icon_bold(crate::icon::IconKind::Gold, format!("{}", self.add)),
        };
    }
}

impl CatUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Cat(CatUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(upgrade_state: &UpgradeState) -> Upgrade {
    CatUpgrade::into_upgrade(next_cat_add(upgrade_state.gold_earn_plus()))
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.gold_earn_plus(), super::MAX_GOLD_EARN_PLUS))
}

fn next_cat_add(gold_earn_plus: usize) -> usize {
    match gold_earn_plus {
        0 | 1 => 1,
        2 => 2,
        4 => 4,
        8 => 8,
        _ => 0,
    }
}

