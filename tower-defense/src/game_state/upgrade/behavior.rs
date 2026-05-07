use super::state::UpgradeState;
use super::*;
use crate::game_state::GameState;
use crate::game_state::tower::{Tower, TowerTemplate};
use crate::*;

// ============================================================================
// Upgrade Trait and Structs
// ============================================================================

/// Common trait for all upgrade behaviors
#[derive(Debug, Clone, Copy, Default)]
pub struct TowerPlacementResult {
    pub gold_earn: usize,
}

impl std::ops::AddAssign for TowerPlacementResult {
    fn add_assign(&mut self, other: Self) {
        self.gold_earn += other.gold_earn;
    }
}

impl std::ops::Add for TowerPlacementResult {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            gold_earn: self.gold_earn + other.gold_earn,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StageStartEffects {
    pub extra_dice: usize,
    pub damage_multiplier: f32,
    pub enemy_speed_multiplier: Option<f32>,
    pub free_shop_this_stage: bool,
}

impl StageStartEffects {
    pub fn new() -> Self {
        Self {
            extra_dice: 0,
            damage_multiplier: 1.0,
            enemy_speed_multiplier: None,
            free_shop_this_stage: false,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, State)]
pub struct UpgradeUpdateFlags(u8);

impl UpgradeUpdateFlags {
    pub const NONE: Self = Self(0);
    pub const TOWER_STATS: Self = Self(1 << 0);
    pub const CARD_OPTIONS: Self = Self(1 << 1);
    pub const RESOURCE: Self = Self(1 << 2);
    pub const PLAYER_STATS: Self = Self(1 << 3);
    pub const REVISION_REQUIRED: Self = Self(1 << 4);

    pub fn contains(&self, other: Self) -> bool {
        self.0 & other.0 == other.0
    }

    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    pub fn requires_revision(&self) -> bool {
        self.contains(Self::TOWER_STATS) || self.contains(Self::REVISION_REQUIRED)
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
pub trait UpgradeBehavior {
    fn apply_on_stage_start(&mut self, _stage: usize, _effects: &mut StageStartEffects) {}
    fn on_tower_placement(
        &mut self,
        _tower_template: &mut TowerTemplate,
        _left_dice: usize,
    ) -> usize {
        0
    }
    fn on_tower_placed(&mut self, _tower: &Tower) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        (TowerPlacementResult::default(), UpgradeUpdateFlags::NONE)
    }

    fn on_tower_removed(&mut self) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn tower_upgrade_damage_bonus(
        &self,
        _game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        None
    }

    fn on_monster_death(&mut self) -> bool {
        false
    }

    fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        self.apply_on_stage_start(stage, effects);
        UpgradeUpdateFlags::NONE
    }

    fn on_stage_end(
        &mut self,
        _perfect_clear: bool,
        _gold: usize,
        _item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        (0, UpgradeUpdateFlags::NONE)
    }

    fn on_stage_end_with_state(
        &mut self,
        _game_state: &GameState,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        self.on_stage_end(perfect_clear, gold, item_count)
    }

    fn max_hp_plus(&self) -> f32 {
        0.0
    }

    fn gold_earn_plus(&self) -> usize {
        0
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

    fn is_tower_damage_upgrade(&self) -> bool {
        false
    }

    fn on_upgrade_acquired(&self, _game_state: &GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_upgrade_acquired_mut(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.on_upgrade_acquired(game_state)
    }

    fn on_tower_placed_mut(
        &mut self,
        _game_state: &mut GameState,
        _tower: &Tower,
    ) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
    }

    fn on_item_bought(&mut self) -> UpgradeUpdateFlags {
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
}

#[derive(Clone, Copy)]
pub(super) struct UpgradeDefinition {
    generate: fn(&UpgradeState) -> Upgrade,
    current_and_max: fn(&UpgradeState) -> Option<(usize, usize)>,
}

impl UpgradeDefinition {
    pub(super) const fn new(
        generate: fn(&UpgradeState) -> Upgrade,
        current_and_max: fn(&UpgradeState) -> Option<(usize, usize)>,
    ) -> Self {
        Self {
            generate,
            current_and_max,
        }
    }

    pub(super) fn generate(self, upgrade_state: &UpgradeState) -> Upgrade {
        (self.generate)(upgrade_state)
    }

    pub(super) fn current_and_max(self, upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
        (self.current_and_max)(upgrade_state)
    }
}

pub(super) fn no_current_and_max(_upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
    None
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
mod eraser;
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
pub use eraser::*;
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
    Eraser(EraserUpgrade),
    BrokenPottery(BrokenPotteryUpgrade),
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
            UpgradeDiscriminants::Eraser => eraser::UPGRADE_DEFINITION,
            UpgradeDiscriminants::BrokenPottery => broken_pottery::UPGRADE_DEFINITION,
        }
    }

    pub(crate) fn current_and_max(self, upgrade_state: &UpgradeState) -> Option<(usize, usize)> {
        self.definition().current_and_max(upgrade_state)
    }

