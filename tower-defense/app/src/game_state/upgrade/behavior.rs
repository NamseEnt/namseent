#[cfg(test)]
use super::state::UpgradeState;
use crate::card::{Rank, Suit};
use crate::game_state::GameState;
use crate::game_state::tower::{Tower, TowerKind, TowerTemplate};
use crate::rarity::Rarity;
use crate::{FixedRatio, Health, HealthDelta, TowerId};
use enum_dispatch::enum_dispatch;
use namui::*;
use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// Upgrade Trait and Structs
// ============================================================================

/// Presentation-only behavior for a legacy typed upgrade record.
///
/// Authoritative acquisition, recovery, damage, and trigger mutation live in
/// `td_core`. The typed records implementing this trait are retained only for
/// presentation and legacy save/replay codecs.
#[enum_dispatch]
pub trait UpgradePresentation {
    fn key(&self) -> &'static str;

    fn is_applicable(&self, _context: &SelectedTowerContext) -> bool {
        false
    }

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

    fn thumbnail_source(&self) -> crate::thumbnail::ThumbnailSource<'_>;

    fn thumbnail_overlays(
        &self,
        _game_state: &GameState,
    ) -> Vec<crate::thumbnail::ThumbnailOverlay> {
        Vec::new()
    }

    fn tooltip_sections(
        &self,
        locale: crate::l10n::Locale,
    ) -> Vec<crate::tooltip::TooltipSection<'_>> {
        vec![self.tooltip_section(locale)]
    }

    fn tooltip_section(&self, locale: crate::l10n::Locale) -> crate::tooltip::TooltipSection<'_> {
        crate::tooltip::TooltipSection {
            title: Some(crate::tooltip::SectionText {
                key: format!("upgrade:{}:name", self.key()),
                apply: Box::new(move |builder| {
                    self.l10n_name(builder, &locale);
                }),
            }),
            body: crate::tooltip::SectionText {
                key: format!("upgrade:{}:desc", self.key()),
                apply: Box::new(move |builder| {
                    self.l10n_description(builder, &locale);
                }),
            },
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
pub enum SelectedTowerId {
    Placed(TowerId),
    ToBePlaced,
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
pub struct SelectedTowerContext {
    pub tower_id: SelectedTowerId,
    pub kind: TowerKind,
    pub suit: Option<Suit>,
    pub rank: Option<Rank>,
    pub rerolled_count: Option<usize>,
}

impl SelectedTowerContext {
    pub fn from_tower(tower: &Tower) -> Self {
        Self {
            tower_id: SelectedTowerId::Placed(tower.id()),
            kind: tower.kind,
            suit: tower.suit,
            rank: tower.rank,
            rerolled_count: Some(tower.rerolled_count),
        }
    }

    pub fn from_template(template: &TowerTemplate, rerolled_count: Option<usize>) -> Self {
        Self {
            tower_id: SelectedTowerId::ToBePlaced,
            kind: template.kind,
            suit: template.suit,
            rank: template.rank,
            rerolled_count,
        }
    }
}

mod apple;
mod backpack;
mod banana;
mod black_white;
mod broken_pottery;
mod camera;
mod carrot;
mod cat;
mod crock;
mod cup_noodles;
mod demolition_hammer;
mod dice_bundle;
mod energy_drink;
mod fang;
mod four_leaf_clover;
mod french_fries;
mod gift_box;
mod hamburger;
mod ice_cream;
mod membership_card;
mod metronome;
mod mirror;
mod name_tag;
mod pea;
mod perfect_pottery;
mod piggy_bank;
mod pizza;
mod popcorn;
mod rabbit;
mod resolution;
mod shopping_bag;
mod slot_machine;
mod spanner;
mod strawberry;
mod tape;
mod trophy;
mod watermelon;

pub use apple::*;
pub use backpack::*;
pub use banana::*;
pub use black_white::*;
pub use broken_pottery::*;
pub use camera::*;
pub use carrot::*;
pub use cat::*;
pub use crock::*;
pub use cup_noodles::*;
pub use demolition_hammer::*;
pub use dice_bundle::*;
pub use energy_drink::*;
pub use fang::*;
pub use four_leaf_clover::*;
pub use french_fries::*;
pub use gift_box::*;
pub use hamburger::*;
pub use ice_cream::*;
pub use membership_card::*;
pub use metronome::*;
pub use mirror::*;
pub use name_tag::*;
pub use pea::*;
pub use perfect_pottery::*;
pub use piggy_bank::*;
pub use pizza::*;
pub use popcorn::*;
pub use rabbit::*;
pub use resolution::*;
pub use shopping_bag::*;
pub use slot_machine::*;
pub use spanner::*;
pub use strawberry::*;
pub use tape::*;
pub use trophy::*;
pub use watermelon::*;

/// Legacy typed upgrade payload.
///
/// This enum is not authoritative gameplay state. Its fields are retained so
/// old saves/replays and headed presentation caches can be decoded losslessly.
/// Convert it to [`td_core::UpgradeEntryState`] before any gameplay mutation.
#[enum_dispatch(UpgradePresentation)]
#[derive(Debug, Clone, Copy, State, PartialEq, strum_macros::EnumDiscriminants)]
#[strum_discriminants(
    derive(
        strum_macros::EnumIter,
        strum_macros::AsRefStr,
        strum_macros::EnumString
    ),
    name(UpgradeDiscriminants)
)]
pub enum Upgrade {
    Apple(AppleUpgrade),
    Banana(BananaUpgrade),
    Carrot(CarrotUpgrade),
    Cat(CatUpgrade),
    Backpack(BackpackUpgrade),
    DiceBundle(DiceBundleUpgrade),
    EnergyDrink(EnergyDrinkUpgrade),
    PerfectPottery(PerfectPotteryUpgrade),
    FourLeafClover(FourLeafCloverUpgrade),
    Rabbit(RabbitUpgrade),
    BlackWhite(BlackWhiteUpgrade),
    Trophy(TrophyUpgrade),
    Crock(CrockUpgrade),
    CupNoodles(CupNoodlesUpgrade),
    FrenchFries(FrenchFriesUpgrade),
    Hamburger(HamburgerUpgrade),
    Pizza(PizzaUpgrade),
    DemolitionHammer(DemolitionHammerUpgrade),
    Metronome(MetronomeUpgrade),
    Tape(TapeUpgrade),
    NameTag(NameTagUpgrade),
    ShoppingBag(ShoppingBagUpgrade),
    Resolution(ResolutionUpgrade),
    Mirror(MirrorUpgrade),
    IceCream(IceCreamUpgrade),
    Spanner(SpannerUpgrade),
    Pea(PeaUpgrade),
    SlotMachine(SlotMachineUpgrade),
    PiggyBank(PiggyBankUpgrade),
    Camera(CameraUpgrade),
    GiftBox(GiftBoxUpgrade),
    Fang(FangUpgrade),
    Popcorn(PopcornUpgrade),
    MembershipCard(MembershipCardUpgrade),
    BrokenPottery(BrokenPotteryUpgrade),
    Strawberry(StrawberryUpgrade),
    Watermelon(WatermelonUpgrade),
}

