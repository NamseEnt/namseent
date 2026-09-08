pub(super) mod apple;
pub(super) mod backpack;
pub(super) mod banana;
pub(super) mod black_white;
pub(super) mod broken_pottery;
pub(super) mod camera;
pub(super) mod carrot;
pub(super) mod cat;
pub(super) mod crock;
pub(super) mod cup_noodles;
pub(super) mod demolition_hammer;
pub(super) mod dice_bundle;
pub(super) mod energy_drink;
pub(super) mod fang;
pub(super) mod four_leaf_clover;
pub(super) mod french_fries;
pub(super) mod gift_box;
pub(super) mod hamburger;
pub(super) mod ice_cream;
pub(super) mod membership_card;
pub(super) mod metronome;
pub(super) mod mirror;
pub(super) mod name_tag;
pub(super) mod pea;
pub(super) mod perfect_pottery;
pub(super) mod piggy_bank;
pub(super) mod pizza;
pub(super) mod popcorn;
pub(super) mod rabbit;
pub(super) mod resolution;
pub(super) mod shopping_bag;
pub(super) mod slot_machine;
pub(super) mod spanner;
pub(super) mod strawberry;
pub(crate) mod support;
pub(super) mod tape;
pub(super) mod trophy;
pub(super) mod watermelon;

use super::{
    UpgradeAcquireRecovery, UpgradeCacheContribution, UpgradeEntry, UpgradeTriggerContext,
};
use crate::{CoreState, TowerState, TowerTemplateState};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait UpgradeBehavior {
    fn kind(&self) -> crate::UpgradeKind;
    fn rarity(&self) -> crate::Rarity;
    fn generate(&self) -> super::UpgradeRuntimeState;

    fn cache(&self, _: &UpgradeEntry) -> UpgradeCacheContribution {
        UpgradeCacheContribution {
            clear_shield_on_stage_start: true,
            ..Default::default()
        }
    }

    fn acquire(&self, core: &mut CoreState, mut upgrade: UpgradeEntry) -> usize {
        upgrade.id = core.next_upgrade_id();
        core.upgrades.upgrades.push(upgrade);
        0
    }

    fn recovery(&self) -> UpgradeAcquireRecovery {
        UpgradeAcquireRecovery::None
    }

    fn tower_bonus(&self, _: &UpgradeEntry, _: &TowerState) -> i64 {
        0
    }

    fn tower_bonus_for_template(&self, _: &UpgradeEntry, _: &TowerTemplateState) -> i64 {
        0
    }

    fn current_and_max(&self, _: &CoreState) -> Option<(usize, usize)> {
        None
    }

    fn monster_death(
        &self,
        _: &mut UpgradeTriggerContext,
        _: &UpgradeEntry,
        _: &mut usize,
        _: &mut i64,
    ) {
    }

    fn gold_earned(&self, _: &mut UpgradeTriggerContext, _: &mut UpgradeEntry) -> bool {
        false
    }

    fn card_rerolled(&self, _: &mut UpgradeTriggerContext, _: &mut UpgradeEntry) -> bool {
        false
    }

    fn shop_purchase(&self, _: &mut UpgradeTriggerContext, _: &mut UpgradeEntry, _: bool) -> bool {
        false
    }

    fn tower_placed(
        &self,
        _: &mut UpgradeTriggerContext,
        _: &mut UpgradeEntry,
        _: u64,
        _: bool,
        _: &TowerTemplateState,
        _: &mut usize,
    ) -> bool {
        false
    }

    fn tower_removed(&self, _: &mut UpgradeTriggerContext, _: &mut UpgradeEntry, _: usize) {}

    fn stage_start(&self, _: &mut UpgradeTriggerContext, _: &mut UpgradeEntry, _: usize) -> bool {
        false
    }

    fn stage_end(
        &self,
        _: &mut UpgradeTriggerContext,
        _: &mut UpgradeEntry,
        _: bool,
        _: usize,
        _: usize,
    ) -> (bool, usize) {
        (false, 0)
    }
}

