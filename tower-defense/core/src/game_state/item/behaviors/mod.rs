use super::ItemUseEffect;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait ItemBehavior {
    fn kind(&self) -> crate::ItemKind;
    fn rarity(&self) -> crate::Rarity;
    fn generated_state(&self) -> ItemRuntimeState;
    fn can_use(&self, core: &crate::CoreState) -> bool;
    fn prepare_use(
        &self,
        core: &crate::CoreState,
    ) -> Result<Option<crate::TowerTemplateState>, crate::CommandError>;
    fn apply_use(
        &self,
        core: &mut crate::CoreState,
        state: ItemRuntimeState,
        prepared: Option<crate::TowerTemplateState>,
    ) -> Result<Vec<ItemUseEffect>, crate::CommandError>;
}

pub(super) mod bread;
pub(super) mod candy;
pub(super) mod cannoli;
pub(super) mod cookie;
pub(super) mod donut;
pub(super) mod gimbap;
pub(super) mod lump_sugar;
pub(super) mod lunch_box;
pub(super) mod milk;
pub(super) mod rice_ball;
pub(super) mod rubber_cone;
pub(super) mod support;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ItemRuntimeState {
    Bread(bread::BreadItemState),
    Candy(candy::CandyItemState),
    Cannoli(cannoli::CannoliItemState),
    Cookie(cookie::CookieItemState),
    Donut(donut::DonutItemState),
    Gimbap(gimbap::GimbapItemState),
    LumpSugar(lump_sugar::LumpSugarItemState),
    LunchBox(lunch_box::LunchBoxItemState),
    Milk(milk::MilkItemState),
    RiceBall(rice_ball::RiceBallItemState),
    RubberCone(rubber_cone::RubberConeItemState),
}

#[enum_dispatch(ItemBehavior)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ItemBehaviorImpl {
    Bread(bread::Behavior),
    Candy(candy::Behavior),
    Cannoli(cannoli::Behavior),
    Cookie(cookie::Behavior),
    Donut(donut::Behavior),
    Gimbap(gimbap::Behavior),
    LumpSugar(lump_sugar::Behavior),
    LunchBox(lunch_box::Behavior),
    Milk(milk::Behavior),
    RiceBall(rice_ball::Behavior),
    RubberCone(rubber_cone::Behavior),
}

impl ItemBehaviorImpl {
    pub(crate) fn for_kind(kind: crate::ItemKind) -> Self {
        match kind {
            crate::ItemKind::Bread => Self::Bread(bread::Behavior),
            crate::ItemKind::Candy => Self::Candy(candy::Behavior),
            crate::ItemKind::Cannoli => Self::Cannoli(cannoli::Behavior),
            crate::ItemKind::Cookie => Self::Cookie(cookie::Behavior),
            crate::ItemKind::Donut => Self::Donut(donut::Behavior),
            crate::ItemKind::Gimbap => Self::Gimbap(gimbap::Behavior),
            crate::ItemKind::LumpSugar => Self::LumpSugar(lump_sugar::Behavior),
            crate::ItemKind::LunchBox => Self::LunchBox(lunch_box::Behavior),
            crate::ItemKind::Milk => Self::Milk(milk::Behavior),
            crate::ItemKind::RiceBall => Self::RiceBall(rice_ball::Behavior),
            crate::ItemKind::RubberCone => Self::RubberCone(rubber_cone::Behavior),
        }
    }
}