#[derive(Debug, Clone, Copy, State, PartialEq, Eq, Hash)]
pub struct UpgradeId(pub u64);

/// Presentation/migration wrapper around a core-owned upgrade entry.
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct UpgradeWithId {
    pub id: UpgradeId,
    pub upgrade: Upgrade,
}

static NEXT_UPGRADE_ID: AtomicU64 = AtomicU64::new(1);

impl UpgradeWithId {
    /// Legacy headed codec for the stable core upgrade payload.
    pub fn new(upgrade: Upgrade) -> Self {
        Self {
            id: UpgradeId(NEXT_UPGRADE_ID.fetch_add(1, Ordering::Relaxed)),
            upgrade,
        }
    }

    /// Encodes the legacy typed payload without changing its wire shape.
    pub fn to_core_state(self) -> td_core::UpgradeEntryState {
        let mut state = td_core::UpgradeEntryState {
            id: self.id.0,
            kind: self.upgrade.core_kind().raw(),
            scalar_values: Vec::new(),
            ratio_values_raw: Vec::new(),
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        };
        match &self.upgrade {
            Upgrade::Backpack(upgrade) => state.scalar_values.push(upgrade.add as u64),
            Upgrade::Cat(upgrade) => state.scalar_values.push(upgrade.add as u64),
            Upgrade::Crock(upgrade) => state.scalar_values.push(upgrade.current_step as u64),
            Upgrade::DiceBundle(upgrade) => state.scalar_values.push(upgrade.add as u64),
            Upgrade::EnergyDrink(upgrade) => state.scalar_values.push(upgrade.add as u64),
            Upgrade::Fang(upgrade) => state.scalar_values.push(upgrade.add as u64),
            Upgrade::GiftBox(upgrade) => state.scalar_values.push(upgrade.add as u64),
            Upgrade::IceCream(upgrade) => {
                state.ratio_values_raw.push(upgrade.damage_bonus_pct.raw());
                state.scalar_values.push(upgrade.waves_remaining as u64);
            }
            Upgrade::MembershipCard(upgrade) => state.bool_values.push(upgrade.pending_free_shop),
            Upgrade::Metronome(upgrade) => state.scalar_values.push(upgrade.acquired_stage as u64),
            Upgrade::Mirror(upgrade) => state.bool_values.push(upgrade.pending),
            Upgrade::NameTag(upgrade) => {
                state.ratio_values_raw.push(upgrade.damage_bonus_pct.raw());
                state
                    .optional_ids
                    .push(upgrade.target_tower_id.map(|id| id.raw()));
            }
            Upgrade::PerfectPottery(upgrade) => {
                state.ratio_values_raw.push(upgrade.damage_bonus_pct.raw())
            }
            Upgrade::Popcorn(upgrade) => {
                state.ratio_values_raw.push(upgrade.max_multiplier.raw());
                state.scalar_values.push(upgrade.duration as u64);
                state.scalar_values.push(upgrade.waves_remaining as u64);
                state
                    .ratio_values_raw
                    .push(upgrade.active_stage_damage_bonus.raw());
            }
            Upgrade::Resolution(upgrade) => {
                state
                    .ratio_values_raw
                    .push(upgrade.damage_bonus_pct_per_reroll.raw());
                state.scalar_values.push(upgrade.stored_rerolls as u64);
            }
            Upgrade::SlotMachine(upgrade) => {
                state.scalar_values.push(upgrade.next_round_dice as u64)
            }
            Upgrade::Tape(upgrade) => state.scalar_values.push(upgrade.acquired_stage as u64),
            Upgrade::Apple(_)
            | Upgrade::Banana(_)
            | Upgrade::BlackWhite(_)
            | Upgrade::BrokenPottery(_)
            | Upgrade::Camera(_)
            | Upgrade::Carrot(_)
            | Upgrade::CupNoodles(_)
            | Upgrade::DemolitionHammer(_)
            | Upgrade::FourLeafClover(_)
            | Upgrade::FrenchFries(_)
            | Upgrade::Hamburger(_)
            | Upgrade::Pea(_)
            | Upgrade::PiggyBank(_)
            | Upgrade::Pizza(_)
            | Upgrade::Rabbit(_)
            | Upgrade::ShoppingBag(_)
            | Upgrade::Spanner(_)
            | Upgrade::Strawberry(_)
            | Upgrade::Trophy(_)
            | Upgrade::Watermelon(_) => {}
        }
        state
    }

