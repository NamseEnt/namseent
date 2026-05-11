use super::state::UpgradeState;
use super::*;
use crate::game_state::GameState;
use crate::game_state::tower::Tower;
use crate::game_state::upgrade::tower::TowerUpgradeTarget;
use crate::*;
use enum_dispatch::enum_dispatch;

// ============================================================================
// Upgrade Trait and Structs
// ============================================================================

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, State)]
pub struct UpgradeUpdateFlags(u8);

impl UpgradeUpdateFlags {
    pub const NONE: Self = Self(0);
    pub const TOWER_STATS: Self = Self(1 << 0);
    pub const CARD_OPTIONS: Self = Self(1 << 1);
    pub const RESOURCE: Self = Self(1 << 2);
    pub const PLAYER_STATS: Self = Self(1 << 3);
    pub const REVISION_REQUIRED: Self = Self(1 << 4);
    pub const HEAL_TO_FULL: Self = Self(1 << 5);

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
#[enum_dispatch]
pub trait UpgradeBehavior {
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

    fn on_monster_death(&mut self, _game_state: &mut GameState) -> bool {
        false
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

    fn on_upgrade_acquired(self, game_state: &mut GameState) -> UpgradeUpdateFlags
    where
        Self: Sized,
        Upgrade: From<Self>,
    {
        merge_for_acquire(game_state, self.into())
    }

    fn merge_for_acquire(&mut self, _incoming: Upgrade) -> bool {
        false
    }

    fn on_upgrade_acquired_effect(&mut self, _game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeUpdateFlags::NONE
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
}

pub(super) fn merge_for_acquire(
    game_state: &mut GameState,
    incoming: Upgrade,
) -> UpgradeUpdateFlags {
    let index = {
        let upgrades = &mut game_state.upgrade_state.upgrades;
        let mut merged_index = None;
        for (index, upgrade) in upgrades.iter_mut().enumerate() {
            if upgrade.merge_for_acquire(incoming) {
                merged_index = Some(index);
                break;
            }
        }

        if let Some(index) = merged_index {
            index
        } else {
            upgrades.push(incoming);
            upgrades.len() - 1
        }
    };

    let mut target_upgrade = game_state.upgrade_state.upgrades.remove(index);
    let flags = target_upgrade.on_upgrade_acquired_effect(game_state)
        | UpgradeUpdateFlags::REVISION_REQUIRED;
    game_state
        .upgrade_state
        .upgrades
        .insert(index, target_upgrade);
    flags
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
    pub fn on_upgrade_acquired(self, game_state: &mut GameState) -> UpgradeUpdateFlags {
        UpgradeBehavior::on_upgrade_acquired(self, game_state)
    }

    pub fn name_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::Name(self)
    }

    pub fn description_text(&self) -> crate::l10n::upgrade::UpgradeTypeText<'_> {
        crate::l10n::upgrade::UpgradeTypeText::DescriptionUpgrade(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_state::{GameStateAction, create_initial_game_state};

    fn damage_bonus_pct(upgrade: &Upgrade) -> f32 {
        match upgrade {
            Upgrade::Staff(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::LongSword(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::Mace(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::ClubSword(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::Tricycle(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::SingleChopstick(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::PairChopsticks(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::FountainPen(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::Brush(upgrade) => upgrade.damage_bonus_pct,
            Upgrade::BrokenPottery(upgrade) => upgrade.damage_bonus_pct,
            _ => panic!("unsupported upgrade for merge damage check"),
        }
    }

    #[test]
    fn merge_for_acquire_merges_same_upgrade_types() {
        let cases = vec![
            (
                Upgrade::Staff(StaffUpgrade {
                    damage_bonus_pct: 0.1,
                }),
                Upgrade::Staff(StaffUpgrade {
                    damage_bonus_pct: 0.2,
                }),
                0.3,
            ),
            (
                Upgrade::LongSword(LongSwordUpgrade {
                    damage_bonus_pct: 0.15,
                }),
                Upgrade::LongSword(LongSwordUpgrade {
                    damage_bonus_pct: 0.05,
                }),
                0.2,
            ),
            (
                Upgrade::Mace(MaceUpgrade {
                    damage_bonus_pct: 0.12,
                }),
                Upgrade::Mace(MaceUpgrade {
                    damage_bonus_pct: 0.18,
                }),
                0.3,
            ),
            (
                Upgrade::ClubSword(ClubSwordUpgrade {
                    damage_bonus_pct: 0.25,
                }),
                Upgrade::ClubSword(ClubSwordUpgrade {
                    damage_bonus_pct: 0.25,
                }),
                0.5,
            ),
            (
                Upgrade::Tricycle(TricycleUpgrade {
                    damage_bonus_pct: 0.4,
                }),
                Upgrade::Tricycle(TricycleUpgrade {
                    damage_bonus_pct: 0.2,
                }),
                0.6,
            ),
            (
                Upgrade::SingleChopstick(SingleChopstickUpgrade {
                    damage_bonus_pct: 0.3,
                }),
                Upgrade::SingleChopstick(SingleChopstickUpgrade {
                    damage_bonus_pct: 0.2,
                }),
                0.5,
            ),
            (
                Upgrade::PairChopsticks(PairChopsticksUpgrade {
                    damage_bonus_pct: 0.2,
                }),
                Upgrade::PairChopsticks(PairChopsticksUpgrade {
                    damage_bonus_pct: 0.3,
                }),
                0.5,
            ),
            (
                Upgrade::FountainPen(FountainPenUpgrade {
                    damage_bonus_pct: 0.15,
                }),
                Upgrade::FountainPen(FountainPenUpgrade {
                    damage_bonus_pct: 0.35,
                }),
                0.5,
            ),
            (
                Upgrade::Brush(BrushUpgrade {
                    damage_bonus_pct: 0.22,
                }),
                Upgrade::Brush(BrushUpgrade {
                    damage_bonus_pct: 0.28,
                }),
                0.5,
            ),
            (
                Upgrade::BrokenPottery(BrokenPotteryUpgrade {
                    damage_bonus_pct: 0.1,
                }),
                Upgrade::BrokenPottery(BrokenPotteryUpgrade {
                    damage_bonus_pct: 0.2,
                }),
                0.3,
            ),
        ];

        for (first, second, expected) in cases {
            let mut game_state = create_initial_game_state();
            game_state.action(GameStateAction::Upgrade(first, None));
            game_state.action(GameStateAction::Upgrade(second, None));

            assert_eq!(game_state.upgrade_state.upgrades.len(), 1);
            let merged = &game_state.upgrade_state.upgrades[0];
            assert!((damage_bonus_pct(merged) - expected).abs() < f32::EPSILON);
        }
    }
}
