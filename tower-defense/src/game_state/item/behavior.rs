use std::sync::atomic::{AtomicU64, Ordering};

use crate::card::Card;
use crate::thumbnail::{
    STICKER_THUMBNAIL_STROKE, render_card_thumbnail, render_sticker_image_with_shadow,
};
use enum_dispatch::enum_dispatch;
use namui::*;

mod grant_barricades;
mod grant_card;
mod lump_sugar;
mod painkiller;
mod rice_ball;
mod shield;

pub use grant_barricades::*;
pub use grant_card::*;
pub use lump_sugar::*;
pub use painkiller::*;
pub use rice_ball::*;
pub use shield::*;

#[enum_dispatch]
pub trait ItemBehavior {
    fn can_use(&self, game_state: &crate::game_state::GameState) -> Result<(), ItemUseError> {
        if game_state.stage_modifiers.is_item_use_disabled() {
            return Err(ItemUseError::ItemUseDisabled);
        }
        Ok(())
    }

    fn use_item(&self, game_state: &mut crate::game_state::GameState);

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    );

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    );

    fn thumbnail_with_shadow(
        &self,
        width_height: Wh<Px>,
        stroke_px: Px,
        shadow: bool,
    ) -> RenderingTree;
}

#[enum_dispatch(ItemBehavior)]
#[derive(Debug, Clone, PartialEq, State, strum_macros::EnumDiscriminants)]
#[strum_discriminants(derive(State, strum_macros::EnumIter))]
pub enum Item {
    RiceBall(RiceBallItem),
    LumpSugar(LumpSugarItem),
    Shield(ShieldItem),
    Painkiller(PainkillerItem),
    GrantBarricades(GrantBarricadesItem),
    GrantCard(GrantCardItem),
}

#[derive(Debug, Clone, Copy, State, PartialEq, Eq)]
pub struct ItemId(pub u64);

#[derive(Debug, Clone, State, PartialEq)]
pub struct ItemWithId {
    pub id: ItemId,
    pub item: Item,
}

static NEXT_ITEM_ID: AtomicU64 = AtomicU64::new(1);

impl ItemWithId {
    pub fn new(item: Item) -> Self {
        Self {
            id: ItemId(NEXT_ITEM_ID.fetch_add(1, Ordering::Relaxed)),
            item,
        }
    }
}

impl std::ops::Deref for ItemWithId {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        &self.item
    }
}

impl std::ops::DerefMut for ItemWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.item
    }
}

impl PartialEq<Item> for ItemWithId {
    fn eq(&self, other: &Item) -> bool {
        self.item == *other
    }
}

#[derive(Clone, Debug, PartialEq, State)]
pub enum ItemUseError {
    ItemUseDisabled,
    InvalidFlow { required: String },
}

impl Item {
    pub fn with_unique_id(self) -> ItemWithId {
        ItemWithId::new(self)
    }

    pub fn can_use(&self, game_state: &crate::game_state::GameState) -> Result<(), ItemUseError> {
        ItemBehavior::can_use(self, game_state)
    }

    pub fn use_item(&self, game_state: &mut crate::game_state::GameState) {
        ItemBehavior::use_item(self, game_state)
    }

    pub fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        ItemBehavior::l10n_name(self, builder, locale)
    }

    pub fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        ItemBehavior::l10n_description(self, builder, locale)
    }

    pub fn thumbnail_with_shadow(
        &self,
        width_height: Wh<Px>,
        stroke_px: Px,
        shadow: bool,
    ) -> RenderingTree {
        ItemBehavior::thumbnail_with_shadow(self, width_height, stroke_px, shadow)
    }

    pub fn discriminant(&self) -> ItemDiscriminants {
        self.into()
    }

    pub fn thumbnail(&self, width_height: Wh<Px>) -> RenderingTree {
        self.thumbnail_with_shadow(width_height, STICKER_THUMBNAIL_STROKE, false)
    }
}

pub(crate) fn render_sticker(
    image: Image,
    width_height: Wh<Px>,
    stroke_px: Px,
    shadow: bool,
) -> RenderingTree {
    render_sticker_image_with_shadow(image, width_height, stroke_px, shadow)
}

pub(crate) fn render_card(
    card: &Card,
    width_height: Wh<Px>,
    stroke_px: Px,
    shadow: bool,
) -> RenderingTree {
    render_card_thumbnail(card, width_height, stroke_px, shadow)
}