    /// Rehydrates a presentation record from a core payload without granting
    /// the record any authority over the core state.
    pub fn from_core_state(state: td_core::UpgradeEntryState) -> Option<Self> {
        let core_kind = td_core::UpgradeKind::from_raw(state.kind)?;
        let kind = UpgradeDiscriminants::from_core_kind(core_kind);
        let scalar = |index: usize| usize::try_from(*state.scalar_values.get(index)?).ok();
        let ratio = |index: usize| Some(FixedRatio::from_raw(*state.ratio_values_raw.get(index)?));
        let boolean = |index: usize| state.bool_values.get(index).copied();
        let optional_id = |index: usize| {
            state
                .optional_ids
                .get(index)
                .copied()
                .map(|id| id.map(TowerId::from_raw))
        };
        let upgrade = match kind {
            UpgradeDiscriminants::Apple => Upgrade::Apple(AppleUpgrade),
            UpgradeDiscriminants::Banana => Upgrade::Banana(BananaUpgrade),
            UpgradeDiscriminants::Carrot => Upgrade::Carrot(CarrotUpgrade),
            UpgradeDiscriminants::Cat => Upgrade::Cat(CatUpgrade { add: scalar(0)? }),
            UpgradeDiscriminants::Backpack => {
                Upgrade::Backpack(BackpackUpgrade { add: scalar(0)? })
            }
            UpgradeDiscriminants::DiceBundle => {
                Upgrade::DiceBundle(DiceBundleUpgrade { add: scalar(0)? })
            }
            UpgradeDiscriminants::EnergyDrink => {
                Upgrade::EnergyDrink(EnergyDrinkUpgrade { add: scalar(0)? })
            }
            UpgradeDiscriminants::PerfectPottery => {
                Upgrade::PerfectPottery(PerfectPotteryUpgrade {
                    damage_bonus_pct: ratio(0)?,
                })
            }
            UpgradeDiscriminants::FourLeafClover => Upgrade::FourLeafClover(FourLeafCloverUpgrade),
            UpgradeDiscriminants::Rabbit => Upgrade::Rabbit(RabbitUpgrade),
            UpgradeDiscriminants::BlackWhite => Upgrade::BlackWhite(BlackWhiteUpgrade),
            UpgradeDiscriminants::Trophy => Upgrade::Trophy(TrophyUpgrade),
            UpgradeDiscriminants::Crock => Upgrade::Crock(CrockUpgrade {
                current_step: scalar(0)?,
            }),
            UpgradeDiscriminants::CupNoodles => Upgrade::CupNoodles(CupNoodlesUpgrade),
            UpgradeDiscriminants::FrenchFries => Upgrade::FrenchFries(FrenchFriesUpgrade),
            UpgradeDiscriminants::Hamburger => Upgrade::Hamburger(HamburgerUpgrade),
            UpgradeDiscriminants::Pizza => Upgrade::Pizza(PizzaUpgrade),
            UpgradeDiscriminants::DemolitionHammer => {
                Upgrade::DemolitionHammer(DemolitionHammerUpgrade)
            }
            UpgradeDiscriminants::Metronome => Upgrade::Metronome(MetronomeUpgrade {
                acquired_stage: scalar(0)?,
            }),
            UpgradeDiscriminants::Tape => Upgrade::Tape(TapeUpgrade {
                acquired_stage: scalar(0)?,
            }),
            UpgradeDiscriminants::NameTag => Upgrade::NameTag(NameTagUpgrade {
                damage_bonus_pct: ratio(0)?,
                target_tower_id: optional_id(0)?,
            }),
            UpgradeDiscriminants::ShoppingBag => Upgrade::ShoppingBag(ShoppingBagUpgrade),
            UpgradeDiscriminants::Resolution => Upgrade::Resolution(ResolutionUpgrade {
                damage_bonus_pct_per_reroll: ratio(0)?,
                stored_rerolls: scalar(0)?,
            }),
            UpgradeDiscriminants::Mirror => Upgrade::Mirror(MirrorUpgrade {
                pending: boolean(0)?,
            }),
            UpgradeDiscriminants::IceCream => Upgrade::IceCream(IceCreamUpgrade {
                damage_bonus_pct: ratio(0)?,
                waves_remaining: scalar(0)?,
            }),
            UpgradeDiscriminants::Spanner => Upgrade::Spanner(SpannerUpgrade),
            UpgradeDiscriminants::Pea => Upgrade::Pea(PeaUpgrade),
            UpgradeDiscriminants::SlotMachine => Upgrade::SlotMachine(SlotMachineUpgrade {
                next_round_dice: scalar(0)?,
            }),
            UpgradeDiscriminants::PiggyBank => Upgrade::PiggyBank(PiggyBankUpgrade),
            UpgradeDiscriminants::Camera => Upgrade::Camera(CameraUpgrade),
            UpgradeDiscriminants::GiftBox => Upgrade::GiftBox(GiftBoxUpgrade { add: scalar(0)? }),
            UpgradeDiscriminants::Fang => Upgrade::Fang(FangUpgrade { add: scalar(0)? }),
            UpgradeDiscriminants::Popcorn => Upgrade::Popcorn(PopcornUpgrade {
                max_multiplier: ratio(0)?,
                duration: scalar(0)?,
                waves_remaining: scalar(1)?,
                active_stage_damage_bonus: ratio(1)?,
            }),
            UpgradeDiscriminants::MembershipCard => {
                Upgrade::MembershipCard(MembershipCardUpgrade {
                    pending_free_shop: boolean(0)?,
                })
            }
            UpgradeDiscriminants::BrokenPottery => Upgrade::BrokenPottery(BrokenPotteryUpgrade),
            UpgradeDiscriminants::Strawberry => Upgrade::Strawberry(StrawberryUpgrade),
            UpgradeDiscriminants::Watermelon => Upgrade::Watermelon(WatermelonUpgrade),
        };
        let restored = Self {
            id: UpgradeId(state.id),
            upgrade,
        };
        (restored.to_core_state() == state).then_some(restored)
    }
}

