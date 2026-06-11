use super::state::UpgradeState;
use super::*;
use crate::game_state::GameState;
use crate::game_state::tower::{Tower, TowerKind, TowerTemplate};
use crate::game_state::upgrade::tower::TowerUpgradeTarget;
use crate::rarity::Rarity;
use enum_dispatch::enum_dispatch;
use namui::*;

const UPGRADE_STICKER_THUMBNAIL_STROKE: Px = px(6.0);
use std::sync::atomic::{AtomicU64, Ordering};

// ============================================================================
// Upgrade Trait and Structs
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, State)]
pub struct UpgradeUpdateFlags(u8);

impl UpgradeUpdateFlags {
    pub const NONE: Self = Self(0);
    pub const TOWER_STATS: Self = Self(1 << 0);
    pub const CACHE: Self = Self(1 << 1);
    pub const HEAL_TO_FULL: Self = Self(1 << 2);
    pub const REVISION: Self = Self(1 << 3);

    pub fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl std::ops::BitOr for UpgradeUpdateFlags {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for UpgradeUpdateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

/// Common trait for all upgrade behaviors
#[enum_dispatch]
pub trait UpgradeBehavior {
    fn is_applicable(&self, _context: &SelectedTowerContext) -> bool {
        false
    }

    fn on_tower_placed(
        &mut self,
        _game_state: &mut GameState,
        _tower: &Tower,
    ) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_tower_removed(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn tower_upgrade_damage_bonus(&self) -> Option<(TowerUpgradeTarget, f32)> {
        None
    }

    fn on_monster_death(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_stage_start(&mut self, _game_state: &mut GameState, _stage: usize) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_stage_end(
        &mut self,
        _game_state: &mut GameState,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_card_reroll(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn max_hp_plus(&self) -> f32 {
        0.0
    }

    fn shop_slot_expand(&self) -> usize {
        0
    }

    fn dice_chance_plus(&self) -> usize {
        0
    }

    fn shop_item_price_minus(&self) -> usize {
        0
    }

    fn shorten_straight_flush_to_4_cards(&self) -> bool {
        false
    }

    fn skip_rank_for_straight(&self) -> bool {
        false
    }

    fn treat_suits_as_same(&self) -> bool {
        false
    }

    fn removed_number_rank_count(&self) -> usize {
        0
    }

    fn acquire(self, game_state: &mut GameState) -> UpgradeUpdateFlags
    where
        Self: Sized + Into<Upgrade>,
    {
        game_state
            .upgrade_state
            .upgrades
            .push(self.into().with_unique_id());
        UpgradeUpdateFlags::REVISION
    }

    fn on_item_bought(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_gold_earned(
        &mut self,
        _game_state: &mut GameState,
        _earned: usize,
    ) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_gold_spent(&mut self, _game_state: &mut GameState, _spent: usize) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn clear_shield_on_stage_start(&self) -> bool {
        true
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

    fn thumbnail(&self, width_height: Wh<Px>, shadow: bool) -> RenderingTree;

    fn thumbnail_overlay(
        &self,
        _width_height: Wh<Px>,
        _game_state: &GameState,
    ) -> Option<RenderingTree> {
        None
    }

    #[allow(dead_code)]
    fn rarity(&self) -> crate::Rarity {
        crate::Rarity::Common
    }
}

#[derive(Clone, Copy)]
pub(super) struct UpgradeDefinition {
    generate: fn(&UpgradeState) -> Upgrade,
    current_and_max: fn(&UpgradeState) -> Option<(usize, usize)>,
    rarity: fn() -> Rarity,
}

impl UpgradeDefinition {
    pub(super) const fn new(
        generate: fn(&UpgradeState) -> Upgrade,
        current_and_max: fn(&UpgradeState) -> Option<(usize, usize)>,
        rarity: fn() -> Rarity,
    ) -> Self {
        Self {
            generate,
            current_and_max,
            rarity,
        }
    }

    const fn rarity_common() -> Rarity {
        Rarity::Common
    }
    const fn rarity_rare() -> Rarity {
        Rarity::Rare
    }
    const fn rarity_epic() -> Rarity {
        Rarity::Epic
    }
    const fn rarity_legendary() -> Rarity {
        Rarity::Legendary
    }

    pub(super) fn generate(self, upgrade_state: &UpgradeState) -> Upgrade {
        (self.generate)(upgrade_state)
    }

    pub(super) fn current_and_max(self, upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
        (self.current_and_max)(upgrade_state)
    }

    pub(super) fn rarity(self) -> Rarity {
        (self.rarity)()
    }
}

pub(super) fn no_current_and_max(_upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    None
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
pub enum SelectedTowerId {
    Placed(usize),
    ToBePlaced,
}

#[derive(Clone, Copy, PartialEq, Eq, State)]
pub struct SelectedTowerContext {
    pub tower_id: SelectedTowerId,
    pub kind: TowerKind,
    pub suit: Option<crate::card::Suit>,
    pub rank: Option<crate::card::Rank>,
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

    pub fn is_low_card_tower(&self) -> bool {
        self.kind.is_low_card_tower()
    }
}

mod backpack;
mod black_white;
mod broken_pottery;
mod brush;
mod camera;
mod cat;
mod club_sword;
mod crock;
mod demolition_hammer;
mod dice_bundle;
mod energy_drink;
mod fang;
mod fountain_pen;
mod four_leaf_clover;
mod gift_box;
mod ice_cream;
mod long_sword;
mod mace;
mod membership_card;
mod metronome;
mod mirror;
mod name_tag;
mod pair_chopsticks;
mod pea;
mod perfect_pottery;
mod piggy_bank;
mod popcorn;
mod rabbit;
mod resolution;
mod shopping_bag;
mod single_chopstick;
mod slot_machine;
mod spanner;
mod staff;
mod tape;
mod tricycle;
mod trophy;

pub use backpack::*;
pub use black_white::*;
pub use broken_pottery::*;
pub use brush::*;
pub use camera::*;
pub use cat::*;
pub use club_sword::*;
pub use crock::*;
pub use demolition_hammer::*;
pub use dice_bundle::*;
pub use energy_drink::*;
pub use fang::*;
pub use fountain_pen::*;
pub use four_leaf_clover::*;
pub use gift_box::*;
pub use ice_cream::*;
pub use long_sword::*;
pub use mace::*;
pub use membership_card::*;
pub use metronome::*;
pub use mirror::*;
pub use name_tag::*;
pub use pair_chopsticks::*;
pub use pea::*;
pub use perfect_pottery::*;
pub use piggy_bank::*;
pub use popcorn::*;
pub use rabbit::*;
pub use resolution::*;
pub use shopping_bag::*;
pub use single_chopstick::*;
pub use slot_machine::*;
pub use spanner::*;
pub use staff::*;
pub use tape::*;
pub use tricycle::*;
pub use trophy::*;

#[enum_dispatch(UpgradeBehavior)]
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
    Cat(CatUpgrade),
    Staff(StaffUpgrade),
    LongSword(LongSwordUpgrade),
    Mace(MaceUpgrade),
    ClubSword(ClubSwordUpgrade),
    Backpack(BackpackUpgrade),
    DiceBundle(DiceBundleUpgrade),
    Tricycle(TricycleUpgrade),
    EnergyDrink(EnergyDrinkUpgrade),
    PerfectPottery(PerfectPotteryUpgrade),
    SingleChopstick(SingleChopstickUpgrade),
    PairChopsticks(PairChopsticksUpgrade),
    FountainPen(FountainPenUpgrade),
    Brush(BrushUpgrade),
    FourLeafClover(FourLeafCloverUpgrade),
    Rabbit(RabbitUpgrade),
    BlackWhite(BlackWhiteUpgrade),
    Trophy(TrophyUpgrade),
    Crock(CrockUpgrade),
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
}

#[derive(Debug, Clone, Copy, State, PartialEq, Eq)]
pub struct UpgradeId(pub u64);

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct UpgradeWithId {
    pub id: UpgradeId,
    pub upgrade: Upgrade,
}

static NEXT_UPGRADE_ID: AtomicU64 = AtomicU64::new(1);

impl UpgradeWithId {
    pub fn new(upgrade: Upgrade) -> Self {
        Self {
            id: UpgradeId(NEXT_UPGRADE_ID.fetch_add(1, Ordering::Relaxed)),
            upgrade,
        }
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
            Upgrade::Cat(_) => UpgradeDiscriminants::Cat,
            Upgrade::Staff(_) => UpgradeDiscriminants::Staff,
            Upgrade::LongSword(_) => UpgradeDiscriminants::LongSword,
            Upgrade::Mace(_) => UpgradeDiscriminants::Mace,
            Upgrade::ClubSword(_) => UpgradeDiscriminants::ClubSword,
            Upgrade::Backpack(_) => UpgradeDiscriminants::Backpack,
            Upgrade::DiceBundle(_) => UpgradeDiscriminants::DiceBundle,
            Upgrade::Tricycle(_) => UpgradeDiscriminants::Tricycle,
            Upgrade::EnergyDrink(_) => UpgradeDiscriminants::EnergyDrink,
            Upgrade::PerfectPottery(_) => UpgradeDiscriminants::PerfectPottery,
            Upgrade::SingleChopstick(_) => UpgradeDiscriminants::SingleChopstick,
            Upgrade::PairChopsticks(_) => UpgradeDiscriminants::PairChopsticks,
            Upgrade::FountainPen(_) => UpgradeDiscriminants::FountainPen,
            Upgrade::Brush(_) => UpgradeDiscriminants::Brush,
            Upgrade::FourLeafClover(_) => UpgradeDiscriminants::FourLeafClover,
            Upgrade::Rabbit(_) => UpgradeDiscriminants::Rabbit,
            Upgrade::BlackWhite(_) => UpgradeDiscriminants::BlackWhite,
            Upgrade::Trophy(_) => UpgradeDiscriminants::Trophy,
            Upgrade::Crock(_) => UpgradeDiscriminants::Crock,
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
        }
    }
}

impl UpgradeDiscriminants {
    fn definition(self) -> UpgradeDefinition {
        match self {
            UpgradeDiscriminants::Cat => cat::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Staff => staff::UPGRADE_DEFINITION,
            UpgradeDiscriminants::LongSword => long_sword::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Mace => mace::UPGRADE_DEFINITION,
            UpgradeDiscriminants::ClubSword => club_sword::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Backpack => backpack::UPGRADE_DEFINITION,
            UpgradeDiscriminants::DiceBundle => dice_bundle::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Tricycle => tricycle::UPGRADE_DEFINITION,
            UpgradeDiscriminants::EnergyDrink => energy_drink::UPGRADE_DEFINITION,
            UpgradeDiscriminants::PerfectPottery => perfect_pottery::UPGRADE_DEFINITION,
            UpgradeDiscriminants::SingleChopstick => single_chopstick::UPGRADE_DEFINITION,
            UpgradeDiscriminants::PairChopsticks => pair_chopsticks::UPGRADE_DEFINITION,
            UpgradeDiscriminants::FountainPen => fountain_pen::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Brush => brush::UPGRADE_DEFINITION,
            UpgradeDiscriminants::FourLeafClover => four_leaf_clover::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Rabbit => rabbit::UPGRADE_DEFINITION,
            UpgradeDiscriminants::BlackWhite => black_white::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Trophy => trophy::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Crock => crock::UPGRADE_DEFINITION,
            UpgradeDiscriminants::DemolitionHammer => demolition_hammer::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Metronome => metronome::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Tape => tape::UPGRADE_DEFINITION,
            UpgradeDiscriminants::NameTag => name_tag::UPGRADE_DEFINITION,
            UpgradeDiscriminants::ShoppingBag => shopping_bag::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Resolution => resolution::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Mirror => mirror::UPGRADE_DEFINITION,
            UpgradeDiscriminants::IceCream => ice_cream::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Spanner => spanner::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Pea => pea::UPGRADE_DEFINITION,
            UpgradeDiscriminants::SlotMachine => slot_machine::UPGRADE_DEFINITION,
            UpgradeDiscriminants::PiggyBank => piggy_bank::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Camera => camera::UPGRADE_DEFINITION,
            UpgradeDiscriminants::GiftBox => gift_box::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Fang => fang::UPGRADE_DEFINITION,
            UpgradeDiscriminants::Popcorn => popcorn::UPGRADE_DEFINITION,
            UpgradeDiscriminants::MembershipCard => membership_card::UPGRADE_DEFINITION,
            UpgradeDiscriminants::BrokenPottery => broken_pottery::UPGRADE_DEFINITION,
        }
    }

    pub(crate) fn current_and_max(self, upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
        self.definition().current_and_max(upgrade_state)
    }

    pub(crate) fn generate(self, upgrade_state: &UpgradeState) -> Upgrade {
        self.definition().generate(upgrade_state)
    }

    pub(crate) fn rarity(self) -> Rarity {
        self.definition().rarity()
    }
}

impl Upgrade {
    pub fn name_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::Name(self)
    }

    pub fn description_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::DescriptionUpgrade(self)
    }
}