#[enum_dispatch(UpgradeBehavior)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UpgradeBehaviorImpl {
    Apple(apple::Behavior),
    Backpack(backpack::Behavior),
    Banana(banana::Behavior),
    BlackWhite(black_white::Behavior),
    BrokenPottery(broken_pottery::Behavior),
    Camera(camera::Behavior),
    Carrot(carrot::Behavior),
    Cat(cat::Behavior),
    Crock(crock::Behavior),
    CupNoodles(cup_noodles::Behavior),
    DemolitionHammer(demolition_hammer::Behavior),
    DiceBundle(dice_bundle::Behavior),
    EnergyDrink(energy_drink::Behavior),
    Fang(fang::Behavior),
    FourLeafClover(four_leaf_clover::Behavior),
    FrenchFries(french_fries::Behavior),
    GiftBox(gift_box::Behavior),
    Hamburger(hamburger::Behavior),
    IceCream(ice_cream::Behavior),
    MembershipCard(membership_card::Behavior),
    Metronome(metronome::Behavior),
    Mirror(mirror::Behavior),
    NameTag(name_tag::Behavior),
    Pea(pea::Behavior),
    PerfectPottery(perfect_pottery::Behavior),
    PiggyBank(piggy_bank::Behavior),
    Pizza(pizza::Behavior),
    Popcorn(popcorn::Behavior),
    Rabbit(rabbit::Behavior),
    Resolution(resolution::Behavior),
    ShoppingBag(shopping_bag::Behavior),
    SlotMachine(slot_machine::Behavior),
    Spanner(spanner::Behavior),
    Strawberry(strawberry::Behavior),
    Tape(tape::Behavior),
    Trophy(trophy::Behavior),
    Watermelon(watermelon::Behavior),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum UpgradeRuntimeState {
    Apple(apple::AppleUpgradeState),
    Banana(banana::BananaUpgradeState),
    Carrot(carrot::CarrotUpgradeState),
    Cat(cat::CatUpgradeState),
    Backpack(backpack::BackpackUpgradeState),
    DiceBundle(dice_bundle::DiceBundleUpgradeState),
    EnergyDrink(energy_drink::EnergyDrinkUpgradeState),
    Popcorn(popcorn::PopcornUpgradeState),
    FourLeafClover(four_leaf_clover::FourLeafCloverUpgradeState),
    Rabbit(rabbit::RabbitUpgradeState),
    BlackWhite(black_white::BlackWhiteUpgradeState),
    Trophy(trophy::TrophyUpgradeState),
    Crock(crock::CrockUpgradeState),
    CupNoodles(cup_noodles::CupNoodlesUpgradeState),
    FrenchFries(french_fries::FrenchFriesUpgradeState),
    Hamburger(hamburger::HamburgerUpgradeState),
    Pizza(pizza::PizzaUpgradeState),
    DemolitionHammer(demolition_hammer::DemolitionHammerUpgradeState),
    Metronome(metronome::MetronomeUpgradeState),
    Tape(tape::TapeUpgradeState),
    NameTag(name_tag::NameTagUpgradeState),
    ShoppingBag(shopping_bag::ShoppingBagUpgradeState),
    Resolution(resolution::ResolutionUpgradeState),
    Mirror(mirror::MirrorUpgradeState),
    IceCream(ice_cream::IceCreamUpgradeState),
    Spanner(spanner::SpannerUpgradeState),
    Pea(pea::PeaUpgradeState),
    SlotMachine(slot_machine::SlotMachineUpgradeState),
    PiggyBank(piggy_bank::PiggyBankUpgradeState),
    Camera(camera::CameraUpgradeState),
    GiftBox(gift_box::GiftBoxUpgradeState),
    Fang(fang::FangUpgradeState),
    PerfectPottery(perfect_pottery::PerfectPotteryUpgradeState),
    MembershipCard(membership_card::MembershipCardUpgradeState),
    BrokenPottery(broken_pottery::BrokenPotteryUpgradeState),
    Strawberry(strawberry::StrawberryUpgradeState),
    Watermelon(watermelon::WatermelonUpgradeState),
}

