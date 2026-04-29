mod display;
mod generation;
mod thumbnail;

use crate::{
    card::Suit,
    game_state::{
        GameState,
        tower::{
            Tower, TowerStatusEffect, TowerStatusEffectEnd, TowerStatusEffectKind, TowerTemplate,
        },
    },
    *,
};
pub use generation::*;

pub const MAX_GOLD_EARN_PLUS: usize = 16;
pub const MAX_SHOP_SLOT_EXPAND: usize = 2;
pub const MAX_DICE_CHANCE_PLUS: usize = 4;
pub const MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE: usize = 15;
pub const MAX_REMOVE_NUMBER_RANKS: usize = 5;
const TROPHY_DAMAGE_MULTIPLIER: f32 = 2.0;

// ============================================================================
// Upgrade Trait and Structs
// ============================================================================

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
    fn get_global_damage_multiplier(&self, _game_state: &GameState) -> Option<f32> {
        None
    }
}

// Treasure upgrades (simple gold/shop bonuses)
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CatUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for CatUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BackpackUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for BackpackUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DiceBundleUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for DiceBundleUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EnergyDrinkUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for EnergyDrinkUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct EraserUpgrade {
    pub add: usize,
}
impl UpgradeBehavior for EraserUpgrade {}

// Card hand rule modifiers
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FourLeafCloverUpgrade;
impl UpgradeBehavior for FourLeafCloverUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct RabbitUpgrade;
impl UpgradeBehavior for RabbitUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BlackWhiteUpgrade;
impl UpgradeBehavior for BlackWhiteUpgrade {}