impl std::ops::Deref for UpgradeWithId {
    type Target = Upgrade;

    fn deref(&self) -> &Self::Target {
        &self.upgrade
    }
}

impl std::ops::DerefMut for UpgradeWithId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.upgrade
    }
}

impl PartialEq<Upgrade> for UpgradeWithId {
    fn eq(&self, other: &Upgrade) -> bool {
        self.upgrade == *other
    }
}

impl Upgrade {
    pub fn with_unique_id(self) -> UpgradeWithId {
        UpgradeWithId::new(self)
    }

    pub fn discriminant(&self) -> UpgradeDiscriminants {
        match self {
            Upgrade::Apple(_) => UpgradeDiscriminants::Apple,
            Upgrade::Banana(_) => UpgradeDiscriminants::Banana,
            Upgrade::Carrot(_) => UpgradeDiscriminants::Carrot,
            Upgrade::Cat(_) => UpgradeDiscriminants::Cat,
            Upgrade::Backpack(_) => UpgradeDiscriminants::Backpack,
            Upgrade::DiceBundle(_) => UpgradeDiscriminants::DiceBundle,
            Upgrade::EnergyDrink(_) => UpgradeDiscriminants::EnergyDrink,
            Upgrade::PerfectPottery(_) => UpgradeDiscriminants::PerfectPottery,
            Upgrade::FourLeafClover(_) => UpgradeDiscriminants::FourLeafClover,
            Upgrade::Rabbit(_) => UpgradeDiscriminants::Rabbit,
            Upgrade::BlackWhite(_) => UpgradeDiscriminants::BlackWhite,
            Upgrade::Trophy(_) => UpgradeDiscriminants::Trophy,
            Upgrade::Crock(_) => UpgradeDiscriminants::Crock,
            Upgrade::CupNoodles(_) => UpgradeDiscriminants::CupNoodles,
            Upgrade::FrenchFries(_) => UpgradeDiscriminants::FrenchFries,
            Upgrade::Hamburger(_) => UpgradeDiscriminants::Hamburger,
            Upgrade::Pizza(_) => UpgradeDiscriminants::Pizza,
            Upgrade::DemolitionHammer(_) => UpgradeDiscriminants::DemolitionHammer,
            Upgrade::Metronome(_) => UpgradeDiscriminants::Metronome,
            Upgrade::Tape(_) => UpgradeDiscriminants::Tape,
            Upgrade::NameTag(_) => UpgradeDiscriminants::NameTag,
            Upgrade::ShoppingBag(_) => UpgradeDiscriminants::ShoppingBag,
            Upgrade::Resolution(_) => UpgradeDiscriminants::Resolution,
            Upgrade::Mirror(_) => UpgradeDiscriminants::Mirror,
            Upgrade::IceCream(_) => UpgradeDiscriminants::IceCream,
            Upgrade::Spanner(_) => UpgradeDiscriminants::Spanner,
            Upgrade::Pea(_) => UpgradeDiscriminants::Pea,
            Upgrade::SlotMachine(_) => UpgradeDiscriminants::SlotMachine,
            Upgrade::PiggyBank(_) => UpgradeDiscriminants::PiggyBank,
            Upgrade::Camera(_) => UpgradeDiscriminants::Camera,
            Upgrade::GiftBox(_) => UpgradeDiscriminants::GiftBox,
            Upgrade::Fang(_) => UpgradeDiscriminants::Fang,
            Upgrade::Popcorn(_) => UpgradeDiscriminants::Popcorn,
            Upgrade::MembershipCard(_) => UpgradeDiscriminants::MembershipCard,
            Upgrade::BrokenPottery(_) => UpgradeDiscriminants::BrokenPottery,
            Upgrade::Strawberry(_) => UpgradeDiscriminants::Strawberry,
            Upgrade::Watermelon(_) => UpgradeDiscriminants::Watermelon,
        }
    }
}

