pub mod generation;
mod thumbnail;
mod usage;

use crate::asset::image::thumbnail as thumbnail_image;
use crate::card::Card;
pub use crate::game_state::effect::Effect;
use crate::thumbnail::{
    STICKER_THUMBNAIL_STROKE, render_card_thumbnail, render_sticker_image_with_shadow,
};
use namui::*;
pub use usage::*;

#[derive(Debug, Clone, PartialEq, State)]
pub enum ItemKind {
    RiceBall,
    LumpSugar,
    Shield,
    Painkiller,
    GrantBarricades,
    GrantCard { card: Card },
}

#[derive(Debug, Clone, PartialEq, State)]
pub struct Item {
    pub kind: ItemKind,
    pub effect: Effect,
    pub value: OneZero,
}

impl ItemKind {
    pub fn name_text(&self) -> crate::l10n::item_kind::ItemKindText {
        crate::l10n::item_kind::ItemKindText::Name(self.clone())
    }

    pub fn description_text(&self) -> crate::l10n::item_kind::ItemKindText {
        crate::l10n::item_kind::ItemKindText::Description(self.clone())
    }

    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        self.thumbnail_with_shadow(width_height, STICKER_THUMBNAIL_STROKE, false)
    }

    pub fn thumbnail_with_shadow(
        &self,
        width_height: Wh<Px>,
        stroke_px: Px,
        shadow: bool,
    ) -> RenderingTree {
        match self {
            ItemKind::RiceBall => render_sticker_image_with_shadow(
                thumbnail_image::RICE_BALL,
                width_height,
                stroke_px,
                shadow,
            ),
            ItemKind::LumpSugar => render_sticker_image_with_shadow(
                thumbnail_image::LUMP_SUGAR,
                width_height,
                stroke_px,
                shadow,
            ),
            ItemKind::Shield => render_sticker_image_with_shadow(
                thumbnail_image::SHIELD,
                width_height,
                stroke_px,
                shadow,
            ),
            ItemKind::Painkiller => render_sticker_image_with_shadow(
                thumbnail_image::PAINKILLER,
                width_height,
                stroke_px,
                shadow,
            ),
            ItemKind::GrantBarricades => render_sticker_image_with_shadow(
                thumbnail_image::GRANT_BARRICADES,
                width_height,
                stroke_px,
                shadow,
            ),
            ItemKind::GrantCard { card } => {
                render_card_thumbnail(card, width_height, stroke_px, shadow)
            }
        }
    }
}

impl Item {
    pub fn grant_card(card: Card) -> Self {
        Item {
            kind: ItemKind::GrantCard { card },
            effect: Effect::AddCardToHand { card },
            value: 1.0.into(),
        }
    }

    pub fn name_text(&self) -> crate::l10n::item_kind::ItemKindText {
        self.kind.name_text()
    }

    pub fn description_text(&self) -> crate::l10n::item_kind::ItemKindText {
        self.kind.description_text()
    }

    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        self.kind.thumbnail(width_height)
    }

    /// 아이템이 현재 게임 상태에서 사용 가능한지 확인
    pub fn can_use(
        &self,
        game_state: &crate::game_state::GameState,
    ) -> Result<(), crate::game_state::effect::EffectExecutionError> {
        self.effect.can_execute(game_state)
    }

    /// 아이템을 사용할 수 없는 이유를 사용자에게 보여줄 메시지로 반환
    pub fn usage_error_message<'a>(
        &self,
        error: &crate::game_state::effect::EffectExecutionError,
        text_manager: &crate::l10n::TextManager,
        builder: crate::theme::typography::TypographyBuilder<'a>,
    ) -> crate::theme::typography::TypographyBuilder<'a> {
        self.effect
            .execution_error_message(error, text_manager, builder)
    }
}
