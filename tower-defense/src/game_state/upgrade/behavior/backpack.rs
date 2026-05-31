use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BackpackUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for BackpackUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::BACKPACK,
            width_height,
            UPGRADE_STICKER_THUMBNAIL_STROKE,
            shadow,
        )
        }

    fn shop_slot_expand(&self) -> usize {
        self.add
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::locale::Language::English => "Backpack",
            crate::l10n::locale::Language::Korean => "배낭",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::locale::Language::English => builder
                .static_text("Shop slot ")
                .with_icon_bold(crate::icon::IconKind::Shop, format!("+{}", self.add)),
            crate::l10n::locale::Language::Korean => {
                builder.with_icon_bold(crate::icon::IconKind::Shop, "상점 슬롯 +1")
            }
        };
    }
}

impl BackpackUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Backpack(BackpackUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition =
    UpgradeDefinition::new(generate_upgrade, current_and_max);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    BackpackUpgrade::into_upgrade(1)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.shop_slot_expand(),
        super::MAX_SHOP_SLOT_EXPAND,
    ))
}