impl UpgradeDiscriminants {
    /// Encodes the presentation discriminant at the legacy raw codec boundary.
    #[allow(dead_code)]
    pub(crate) const fn to_core_raw(self) -> u8 {
        self.core_kind().raw()
    }

    /// Decodes a legacy raw codec value at the presentation boundary.
    #[allow(dead_code)]
    pub(crate) const fn from_core_raw(value: u8) -> Option<Self> {
        match td_core::UpgradeKind::from_raw(value) {
            Some(kind) => Some(Self::from_core_kind(kind)),
            None => None,
        }
    }

    pub(crate) const fn from_core_kind(kind: td_core::UpgradeKind) -> Self {
        match kind {
            td_core::UpgradeKind::Apple => Self::Apple,
            td_core::UpgradeKind::Banana => Self::Banana,
            td_core::UpgradeKind::Carrot => Self::Carrot,
            td_core::UpgradeKind::Cat => Self::Cat,
            td_core::UpgradeKind::Backpack => Self::Backpack,
            td_core::UpgradeKind::DiceBundle => Self::DiceBundle,
            td_core::UpgradeKind::EnergyDrink => Self::EnergyDrink,
            td_core::UpgradeKind::PerfectPottery => Self::PerfectPottery,
            td_core::UpgradeKind::FourLeafClover => Self::FourLeafClover,
            td_core::UpgradeKind::Rabbit => Self::Rabbit,
            td_core::UpgradeKind::BlackWhite => Self::BlackWhite,
            td_core::UpgradeKind::Trophy => Self::Trophy,
            td_core::UpgradeKind::Crock => Self::Crock,
            td_core::UpgradeKind::CupNoodles => Self::CupNoodles,
            td_core::UpgradeKind::FrenchFries => Self::FrenchFries,
            td_core::UpgradeKind::Hamburger => Self::Hamburger,
            td_core::UpgradeKind::Pizza => Self::Pizza,
            td_core::UpgradeKind::DemolitionHammer => Self::DemolitionHammer,
            td_core::UpgradeKind::Metronome => Self::Metronome,
            td_core::UpgradeKind::Tape => Self::Tape,
            td_core::UpgradeKind::NameTag => Self::NameTag,
            td_core::UpgradeKind::ShoppingBag => Self::ShoppingBag,
            td_core::UpgradeKind::Resolution => Self::Resolution,
            td_core::UpgradeKind::Mirror => Self::Mirror,
            td_core::UpgradeKind::IceCream => Self::IceCream,
            td_core::UpgradeKind::Spanner => Self::Spanner,
            td_core::UpgradeKind::Pea => Self::Pea,
            td_core::UpgradeKind::SlotMachine => Self::SlotMachine,
            td_core::UpgradeKind::PiggyBank => Self::PiggyBank,
            td_core::UpgradeKind::Camera => Self::Camera,
            td_core::UpgradeKind::GiftBox => Self::GiftBox,
            td_core::UpgradeKind::Fang => Self::Fang,
            td_core::UpgradeKind::Popcorn => Self::Popcorn,
            td_core::UpgradeKind::MembershipCard => Self::MembershipCard,
            td_core::UpgradeKind::BrokenPottery => Self::BrokenPottery,
            td_core::UpgradeKind::Strawberry => Self::Strawberry,
            td_core::UpgradeKind::Watermelon => Self::Watermelon,
        }
    }

