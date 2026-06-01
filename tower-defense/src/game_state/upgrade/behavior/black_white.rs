use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BlackWhiteUpgrade;

impl UpgradeBehavior for BlackWhiteUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::BLACK_WHITE,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
    }

    fn treat_suits_as_same(&self) -> bool {
        true
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Black & White",
            crate::l10n::locale::Language::Korean => "흑백",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Treat all suits as one",
            crate::l10n::locale::Language::Korean => {
                "하트와 다이아를, 클럽과 스페이드를 같은 문양으로 간주합니다"
            }
        });
    }
}

impl BlackWhiteUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::BlackWhite(BlackWhiteUpgrade)
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BlackWhiteUpgrade::into_upgrade()
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((upgrade_state.treat_suits_as_same() as usize, 1))
}
