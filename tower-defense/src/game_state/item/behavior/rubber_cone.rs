use super::*;
use crate::l10n::rich_text_helpers::RichTextHelpers;

#[derive(Debug, Clone, Copy, PartialEq, Eq, State)]
pub struct RubberConeItem {
    pub count: usize,
}

impl RubberConeItem {
    pub fn new(count: usize) -> Self {
        Self { count }
    }

    pub fn standard() -> Self {
        Self::new(4)
    }

    pub fn into_item(self) -> Item {
        Item::RubberCone(self)
    }
}

impl ItemBehavior for RubberConeItem {
    fn key(&self) -> &'static str {
        "rubber_cone"
    }

    fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        for _ in 0..self.count {
            game_state.action(crate::game_state::GameStateAction::GrantTowerCard {
                tower_kind: crate::game_state::tower::TowerKind::RubberCone,
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
            crate::l10n::Language::Korean => "러버콘",
            crate::l10n::Language::English => "Rubber Cone",
        });
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        match locale.language {
            crate::l10n::Language::Korean => {
                builder.static_text("핸드에 ");
                builder
                    .with_bold( format!("{}장", self.count))
                    .static_text(" 러버콘 타워를 가져옵니다. 러버콘 타워는 공격 기능 없이 적들의 이동만 방해할 수 있는 타워입니다.");
            }
            crate::l10n::Language::English => {
                builder.static_text("Adds ");
                builder
                    .with_bold( format!("{}", self.count))
                    .static_text(" Rubber Cone towers to your hand. Rubber Cone towers cannot attack; they can only hinder enemy movement.");
            }
        }
    }

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_> {
        crate::thumbnail::ThumbnailSource::Image(crate::asset::image::thumbnail::RUBBER_CONE)
    }
}

pub(super) const DEFINITION: crate::game_state::item::definition::ItemDefinition =
    crate::game_state::item::definition::ItemDefinition::new(generate_rubber_cone_item, || {
        crate::Rarity::Rare
    });

fn generate_rubber_cone_item(_rng: &mut dyn rand::RngCore) -> Item {
    RubberConeItem::standard().into_item()
}