    pub fn rarity(self) -> Rarity {
        td_core::upgrade_rarity(self.core_kind()).into()
    }

    pub(crate) fn core_kind_key(self) -> &'static str {
        self.core_kind().key()
    }

    pub(crate) const fn core_kind(self) -> td_core::UpgradeKind {
        match self {
            Self::Apple => td_core::UpgradeKind::Apple,
            Self::Banana => td_core::UpgradeKind::Banana,
            Self::Carrot => td_core::UpgradeKind::Carrot,
            Self::Cat => td_core::UpgradeKind::Cat,
            Self::Backpack => td_core::UpgradeKind::Backpack,
            Self::DiceBundle => td_core::UpgradeKind::DiceBundle,
            Self::EnergyDrink => td_core::UpgradeKind::EnergyDrink,
            Self::PerfectPottery => td_core::UpgradeKind::PerfectPottery,
            Self::FourLeafClover => td_core::UpgradeKind::FourLeafClover,
            Self::Rabbit => td_core::UpgradeKind::Rabbit,
            Self::BlackWhite => td_core::UpgradeKind::BlackWhite,
            Self::Trophy => td_core::UpgradeKind::Trophy,
            Self::Crock => td_core::UpgradeKind::Crock,
            Self::CupNoodles => td_core::UpgradeKind::CupNoodles,
            Self::FrenchFries => td_core::UpgradeKind::FrenchFries,
            Self::Hamburger => td_core::UpgradeKind::Hamburger,
            Self::Pizza => td_core::UpgradeKind::Pizza,
            Self::DemolitionHammer => td_core::UpgradeKind::DemolitionHammer,
            Self::Metronome => td_core::UpgradeKind::Metronome,
            Self::Tape => td_core::UpgradeKind::Tape,
            Self::NameTag => td_core::UpgradeKind::NameTag,
            Self::ShoppingBag => td_core::UpgradeKind::ShoppingBag,
            Self::Resolution => td_core::UpgradeKind::Resolution,
            Self::Mirror => td_core::UpgradeKind::Mirror,
            Self::IceCream => td_core::UpgradeKind::IceCream,
            Self::Spanner => td_core::UpgradeKind::Spanner,
            Self::Pea => td_core::UpgradeKind::Pea,
            Self::SlotMachine => td_core::UpgradeKind::SlotMachine,
            Self::PiggyBank => td_core::UpgradeKind::PiggyBank,
            Self::Camera => td_core::UpgradeKind::Camera,
            Self::GiftBox => td_core::UpgradeKind::GiftBox,
            Self::Fang => td_core::UpgradeKind::Fang,
            Self::Popcorn => td_core::UpgradeKind::Popcorn,
            Self::MembershipCard => td_core::UpgradeKind::MembershipCard,
            Self::BrokenPottery => td_core::UpgradeKind::BrokenPottery,
            Self::Strawberry => td_core::UpgradeKind::Strawberry,
            Self::Watermelon => td_core::UpgradeKind::Watermelon,
        }
    }
}

