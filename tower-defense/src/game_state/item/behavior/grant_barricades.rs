use super::*;

use crate::icon::IconKind;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct GrantBarricadesItem {
    pub count: usize,
}

impl GrantBarricadesItem {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    pub fn standard() -> Self {
        Self::new(4)
    }

    pub fn into_item(self) -> Item {
        Item::GrantBarricades(self)
    }
}

impl ItemBehavior for GrantBarricadesItem {
    fn key(&self) -> &'static str {
        "grant_barricades"
    }

    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        for _ in 0..self.count {
            game_state.action(crate::game_state::GameStateAction::GrantTowerCard {
                tower_kind: crate::game_state::tower::TowerKind::Barricade,
                suit: None,
                rank: None,
            });
        }
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        builder.static_text(match locale.language {
            crate::l10n::Language::Korean => "바리케이드",
            crate::l10n::Language::English => "Barricades",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::Language::Korean => {
                builder
                    .with_icon_bold(IconKind::Card, format!("{}", self.count))
                    .static_text(" 바리케이드 타워");
            }
            crate::l10n::Language::English => {
                builder
                    .with_icon_bold(IconKind::Card, format!("{}", self.count))
                    .static_text(" barricade towers");
            }
        }
    }

    fn thumbnail_with_shadow(
        &self,
        width_height: Wh<Px>,
        stroke_px: Px,
        shadow: bool,
    ) -> RenderingTree {
        render_sticker(
            crate::asset::image::thumbnail::GRANT_BARRICADES,
            width_height,
            stroke_px,
            shadow,
        )
    }
}