impl UpgradeBehaviorImpl {
    pub(crate) fn for_kind(kind: crate::UpgradeKind) -> Self {
        match kind {
            crate::UpgradeKind::Apple => Self::Apple(apple::Behavior),
            crate::UpgradeKind::Backpack => Self::Backpack(backpack::Behavior),
            crate::UpgradeKind::Banana => Self::Banana(banana::Behavior),
            crate::UpgradeKind::BlackWhite => Self::BlackWhite(black_white::Behavior),
            crate::UpgradeKind::BrokenPottery => Self::BrokenPottery(broken_pottery::Behavior),
            crate::UpgradeKind::Camera => Self::Camera(camera::Behavior),
            crate::UpgradeKind::Carrot => Self::Carrot(carrot::Behavior),
            crate::UpgradeKind::Cat => Self::Cat(cat::Behavior),
            crate::UpgradeKind::Crock => Self::Crock(crock::Behavior),
            crate::UpgradeKind::CupNoodles => Self::CupNoodles(cup_noodles::Behavior),
            crate::UpgradeKind::DemolitionHammer => {
                Self::DemolitionHammer(demolition_hammer::Behavior)
            }
            crate::UpgradeKind::DiceBundle => Self::DiceBundle(dice_bundle::Behavior),
            crate::UpgradeKind::EnergyDrink => Self::EnergyDrink(energy_drink::Behavior),
            crate::UpgradeKind::Fang => Self::Fang(fang::Behavior),
            crate::UpgradeKind::FourLeafClover => Self::FourLeafClover(four_leaf_clover::Behavior),
            crate::UpgradeKind::FrenchFries => Self::FrenchFries(french_fries::Behavior),
            crate::UpgradeKind::GiftBox => Self::GiftBox(gift_box::Behavior),
            crate::UpgradeKind::Hamburger => Self::Hamburger(hamburger::Behavior),
            crate::UpgradeKind::IceCream => Self::IceCream(ice_cream::Behavior),
            crate::UpgradeKind::MembershipCard => Self::MembershipCard(membership_card::Behavior),
            crate::UpgradeKind::Metronome => Self::Metronome(metronome::Behavior),
            crate::UpgradeKind::Mirror => Self::Mirror(mirror::Behavior),
            crate::UpgradeKind::NameTag => Self::NameTag(name_tag::Behavior),
            crate::UpgradeKind::Pea => Self::Pea(pea::Behavior),
            crate::UpgradeKind::PerfectPottery => Self::PerfectPottery(perfect_pottery::Behavior),
            crate::UpgradeKind::PiggyBank => Self::PiggyBank(piggy_bank::Behavior),
            crate::UpgradeKind::Pizza => Self::Pizza(pizza::Behavior),
            crate::UpgradeKind::Popcorn => Self::Popcorn(popcorn::Behavior),
            crate::UpgradeKind::Rabbit => Self::Rabbit(rabbit::Behavior),
            crate::UpgradeKind::Resolution => Self::Resolution(resolution::Behavior),
            crate::UpgradeKind::ShoppingBag => Self::ShoppingBag(shopping_bag::Behavior),
            crate::UpgradeKind::SlotMachine => Self::SlotMachine(slot_machine::Behavior),
            crate::UpgradeKind::Spanner => Self::Spanner(spanner::Behavior),
            crate::UpgradeKind::Strawberry => Self::Strawberry(strawberry::Behavior),
            crate::UpgradeKind::Tape => Self::Tape(tape::Behavior),
            crate::UpgradeKind::Trophy => Self::Trophy(trophy::Behavior),
            crate::UpgradeKind::Watermelon => Self::Watermelon(watermelon::Behavior),
        }
    }
}