#[cfg(test)]
mod presentation_boundary_tests {
    use super::*;

    #[test]
    fn presentation_calls_do_not_change_authoritative_core_state() {
        let game_state = crate::game_state::upgrade::tests::support::create_mock_game_state();
        let before = td_core::authoritative_hash(game_state.raw_core_state());
        let upgrade = Upgrade::Mirror(MirrorUpgrade { pending: true });
        let context = SelectedTowerContext {
            tower_id: SelectedTowerId::ToBePlaced,
            kind: TowerKind::High,
            suit: None,
            rank: None,
            rerolled_count: None,
        };

        let _ = upgrade.key();
        let _ = upgrade.is_applicable(&context);
        let _ = upgrade.thumbnail_source();
        let _ = upgrade.thumbnail_overlays(&game_state);
        let _ = upgrade.tooltip_sections(crate::l10n::Locale::ENGLISH);

        assert_eq!(
            td_core::authoritative_hash(game_state.raw_core_state()),
            before
        );
    }

    #[test]
    fn presentation_discriminants_use_core_catalog_identity() {
        use strum::IntoEnumIterator;

        for discriminant in UpgradeDiscriminants::iter() {
            assert_eq!(discriminant.core_kind_key(), discriminant.core_kind().key());
        }
        assert_eq!(
            UpgradeDiscriminants::iter().count(),
            td_core::UpgradeKind::COUNT
        );
    }
}

