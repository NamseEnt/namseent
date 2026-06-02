use super::*;

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EraserUpgrade {
    pub add: usize,
}

impl UpgradeBehavior for EraserUpgrade {
    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree {
        crate::thumbnail::render_sticker_image_with_shadow(
            crate::asset::image::thumbnail::ERASER,
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
            &format!("{}", self.add),
            crate::theme::palette::WHITE,
        ))
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        for upgrade in game_state.upgrade_state.upgrades.iter_mut() {
            if let Upgrade::Eraser(upgrade) = &mut upgrade.upgrade {
                upgrade.add += self.add;
                return UpgradeUpdateFlags::NONE;
            }
        }

        game_state
            .upgrade_state
            .upgrades
            .push(Upgrade::from(self).with_unique_id());
        UpgradeUpdateFlags::NONE
    }

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
            crate::l10n::locale::Language::English => {
                builder.text(format!("Remove {} rank from the deck", self.add))
            }
            crate::l10n::locale::Language::Korean => {
                builder.text(format!("덱에서 {}개 숫자를 제거합니다", self.add))
            }
        };
    }
}

impl EraserUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Eraser(EraserUpgrade { add })
    }
}

pub(super) const UPGRADE_DEFINITION: UpgradeDefinition = UpgradeDefinition::new(
    generate_upgrade,
    current_and_max,
    UpgradeDefinition::rarity_rare,
);

fn generate_upgrade(_upgrade_state: &UpgradeState) -> Upgrade {
    EraserUpgrade::into_upgrade(1)
}

fn current_and_max(upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    Some((
        upgrade_state.removed_number_rank_count(),
        super::MAX_REMOVE_NUMBER_RANKS,
    ))
}
