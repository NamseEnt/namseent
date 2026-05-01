use super::*;
use crate::game_state::GameState;
use crate::game_state::tower::{
    Tower, TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind, TowerTemplate,
};
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

/// Common trait for all upgrade behaviors
pub trait UpgradeBehavior {
    fn apply_on_stage_start(&mut self, _stage: usize, _effects: &mut StageStartEffects) {}
    fn record_perfect_clear(&mut self) {}
    fn record_tower_removed(&mut self) {}
    fn apply_pending_placement_bonuses(
        &mut self,
        _tower_template: &mut TowerTemplate,
        _left_dice: usize,
    ) {
    }
    fn consume_pending_mirror_count(&mut self) -> usize {
        0
    }
    fn on_tower_placed(&mut self, _tower: &Tower) -> TowerPlacementResult {
        TowerPlacementResult::default()
    }
    fn get_global_damage_multiplier(&self, _game_state: &GameState) -> Option<f32> {
        None
    }

    fn on_monster_death(&mut self) -> bool {
        false
    }

    fn on_stage_end(&mut self, _gold: usize, _item_count: usize) -> usize {
        0
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

    fn clear_shield_on_stage_start(&self) -> bool {
        true
    }
}

mod card_rules;
mod damage;
mod simple_flags;
mod stateful;
mod treasure;

pub use card_rules::*;
pub use damage::*;
pub use simple_flags::*;
pub use stateful::*;
pub use treasure::*;

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

impl CatUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Cat(CatUpgrade { add })
    }
}

impl StaffUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::Staff(StaffUpgrade { damage_multiplier })
    }
}

impl LongSwordUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::LongSword(LongSwordUpgrade { damage_multiplier })
    }
}

impl MaceUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::Mace(MaceUpgrade { damage_multiplier })
    }
}

impl ClubSwordUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::ClubSword(ClubSwordUpgrade { damage_multiplier })
    }
}

impl BackpackUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Backpack(BackpackUpgrade { add })
    }
}

impl DiceBundleUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::DiceBundle(DiceBundleUpgrade { add })
    }
}

impl TricycleUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::Tricycle(TricycleUpgrade { damage_multiplier })
    }
}

impl EnergyDrinkUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::EnergyDrink(EnergyDrinkUpgrade { add })
    }
}

impl PerfectPotteryUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::PerfectPottery(PerfectPotteryUpgrade { damage_multiplier })
    }
}

impl SingleChopstickUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::SingleChopstick(SingleChopstickUpgrade { damage_multiplier })
    }
}

impl PairChopsticksUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::PairChopsticks(PairChopsticksUpgrade { damage_multiplier })
    }
}

impl FountainPenUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::FountainPen(FountainPenUpgrade { damage_multiplier })
    }
}

impl BrushUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::Brush(BrushUpgrade { damage_multiplier })
    }
}

impl FourLeafCloverUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::FourLeafClover(FourLeafCloverUpgrade)
    }
}

impl RabbitUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Rabbit(RabbitUpgrade)
    }
}

impl BlackWhiteUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::BlackWhite(BlackWhiteUpgrade)
    }
}

impl TrophyUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Trophy(TrophyUpgrade {
            perfect_clear_stacks: 0,
        })
    }
}

impl CrockUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Crock(CrockUpgrade)
    }
}

impl DemolitionHammerUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::DemolitionHammer(DemolitionHammerUpgrade {
            damage_multiplier,
            removed_tower_count: 0,
        })
    }
}

impl MetronomeUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Metronome(MetronomeUpgrade { start_stage: None })
    }
}

impl TapeUpgrade {
    pub fn into_upgrade(acquired_stage: usize) -> Upgrade {
        Upgrade::Tape(TapeUpgrade { acquired_stage })
    }
}

impl NameTagUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::NameTag(NameTagUpgrade {
            damage_multiplier,
            pending: true,
        })
    }
}

impl ShoppingBagUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::ShoppingBag(ShoppingBagUpgrade {
            damage_multiplier,
            stacks: 0,
        })
    }
}