// Simple flags
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CrockUpgrade;
impl UpgradeBehavior for CrockUpgrade {
    fn get_global_damage_multiplier(&self, game_state: &GameState) -> Option<f32> {
        Some((game_state.gold / 1000) as f32)
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SpannerUpgrade;
impl UpgradeBehavior for SpannerUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PeaUpgrade;
impl UpgradeBehavior for PeaUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PiggyBankUpgrade;
impl UpgradeBehavior for PiggyBankUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct CameraUpgrade;
impl UpgradeBehavior for CameraUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct GiftBoxUpgrade;
impl UpgradeBehavior for GiftBoxUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FangUpgrade;
impl UpgradeBehavior for FangUpgrade {}

// Tower damage multipliers (suit-based and others)
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct StaffUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for StaffUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct LongSwordUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for LongSwordUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MaceUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for MaceUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ClubSwordUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for ClubSwordUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SingleChopstickUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for SingleChopstickUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PairChopsticksUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for PairChopsticksUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct FountainPenUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for FountainPenUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrushUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for BrushUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct BrokenPotteryUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for BrokenPotteryUpgrade {}

// Tower select upgrades
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TricycleUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for TricycleUpgrade {}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PerfectPotteryUpgrade {
    pub damage_multiplier: f32,
}
impl UpgradeBehavior for PerfectPotteryUpgrade {}

// Stateful upgrades with stage-based effects
#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MetronomeUpgrade {
    pub start_stage: Option<usize>,
}
impl UpgradeBehavior for MetronomeUpgrade {
    fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        let start = self.start_stage.get_or_insert(stage);
        if stage >= *start && (stage - *start).is_multiple_of(2) {
            effects.extra_dice += 1;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TapeUpgrade {
    pub acquired_stage: usize,
}
impl UpgradeBehavior for TapeUpgrade {
    fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        if stage > self.acquired_stage && (stage - self.acquired_stage - 1).is_multiple_of(4) {
            effects.enemy_speed_multiplier = Some(0.75);
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct DemolitionHammerUpgrade {
    pub damage_multiplier: f32,
    pub removed_tower_count: usize,
}
impl UpgradeBehavior for DemolitionHammerUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.removed_tower_count > 0 {
            effects.damage_multiplier *=
                1.0 + self.damage_multiplier * self.removed_tower_count as f32;
            self.removed_tower_count = 0;
        }
    }

    fn record_tower_removed(&mut self) {
        self.removed_tower_count += 1;
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct TrophyUpgrade {
    pub perfect_clear_stacks: usize,
}
impl UpgradeBehavior for TrophyUpgrade {
    fn record_perfect_clear(&mut self) {
        self.perfect_clear_stacks += 1;
    }

    fn get_global_damage_multiplier(&self, _game_state: &GameState) -> Option<f32> {
        if self.perfect_clear_stacks > 0 {
            Some(self.perfect_clear_stacks as f32 * (TROPHY_DAMAGE_MULTIPLIER - 1.0))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ShoppingBagUpgrade {
    pub damage_multiplier: f32,
    pub stacks: usize,
}
impl UpgradeBehavior for ShoppingBagUpgrade {
    fn get_global_damage_multiplier(&self, _game_state: &GameState) -> Option<f32> {
        if self.stacks > 0 {
            Some(self.stacks as f32 * (self.damage_multiplier - 1.0))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct NameTagUpgrade {
    pub damage_multiplier: f32,
    pub pending: bool,
}
impl UpgradeBehavior for NameTagUpgrade {
    fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        _left_dice: usize,
    ) {
        if self.pending {
            tower_template
                .default_status_effects
                .push(TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageMul {
                        mul: self.damage_multiplier,
                    },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                });
            self.pending = false;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct ResolutionUpgrade {
    pub damage_multiplier_per_reroll: f32,
    pub pending: bool,
}
impl UpgradeBehavior for ResolutionUpgrade {
    fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) {
        if self.pending {
            let multiplier = 1.0 + left_dice as f32 * self.damage_multiplier_per_reroll;
            tower_template
                .default_status_effects
                .push(TowerStatusEffect {
                    kind: TowerStatusEffectKind::DamageMul { mul: multiplier },
                    end_at: TowerStatusEffectEnd::NeverEnd,
                });
            self.pending = false;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MirrorUpgrade {
    pub pending: bool,
}
impl UpgradeBehavior for MirrorUpgrade {
    fn consume_pending_mirror_count(&mut self) -> usize {
        if self.pending {
            self.pending = false;
            1
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct IceCreamUpgrade {
    pub damage_multiplier: f32,
    pub waves_remaining: usize,
}
impl UpgradeBehavior for IceCreamUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.waves_remaining > 0 {
            effects.damage_multiplier *= self.damage_multiplier;
            self.waves_remaining -= 1;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct SlotMachineUpgrade {
    pub next_round_dice: usize,
}
impl UpgradeBehavior for SlotMachineUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.next_round_dice > 0 {
            effects.extra_dice += self.next_round_dice;
            self.next_round_dice = 0;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct PopcornUpgrade {
    pub max_multiplier: f32,
    pub duration: usize,
    pub waves_remaining: usize,
}
impl UpgradeBehavior for PopcornUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.waves_remaining > 0 {
            let duration = self.duration.max(1);
            let elapsed = duration.saturating_sub(self.waves_remaining);
            let popcorn_multiplier = if duration <= 1 {
                self.max_multiplier
            } else {
                let step = (self.max_multiplier - 1.0) / (duration - 1) as f32;
                (self.max_multiplier - step * elapsed as f32).max(1.0)
            };

            effects.damage_multiplier *= popcorn_multiplier;
            self.waves_remaining -= 1;
        }
    }
}

#[derive(Debug, Clone, Copy, State, PartialEq)]
pub struct MembershipCardUpgrade {
    pub pending_free_shop: bool,
}
impl UpgradeBehavior for MembershipCardUpgrade {
    fn apply_on_stage_start(&mut self, _stage: usize, effects: &mut StageStartEffects) {
        if self.pending_free_shop {
            effects.free_shop_this_stage = true;
            self.pending_free_shop = false;
        }
    }
}

#[derive(Debug, Clone, State, Default)]
pub struct UpgradeState {
    pub upgrades: Vec<Upgrade>,
    pub revision: usize,
}

#[derive(Debug, Clone, Copy, State)]
pub struct Upgrade {
    pub kind: UpgradeKind,
    pub value: crate::OneZero,
}

impl UpgradeState {
    pub fn upgrade(&mut self, upgrade: Upgrade) {
        self.upgrades.push(upgrade);
        self.revision = self.revision.wrapping_add(1);
    }

    pub fn gold_earn_plus(&self) -> usize {
        self.sum_upgrade_field(|upgrade| {
            if let UpgradeKind::Cat(u) = upgrade.kind {
                Some(u.add)
            } else {
                None
            }
        })
        .min(MAX_GOLD_EARN_PLUS)
    }

    pub fn shop_slot_expand(&self) -> usize {
        self.sum_upgrade_field(|upgrade| {
            if let UpgradeKind::Backpack(u) = upgrade.kind {
                Some(u.add)
            } else {
                None
            }
        })
        .min(MAX_SHOP_SLOT_EXPAND)
    }

    pub fn dice_chance_plus(&self) -> usize {
        self.sum_upgrade_field(|upgrade| {
            if let UpgradeKind::DiceBundle(u) = upgrade.kind {
                Some(u.add)
            } else {
                None
            }
        })
        .min(MAX_DICE_CHANCE_PLUS)
    }

    pub fn shop_item_price_minus(&self) -> usize {
        self.sum_upgrade_field(|upgrade| {
            if let UpgradeKind::EnergyDrink(u) = upgrade.kind {
                Some(u.add)
            } else {
                None
            }
        })
        .min(MAX_SHOP_ITEM_PRICE_MINUS_UPGRADE)
    }

    pub fn removed_number_rank_count(&self) -> usize {
        self.sum_upgrade_field(|upgrade| {
            if let UpgradeKind::Eraser(u) = upgrade.kind {
                Some(u.add)
            } else {
                None
            }
        })
        .min(MAX_REMOVE_NUMBER_RANKS)
    }

    /// Helper to sum a field from matching upgrades
    #[inline]
    fn sum_upgrade_field<F>(&self, extractor: F) -> usize
    where
        F: Fn(&Upgrade) -> Option<usize>,
    {
        self.upgrades.iter().filter_map(extractor).sum()
    }

    pub fn shorten_straight_flush_to_4_cards(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::FourLeafClover(_)))
    }

    pub fn skip_rank_for_straight(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::Rabbit(_)))
    }

    pub fn treat_suits_as_same(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::BlackWhite(_)))
    }

    pub fn stage_start_effects(&mut self, stage: usize) -> StageStartEffects {
        let mut effects = StageStartEffects {
            damage_multiplier: 1.0,
            extra_dice: 0,
            enemy_speed_multiplier: None,
            free_shop_this_stage: false,
        };

        for upgrade in &mut self.upgrades {
            upgrade.kind.apply_on_stage_start(stage, &mut effects);
        }

        effects
    }

    pub fn record_perfect_clear(&mut self) {
        let mut changed = false;
        for upgrade in &mut self.upgrades {
            if let UpgradeKind::Trophy(ref mut u) = upgrade.kind {
                u.record_perfect_clear();
                changed = true;
            }
        }
        if changed {
            self.revision = self.revision.wrapping_add(1);
        }
    }

    pub fn record_tower_removed(&mut self) {
        for upgrade in &mut self.upgrades {
            if let UpgradeKind::DemolitionHammer(ref mut u) = upgrade.kind {
                u.record_tower_removed();
            }
        }
    }

    pub fn tower_upgrades(&self, tower: &Tower) -> Vec<TowerUpgradeState> {
        [
            TowerUpgradeTarget::Suit { suit: tower.suit },
            TowerUpgradeTarget::EvenOdd {
                even: tower.rank.is_even(),
            },
            TowerUpgradeTarget::FaceNumber {
                face: tower.rank.is_face(),
            },
        ]
        .iter()
        .map(|target| self.tower_upgrade_state(*target))
        .collect::<Vec<_>>()
    }

    pub fn tower_upgrade_state(&self, target: TowerUpgradeTarget) -> TowerUpgradeState {
        let mut state = TowerUpgradeState::default();
        for upgrade in &self.upgrades {
            let maybe_upgrade = match upgrade.kind {
                UpgradeKind::Staff(u)
                    if target
                        == TowerUpgradeTarget::Suit {
                            suit: Suit::Diamonds,
                        } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::LongSword(u)
                    if target == TowerUpgradeTarget::Suit { suit: Suit::Spades } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::Mace(u)
                    if target == TowerUpgradeTarget::Suit { suit: Suit::Hearts } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::ClubSword(u)
                    if target == TowerUpgradeTarget::Suit { suit: Suit::Clubs } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::SingleChopstick(u)
                    if target == TowerUpgradeTarget::EvenOdd { even: false } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::PairChopsticks(u)
                    if target == TowerUpgradeTarget::EvenOdd { even: true } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::FountainPen(u)
                    if target == TowerUpgradeTarget::FaceNumber { face: false } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::Brush(u)
                    if target == TowerUpgradeTarget::FaceNumber { face: true } =>
                {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                _ => None,
            };
            if let Some(tower_upgrade) = maybe_upgrade {
                state.apply_upgrade(tower_upgrade);
            }
        }
        state
    }

    pub fn tower_select_upgrade_state(
        &self,
        target: TowerSelectUpgradeTarget,
    ) -> TowerUpgradeState {
        let mut state = TowerUpgradeState::default();
        for upgrade in &self.upgrades {
            let maybe_upgrade = match upgrade.kind {
                UpgradeKind::Tricycle(u) if target == TowerSelectUpgradeTarget::LowCard => {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::PerfectPottery(u) if target == TowerSelectUpgradeTarget::NoReroll => {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                UpgradeKind::BrokenPottery(u) if target == TowerSelectUpgradeTarget::Reroll => {
                    Some(TowerUpgrade::DamageMultiplier {
                        multiplier: u.damage_multiplier,
                    })
                }
                _ => None,
            };
            if let Some(tower_upgrade) = maybe_upgrade {
                state.apply_upgrade(tower_upgrade);
            }
        }
        state
    }

    pub fn apply_pending_placement_bonuses(
        &mut self,
        tower_template: &mut TowerTemplate,
        left_dice: usize,
    ) {
        for upgrade in &mut self.upgrades {
            match &mut upgrade.kind {
                UpgradeKind::NameTag(u) => {
                    u.apply_pending_placement_bonuses(tower_template, left_dice);
                }
                UpgradeKind::Resolution(u) => {
                    u.apply_pending_placement_bonuses(tower_template, left_dice);
                }
                _ => {}
            }
        }
    }

    pub fn consume_pending_mirror_count(&mut self) -> usize {
        let mut count = 0;
        for upgrade in &mut self.upgrades {
            if let UpgradeKind::Mirror(ref mut u) = upgrade.kind {
                count += u.consume_pending_mirror_count();
            }
        }
        count
    }

    pub fn has_spanner(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::Spanner(_)))
    }

    pub fn pea_max_hp_plus(&self) -> usize {
        self.upgrades
            .iter()
            .filter(|upgrade| matches!(upgrade.kind, UpgradeKind::Pea(_)))
            .count()
            * 10
    }

    pub fn has_piggy_bank(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::PiggyBank(_)))
    }

    pub fn has_camera(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::Camera(_)))
    }

    pub fn has_gift_box(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::GiftBox(_)))
    }

    pub fn has_fang(&self) -> bool {
        self.upgrades
            .iter()
            .any(|upgrade| matches!(upgrade.kind, UpgradeKind::Fang(_)))
    }

    pub fn global_tower_damage_multiplier(&self, game_state: &GameState) -> f32 {
        self.upgrades.iter().fold(1.0_f32, |mul, upgrade| {
            mul + match &upgrade.kind {
                UpgradeKind::Crock(u) => u.get_global_damage_multiplier(game_state),
                UpgradeKind::Trophy(u) => u.get_global_damage_multiplier(game_state),
                UpgradeKind::ShoppingBag(u) => u.get_global_damage_multiplier(game_state),
                _ => None,
            }
            .unwrap_or(0.0)
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct StageStartEffects {
    pub damage_multiplier: f32,
    pub extra_dice: usize,
    pub enemy_speed_multiplier: Option<f32>,
    pub free_shop_this_stage: bool,
}

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Copy, State, PartialEq, strum_macros::EnumDiscriminants)]
#[strum_discriminants(
    derive(
        strum_macros::EnumIter,
        strum_macros::AsRefStr,
        strum_macros::EnumString
    ),
    name(UpgradeKindDiscriminants)
)]
pub enum UpgradeKind {
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

impl UpgradeKind {
    pub fn apply_on_stage_start(&mut self, stage: usize, effects: &mut StageStartEffects) {
        match self {
            UpgradeKind::Cat(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Staff(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::LongSword(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Mace(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::ClubSword(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Backpack(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::DiceBundle(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Tricycle(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::EnergyDrink(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::PerfectPottery(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::SingleChopstick(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::PairChopsticks(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::FountainPen(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Brush(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::FourLeafClover(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Rabbit(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::BlackWhite(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Trophy(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Crock(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::DemolitionHammer(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Metronome(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Tape(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::NameTag(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::ShoppingBag(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Resolution(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Mirror(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::IceCream(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Spanner(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Pea(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::SlotMachine(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::PiggyBank(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Camera(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::GiftBox(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Fang(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Popcorn(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::MembershipCard(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::Eraser(u) => u.apply_on_stage_start(stage, effects),
            UpgradeKind::BrokenPottery(u) => u.apply_on_stage_start(stage, effects),
        }
    }

    pub fn is_tower_damage_upgrade(&self) -> bool {
        matches!(
            self,
            UpgradeKind::Staff(_)
                | UpgradeKind::LongSword(_)
                | UpgradeKind::Mace(_)
                | UpgradeKind::ClubSword(_)
                | UpgradeKind::Tricycle(_)
                | UpgradeKind::PerfectPottery(_)
                | UpgradeKind::SingleChopstick(_)
                | UpgradeKind::PairChopsticks(_)
                | UpgradeKind::FountainPen(_)
                | UpgradeKind::Brush(_)
                | UpgradeKind::BrokenPottery(_)
        )
    }

    pub fn is_treasure_upgrade(&self) -> bool {
        !self.is_tower_damage_upgrade()
    }

    pub fn name_text(&self) -> crate::l10n::upgrade::UpgradeKindText<'_> {
        crate::l10n::upgrade::UpgradeKindText::Name(self)
    }

    pub fn description_text(&self) -> crate::l10n::upgrade::UpgradeKindText<'_> {
        crate::l10n::upgrade::UpgradeKindText::Description(self)
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord, State)]
pub enum TowerUpgradeTarget {
    Suit { suit: Suit },
    EvenOdd { even: bool },
    FaceNumber { face: bool },
}
#[derive(Debug, Clone, Copy, State)]
pub enum TowerUpgrade {
    DamageMultiplier { multiplier: f32 },
}
#[derive(Debug, Clone, Copy, State)]
pub struct TowerUpgradeState {
    pub damage_multiplier: f32,
}
impl TowerUpgradeState {
    fn apply_upgrade(&mut self, upgrade: TowerUpgrade) {
        match upgrade {
            TowerUpgrade::DamageMultiplier { multiplier } => self.damage_multiplier *= multiplier,
        }
    }
}
impl Default for TowerUpgradeState {
    fn default() -> Self {
        TowerUpgradeState {
            damage_multiplier: 1.0,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord, State)]
pub enum TowerSelectUpgradeTarget {
    LowCard,
    NoReroll,
    Reroll,
}