impl Upgrade {
    pub fn core_kind(&self) -> td_core::UpgradeKind {
        self.discriminant().core_kind()
    }

    pub fn name_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::Name(self)
    }

    pub fn description_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::DescriptionUpgrade(self)
    }
}
#[cfg(test)]
mod food_upgrade_tests {
    use super::*;

    #[test]
    fn max_hp_food_upgrades_recover_after_cache_refresh() {
        use crate::game_state::upgrade::tests::support;

        let cases = [
            (Upgrade::Apple(AppleUpgrade), 4_i64, 6_i64),
            (Upgrade::Banana(BananaUpgrade), 6, 9),
            (Upgrade::Strawberry(StrawberryUpgrade), 2, 3),
            (Upgrade::Watermelon(WatermelonUpgrade), 8, 12),
            (Upgrade::FrenchFries(FrenchFriesUpgrade), -4, 12),
            (Upgrade::Hamburger(HamburgerUpgrade), -6, 18),
            (Upgrade::Pizza(PizzaUpgrade), -8, 24),
        ];

        for (upgrade, max_hp_plus, heal_amount) in cases {
            let mut game_state = support::create_mock_game_state();
            let base_max_hp = game_state.max_hp();
            game_state.hp = base_max_hp.saturating_sub(crate::Health::from_integer(10));

            game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
                upgrade, None,
            ));

            let hp_delta = crate::HealthDelta::from_integer(max_hp_plus);
            let heal_amount = crate::Health::from_integer(heal_amount);
            assert_eq!(
                game_state.max_hp(),
                base_max_hp.saturating_add_delta(hp_delta)
            );
            assert_eq!(
                game_state.hp,
                base_max_hp
                    .saturating_sub(crate::Health::from_integer(10))
                    .saturating_add(heal_amount)
                    .min(game_state.max_hp())
            );
        }
    }

    #[test]
    fn carrot_fully_recovers_after_cache_refresh() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        game_state.hp = crate::Health::from_integer(1);

        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            Upgrade::Carrot(CarrotUpgrade),
            None,
        ));

        assert_eq!(
            game_state
                .raw_core_state()
                .upgrades()
                .upgrades
                .iter()
                .filter(|upgrade| { upgrade.upgrade_kind() == Ok(td_core::UpgradeKind::Carrot) })
                .count(),
            1
        );
        assert_eq!(
            game_state
                .presentation_upgrade_state_snapshot()
                .max_hp_plus(),
            crate::HealthDelta::from_integer(6)
        );
        assert_eq!(game_state.hp, game_state.max_hp());
    }
    #[test]
    fn cup_noodles_decreases_max_hp_before_recovering() {
        use crate::game_state::upgrade::tests::support;

        let mut game_state = support::create_mock_game_state();
        let base_max_hp = game_state.max_hp();
        game_state.hp = base_max_hp.saturating_sub(crate::Health::from_integer(10));

        game_state.apply_compatibility_action(crate::game_state::CompatibilityAction::Upgrade(
            Upgrade::CupNoodles(CupNoodlesUpgrade),
            None,
        ));

        assert_eq!(
            game_state
                .raw_core_state()
                .upgrades()
                .upgrades
                .iter()
                .filter(|upgrade| {
                    upgrade.upgrade_kind() == Ok(td_core::UpgradeKind::CupNoodles)
                })
                .count(),
            1
        );
        assert_eq!(
            game_state
                .presentation_upgrade_state_snapshot()
                .max_hp_plus(),
            crate::HealthDelta::from_integer(-2)
        );
        assert_eq!(
            game_state.max_hp(),
            base_max_hp.saturating_add_delta(crate::HealthDelta::from_integer(-2))
        );
        assert_eq!(
            game_state.hp,
            base_max_hp.saturating_sub(crate::Health::from_integer(4))
        );
    }
}