    pub(crate) fn generate(self, upgrade_state: &UpgradeState) -> Upgrade {
        self.definition().generate(upgrade_state)
    }
}

impl Upgrade {
    pub fn behavior_mut(&mut self) -> &mut dyn UpgradeBehavior {
        match self {
            Upgrade::Cat(upgrade) => upgrade,
            Upgrade::Staff(upgrade) => upgrade,
            Upgrade::LongSword(upgrade) => upgrade,
            Upgrade::Mace(upgrade) => upgrade,
            Upgrade::ClubSword(upgrade) => upgrade,
            Upgrade::Backpack(upgrade) => upgrade,
            Upgrade::DiceBundle(upgrade) => upgrade,
            Upgrade::Tricycle(upgrade) => upgrade,
            Upgrade::EnergyDrink(upgrade) => upgrade,
            Upgrade::PerfectPottery(upgrade) => upgrade,
            Upgrade::SingleChopstick(upgrade) => upgrade,
            Upgrade::PairChopsticks(upgrade) => upgrade,
            Upgrade::FountainPen(upgrade) => upgrade,
            Upgrade::Brush(upgrade) => upgrade,
            Upgrade::FourLeafClover(upgrade) => upgrade,
            Upgrade::Rabbit(upgrade) => upgrade,
            Upgrade::BlackWhite(upgrade) => upgrade,
            Upgrade::Trophy(upgrade) => upgrade,
            Upgrade::Crock(upgrade) => upgrade,
            Upgrade::DemolitionHammer(upgrade) => upgrade,
            Upgrade::Metronome(upgrade) => upgrade,
            Upgrade::Tape(upgrade) => upgrade,
            Upgrade::NameTag(upgrade) => upgrade,
            Upgrade::ShoppingBag(upgrade) => upgrade,
            Upgrade::Resolution(upgrade) => upgrade,
            Upgrade::Mirror(upgrade) => upgrade,
            Upgrade::IceCream(upgrade) => upgrade,
            Upgrade::Spanner(upgrade) => upgrade,
            Upgrade::Pea(upgrade) => upgrade,
            Upgrade::SlotMachine(upgrade) => upgrade,
            Upgrade::PiggyBank(upgrade) => upgrade,
            Upgrade::Camera(upgrade) => upgrade,
            Upgrade::GiftBox(upgrade) => upgrade,
            Upgrade::Fang(upgrade) => upgrade,
            Upgrade::Popcorn(upgrade) => upgrade,
            Upgrade::MembershipCard(upgrade) => upgrade,
            Upgrade::Eraser(upgrade) => upgrade,
            Upgrade::BrokenPottery(upgrade) => upgrade,
        }
    }

    pub fn behavior(&self) -> &dyn UpgradeBehavior {
        match self {
            Upgrade::Cat(upgrade) => upgrade,
            Upgrade::Staff(upgrade) => upgrade,
            Upgrade::LongSword(upgrade) => upgrade,
            Upgrade::Mace(upgrade) => upgrade,
            Upgrade::ClubSword(upgrade) => upgrade,
            Upgrade::Backpack(upgrade) => upgrade,
            Upgrade::DiceBundle(upgrade) => upgrade,
            Upgrade::Tricycle(upgrade) => upgrade,
            Upgrade::EnergyDrink(upgrade) => upgrade,
            Upgrade::PerfectPottery(upgrade) => upgrade,
            Upgrade::SingleChopstick(upgrade) => upgrade,
            Upgrade::PairChopsticks(upgrade) => upgrade,
            Upgrade::FountainPen(upgrade) => upgrade,
            Upgrade::Brush(upgrade) => upgrade,
            Upgrade::FourLeafClover(upgrade) => upgrade,
            Upgrade::Rabbit(upgrade) => upgrade,
            Upgrade::BlackWhite(upgrade) => upgrade,
            Upgrade::Trophy(upgrade) => upgrade,
            Upgrade::Crock(upgrade) => upgrade,
            Upgrade::DemolitionHammer(upgrade) => upgrade,
            Upgrade::Metronome(upgrade) => upgrade,
            Upgrade::Tape(upgrade) => upgrade,
            Upgrade::NameTag(upgrade) => upgrade,
            Upgrade::ShoppingBag(upgrade) => upgrade,
            Upgrade::Resolution(upgrade) => upgrade,
            Upgrade::Mirror(upgrade) => upgrade,
            Upgrade::IceCream(upgrade) => upgrade,
            Upgrade::Spanner(upgrade) => upgrade,
            Upgrade::Pea(upgrade) => upgrade,
            Upgrade::SlotMachine(upgrade) => upgrade,
            Upgrade::PiggyBank(upgrade) => upgrade,
            Upgrade::Camera(upgrade) => upgrade,
            Upgrade::GiftBox(upgrade) => upgrade,
            Upgrade::Fang(upgrade) => upgrade,
            Upgrade::Popcorn(upgrade) => upgrade,
            Upgrade::MembershipCard(upgrade) => upgrade,
            Upgrade::Eraser(upgrade) => upgrade,
            Upgrade::BrokenPottery(upgrade) => upgrade,
        }
    }

    pub fn as_mut(&mut self) -> &mut dyn UpgradeBehavior {
        self.behavior_mut()
    }