impl ResolutionUpgrade {
    pub fn into_upgrade(damage_multiplier_per_reroll: f32) -> Upgrade {
        Upgrade::Resolution(ResolutionUpgrade {
            damage_multiplier_per_reroll,
            pending: true,
        })
    }
}

impl MirrorUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Mirror(MirrorUpgrade { pending: true })
    }
}

impl IceCreamUpgrade {
    pub fn into_upgrade(damage_multiplier: f32, waves_remaining: usize) -> Upgrade {
        Upgrade::IceCream(IceCreamUpgrade {
            damage_multiplier,
            waves_remaining,
        })
    }
}

impl SpannerUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Spanner(SpannerUpgrade)
    }
}

impl PeaUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Pea(PeaUpgrade)
    }
}

impl SlotMachineUpgrade {
    pub fn into_upgrade(next_round_dice: usize) -> Upgrade {
        Upgrade::SlotMachine(SlotMachineUpgrade { next_round_dice })
    }
}

impl PiggyBankUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::PiggyBank(PiggyBankUpgrade)
    }
}

impl CameraUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Camera(CameraUpgrade)
    }
}

impl GiftBoxUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::GiftBox(GiftBoxUpgrade)
    }
}

impl FangUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::Fang(FangUpgrade)
    }
}

impl PopcornUpgrade {
    pub fn into_upgrade(max_multiplier: f32, duration: usize, waves_remaining: usize) -> Upgrade {
        Upgrade::Popcorn(PopcornUpgrade {
            max_multiplier,
            duration,
            waves_remaining,
        })
    }
}

impl MembershipCardUpgrade {
    pub fn into_upgrade() -> Upgrade {
        Upgrade::MembershipCard(MembershipCardUpgrade {
            pending_free_shop: true,
        })
    }
}

impl EraserUpgrade {
    pub fn into_upgrade(add: usize) -> Upgrade {
        Upgrade::Eraser(EraserUpgrade { add })
    }
}

impl BrokenPotteryUpgrade {
    pub fn into_upgrade(damage_multiplier: f32) -> Upgrade {
        Upgrade::BrokenPottery(BrokenPotteryUpgrade { damage_multiplier })
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

    pub fn on_tower_placed(&mut self, tower: &Tower) -> TowerPlacementResult {
        self.behavior_mut().on_tower_placed(tower)
    }

    pub fn clear_shield_on_stage_start(&self) -> bool {
        self.behavior().clear_shield_on_stage_start()
    }

    pub fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) {
        self.behavior_mut()
            .apply_pending_placement_bonuses(tower_template, left_dice);
    }

    pub fn consume_pending_mirror_count(&mut self) -> usize {
        self.behavior_mut().consume_pending_mirror_count()
    }

    pub fn get_global_damage_multiplier(&self, game_state: &GameState) -> Option<f32> {
        self.behavior().get_global_damage_multiplier(game_state)
    }

    pub fn record_perfect_clear(&mut self) {
        self.behavior_mut().record_perfect_clear();
    }

    pub fn record_tower_removed(&mut self) {
        self.behavior_mut().record_tower_removed();
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

    fn record_perfect_clear(&mut self) {
        self.behavior_mut().record_perfect_clear();
    }

    fn record_tower_removed(&mut self) {
        self.behavior_mut().record_tower_removed();
    }

    fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) {
        self.behavior_mut()
            .apply_pending_placement_bonuses(tower_template, left_dice);
    }

    fn consume_pending_mirror_count(&mut self) -> usize {
        self.behavior_mut().consume_pending_mirror_count()
    }

    fn on_tower_placed(&mut self, tower: &Tower) -> TowerPlacementResult {
        self.behavior_mut().on_tower_placed(tower)
    }

    fn get_global_damage_multiplier(&self, game_state: &GameState) -> Option<f32> {
        self.behavior().get_global_damage_multiplier(game_state)
    }

    fn on_monster_death(&mut self) -> bool {
        self.behavior_mut().on_monster_death()
    }

    fn on_stage_end(&mut self, gold: usize, item_count: usize) -> usize {
        self.behavior_mut().on_stage_end(gold, item_count)
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
}