    pub fn as_ref(&self) -> &dyn UpgradeBehavior {
        self.behavior()
    }

    pub fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        self.behavior_mut().apply_on_stage_start(stage, effects);
    }

    pub fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        self.behavior_mut().on_stage_start(stage, effects)
    }

    pub fn on_tower_placement(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) -> usize {
        self.behavior_mut()
            .on_tower_placement(tower_template, left_dice)
    }

    pub fn on_tower_placed(&mut self, tower: &Tower) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        self.behavior_mut().on_tower_placed(tower)
    }

    pub fn clear_shield_on_stage_start(&self) -> bool {
        self.behavior().clear_shield_on_stage_start()
    }

    pub fn on_upgrade_acquired(&self, game_state: &GameState) -> UpgradeUpdateFlags {
        self.behavior().on_upgrade_acquired(game_state)
    }

    pub fn on_stage_end(
        &mut self,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        self.behavior_mut()
            .on_stage_end(perfect_clear, gold, item_count)
    }

    pub fn on_item_bought(&mut self) -> UpgradeUpdateFlags {
        self.behavior_mut().on_item_bought()
    }

    pub fn on_tower_removed(&mut self) -> UpgradeUpdateFlags {
        self.behavior_mut().on_tower_removed()
    }

    pub fn name_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::Name(self)
    }

    pub fn description_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::DescriptionUpgrade(self)
    }
}

impl UpgradeBehavior for Upgrade {
    fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        self.behavior_mut().apply_on_stage_start(stage, effects);
    }

    fn on_stage_start(
        &mut self,
        stage: usize,
        effects: &mut StageStartEffects,
    ) -> UpgradeUpdateFlags {
        self.behavior_mut().on_stage_start(stage, effects)
    }

    fn on_tower_placement(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) -> usize {
        self.behavior_mut()
            .on_tower_placement(tower_template, left_dice)
    }

    fn on_tower_placed(&mut self, tower: &Tower) -> (TowerPlacementResult, UpgradeUpdateFlags) {
        self.behavior_mut().on_tower_placed(tower)
    }

    fn on_tower_removed(&mut self) -> UpgradeUpdateFlags {
        self.behavior_mut().on_tower_removed()
    }

    fn tower_upgrade_damage_bonus(
        &self,
        game_state: &GameState,
    ) -> Option<(TowerUpgradeTarget, f32)> {
        self.behavior().tower_upgrade_damage_bonus(game_state)
    }

    fn on_item_bought(&mut self) -> UpgradeUpdateFlags {
        self.behavior_mut().on_item_bought()
    }

    fn on_tower_placed_mut(
        &mut self,
        game_state: &mut GameState,
        tower: &Tower,
    ) -> UpgradeUpdateFlags {
        self.behavior_mut().on_tower_placed_mut(game_state, tower)
    }

    fn on_upgrade_acquired(&self, game_state: &GameState) -> UpgradeUpdateFlags {
        self.behavior().on_upgrade_acquired(game_state)
    }

    fn on_upgrade_acquired_mut(&mut self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        self.behavior_mut().on_upgrade_acquired_mut(game_state)
    }

    fn on_monster_death(&mut self) -> bool {
        self.behavior_mut().on_monster_death()
    }

    fn on_stage_end(
        &mut self,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        self.behavior_mut()
            .on_stage_end(perfect_clear, gold, item_count)
    }

    fn on_stage_end_with_state(
        &mut self,
        game_state: &GameState,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) -> (usize, UpgradeUpdateFlags) {
        self.behavior_mut()
            .on_stage_end_with_state(game_state, perfect_clear, gold, item_count)
    }

    fn max_hp_plus(&self) -> f32 {
        self.behavior().max_hp_plus()
    }

    fn gold_earn_plus(&self) -> usize {
        self.behavior().gold_earn_plus()
    }

    fn shop_slot_expand(&self) -> usize {
        self.behavior().shop_slot_expand()
    }

    fn dice_chance_plus(&self) -> usize {
        self.behavior().dice_chance_plus()
    }

    fn shop_item_price_minus(&self) -> usize {
        self.behavior().shop_item_price_minus()
    }

    fn shorten_straight_flush_to_4_cards(&self) -> bool {
        self.behavior().shorten_straight_flush_to_4_cards()
    }

    fn skip_rank_for_straight(&self) -> bool {
        self.behavior().skip_rank_for_straight()
    }

    fn treat_suits_as_same(&self) -> bool {
        self.behavior().treat_suits_as_same()
    }

    fn removed_number_rank_count(&self) -> usize {
        self.behavior().removed_number_rank_count()
    }

    fn is_tower_damage_upgrade(&self) -> bool {
        self.behavior().is_tower_damage_upgrade()
    }

    fn clear_shield_on_stage_start(&self) -> bool {
        self.behavior().clear_shield_on_stage_start()
    }

    fn l10n_name<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        self.behavior().l10n_name(builder, locale)
    }

    fn l10n_description<'a>(
        &self,
        builder: &mut crate::theme::typography::TypographyBuilder<'a>,
        locale: &crate::l10n::Locale,
    ) {
        self.behavior().l10n_description(builder, locale)
    }
}
