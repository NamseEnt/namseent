mod behaviors;
pub mod codec;
mod codec_impl;
pub(crate) mod payload;

use rand::seq::SliceRandom;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub(crate) use behaviors::UpgradeRuntimeState;
use behaviors::{UpgradeBehavior, UpgradeBehaviorImpl};
pub use codec_impl::UpgradeCodecError;
#[cfg(test)]
pub(crate) use codec_impl::UpgradeWireEntry as TestUpgradeWireEntry;
use codec_impl::UpgradeWireEntry;

pub const BASE_TREASURE_CAPACITY: usize = 5;

pub(crate) struct UpgradeTriggerContext<'a> {
    pub(crate) progress: &'a mut crate::CoreProgress,
    pub(crate) stage_modifiers: &'a mut crate::StageModifiersState,
    pub(crate) hand: &'a mut crate::HandState,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeCacheState {
    pub max_hp_plus_raw: i64,
    pub item_capacity_bonus: usize,
    pub treasure_capacity_bonus: usize,
    pub shop_slot_expand: usize,
    pub dice_chance_plus: usize,
    pub shop_item_price_minus: usize,
    pub shorten_straight_flush_to_4_cards: bool,
    pub skip_rank_for_straight: bool,
    pub treat_suits_as_same: bool,
    pub removed_number_rank_count: usize,
    pub clear_shield_on_stage_start: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct UpgradeEntryIdentityState {
    pub id: u64,
    pub kind: u8,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UpgradeEntry {
    pub(crate) id: u64,
    pub(crate) behavior: UpgradeBehaviorImpl,
    pub(crate) upgrade: UpgradeRuntimeState,
}

macro_rules! runtime_state_accessors {
    ($name:ident, $name_mut:ident, $variant:ident, $state:ty) => {
        #[allow(dead_code)]
        pub(crate) fn $name(&self) -> &$state {
            match &self.upgrade {
                UpgradeRuntimeState::$variant(state) => state,
                _ => panic!("upgrade runtime kind does not match behavior callback"),
            }
        }

        #[allow(dead_code)]
        pub(crate) fn $name_mut(&mut self) -> &mut $state {
            match &mut self.upgrade {
                UpgradeRuntimeState::$variant(state) => state,
                _ => panic!("upgrade runtime kind does not match behavior callback"),
            }
        }
    };
}

impl UpgradeEntry {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn with_id(mut self, id: u64) -> Self {
        self.id = id;
        self
    }

    pub fn scalar_value(&self, index: usize) -> Option<usize> {
        match self.upgrade {
            UpgradeRuntimeState::Backpack(state) => [state.shop_slot_expand].get(index).copied(),
            UpgradeRuntimeState::Cat(state) => [state.gold_per_kill].get(index).copied(),
            UpgradeRuntimeState::Crock(state) => [state.damage_steps].get(index).copied(),
            UpgradeRuntimeState::DiceBundle(state) => [state.dice_chance_plus].get(index).copied(),
            UpgradeRuntimeState::EnergyDrink(state) => [state.discount].get(index).copied(),
            UpgradeRuntimeState::Fang(state) => [state.heal_per_kill].get(index).copied(),
            UpgradeRuntimeState::GiftBox(state) => [state.gold_per_item].get(index).copied(),
            UpgradeRuntimeState::Metronome(state) => [state.acquired_stage].get(index).copied(),
            UpgradeRuntimeState::Tape(state) => [state.acquired_stage].get(index).copied(),
            UpgradeRuntimeState::SlotMachine(state) => [state.dice].get(index).copied(),
            UpgradeRuntimeState::Popcorn(state) => [state.duration_waves, state.waves_remaining]
                .get(index)
                .copied(),
            UpgradeRuntimeState::Resolution(state) => [state.saved_rerolls].get(index).copied(),
            UpgradeRuntimeState::IceCream(state) => [state.waves_remaining].get(index).copied(),
            UpgradeRuntimeState::Apple(_)
            | UpgradeRuntimeState::Banana(_)
            | UpgradeRuntimeState::Carrot(_)
            | UpgradeRuntimeState::BlackWhite(_)
            | UpgradeRuntimeState::BrokenPottery(_)
            | UpgradeRuntimeState::Camera(_)
            | UpgradeRuntimeState::CupNoodles(_)
            | UpgradeRuntimeState::DemolitionHammer(_)
            | UpgradeRuntimeState::FourLeafClover(_)
            | UpgradeRuntimeState::FrenchFries(_)
            | UpgradeRuntimeState::Hamburger(_)
            | UpgradeRuntimeState::Pea(_)
            | UpgradeRuntimeState::Pizza(_)
            | UpgradeRuntimeState::Rabbit(_)
            | UpgradeRuntimeState::ShoppingBag(_)
            | UpgradeRuntimeState::Spanner(_)
            | UpgradeRuntimeState::Strawberry(_)
            | UpgradeRuntimeState::Trophy(_)
            | UpgradeRuntimeState::Watermelon(_)
            | UpgradeRuntimeState::PiggyBank(_)
            | UpgradeRuntimeState::MembershipCard(_)
            | UpgradeRuntimeState::Mirror(_)
            | UpgradeRuntimeState::NameTag(_)
            | UpgradeRuntimeState::PerfectPottery(_) => None,
        }
    }

    pub fn ratio_value(&self, index: usize) -> Option<i64> {
        match self.upgrade {
            UpgradeRuntimeState::Popcorn(state) => {
                [state.max_multiplier_raw, state.active_multiplier_raw]
                    .get(index)
                    .copied()
            }
            UpgradeRuntimeState::NameTag(state) => [state.bonus_raw].get(index).copied(),
            UpgradeRuntimeState::Resolution(state) => [state.reroll_damage_raw].get(index).copied(),
            UpgradeRuntimeState::IceCream(state) => [state.damage_bonus_raw].get(index).copied(),
            UpgradeRuntimeState::PerfectPottery(state) => {
                [state.damage_bonus_raw].get(index).copied()
            }
            _ => None,
        }
    }

    pub fn bool_value(&self, index: usize) -> Option<bool> {
        match self.upgrade {
            UpgradeRuntimeState::MembershipCard(state) => [state.pending].get(index).copied(),
            UpgradeRuntimeState::Mirror(state) => [state.pending].get(index).copied(),
            _ => None,
        }
    }

    pub fn optional_id_value(&self, index: usize) -> Option<Option<u64>> {
        match self.upgrade {
            UpgradeRuntimeState::NameTag(state) => [state.tower_id].get(index).copied(),
            _ => None,
        }
    }

    pub fn set_scalar_value(&mut self, index: usize, value: usize) -> bool {
        match &mut self.upgrade {
            UpgradeRuntimeState::Backpack(state) if index == 0 => state.shop_slot_expand = value,
            UpgradeRuntimeState::Cat(state) if index == 0 => state.gold_per_kill = value,
            UpgradeRuntimeState::Crock(state) if index == 0 => state.damage_steps = value,
            UpgradeRuntimeState::DiceBundle(state) if index == 0 => state.dice_chance_plus = value,
            UpgradeRuntimeState::EnergyDrink(state) if index == 0 => state.discount = value,
            UpgradeRuntimeState::Fang(state) if index == 0 => state.heal_per_kill = value,
            UpgradeRuntimeState::GiftBox(state) if index == 0 => state.gold_per_item = value,
            UpgradeRuntimeState::Metronome(state) if index == 0 => state.acquired_stage = value,
            UpgradeRuntimeState::Tape(state) if index == 0 => state.acquired_stage = value,
            UpgradeRuntimeState::SlotMachine(state) if index == 0 => state.dice = value,
            UpgradeRuntimeState::Popcorn(state) => match index {
                0 => state.duration_waves = value,
                1 => state.waves_remaining = value,
                _ => return false,
            },
            UpgradeRuntimeState::Resolution(state) if index == 0 => state.saved_rerolls = value,
            UpgradeRuntimeState::IceCream(state) if index == 0 => state.waves_remaining = value,
            _ => return false,
        }
        true
    }

    pub fn set_ratio_value(&mut self, index: usize, value: i64) -> bool {
        match &mut self.upgrade {
            UpgradeRuntimeState::Popcorn(state) => match index {
                0 => state.max_multiplier_raw = value,
                1 => state.active_multiplier_raw = value,
                _ => return false,
            },
            UpgradeRuntimeState::NameTag(state) if index == 0 => state.bonus_raw = value,
            UpgradeRuntimeState::Resolution(state) if index == 0 => state.reroll_damage_raw = value,
            UpgradeRuntimeState::IceCream(state) if index == 0 => state.damage_bonus_raw = value,
            UpgradeRuntimeState::PerfectPottery(state) if index == 0 => {
                state.damage_bonus_raw = value
            }
            _ => return false,
        }
        true
    }

    pub fn set_bool_value(&mut self, index: usize, value: bool) -> bool {
        match &mut self.upgrade {
            UpgradeRuntimeState::MembershipCard(state) if index == 0 => state.pending = value,
            UpgradeRuntimeState::Mirror(state) if index == 0 => state.pending = value,
            _ => return false,
        }
        true
    }

    pub fn set_optional_id_value(&mut self, index: usize, value: Option<u64>) -> bool {
        match &mut self.upgrade {
            UpgradeRuntimeState::NameTag(state) if index == 0 => state.tower_id = value,
            _ => return false,
        }
        true
    }

    pub(crate) fn from_raw(entry: UpgradeWireEntry) -> Result<Self, codec_impl::UpgradeCodecError> {
        let upgrade = UpgradeRuntimeState::decode(&entry)?;
        let behavior = UpgradeBehaviorImpl::for_kind(upgrade.kind());
        Ok(Self {
            id: entry.id,
            behavior,
            upgrade,
        })
    }

    pub(crate) fn to_raw(&self) -> UpgradeWireEntry {
        self.upgrade.encode(self.id)
    }

    pub fn kind(&self) -> crate::UpgradeKind {
        self.behavior.kind()
    }

    #[allow(dead_code)]
    pub(crate) fn to_wire(&self) -> UpgradeWireEntry {
        self.to_raw()
    }

    #[allow(dead_code)]
    pub(crate) fn from_wire(entry: UpgradeWireEntry) -> Result<Self, UpgradeCodecError> {
        Self::from_raw(entry)
    }

    pub fn upgrade_kind(&self) -> Result<crate::UpgradeKind, crate::CommandError> {
        Ok(self.kind())
    }

    runtime_state_accessors!(apple, apple_mut, Apple, codec_impl::AppleUpgradeState);
    runtime_state_accessors!(banana, banana_mut, Banana, codec_impl::BananaUpgradeState);
    runtime_state_accessors!(carrot, carrot_mut, Carrot, codec_impl::CarrotUpgradeState);
    runtime_state_accessors!(cat, cat_mut, Cat, codec_impl::CatUpgradeState);
    runtime_state_accessors!(
        backpack,
        backpack_mut,
        Backpack,
        codec_impl::BackpackUpgradeState
    );
    runtime_state_accessors!(
        dice_bundle,
        dice_bundle_mut,
        DiceBundle,
        codec_impl::DiceBundleUpgradeState
    );
    runtime_state_accessors!(
        energy_drink,
        energy_drink_mut,
        EnergyDrink,
        codec_impl::EnergyDrinkUpgradeState
    );
    runtime_state_accessors!(
        popcorn,
        popcorn_mut,
        Popcorn,
        codec_impl::PopcornUpgradeState
    );
    runtime_state_accessors!(
        four_leaf_clover,
        four_leaf_clover_mut,
        FourLeafClover,
        codec_impl::FourLeafCloverUpgradeState
    );
    runtime_state_accessors!(rabbit, rabbit_mut, Rabbit, codec_impl::RabbitUpgradeState);
    runtime_state_accessors!(
        black_white,
        black_white_mut,
        BlackWhite,
        codec_impl::BlackWhiteUpgradeState
    );
    runtime_state_accessors!(trophy, trophy_mut, Trophy, codec_impl::TrophyUpgradeState);
    runtime_state_accessors!(crock, crock_mut, Crock, codec_impl::CrockUpgradeState);
    runtime_state_accessors!(
        cup_noodles,
        cup_noodles_mut,
        CupNoodles,
        codec_impl::CupNoodlesUpgradeState
    );
    runtime_state_accessors!(
        french_fries,
        french_fries_mut,
        FrenchFries,
        codec_impl::FrenchFriesUpgradeState
    );
    runtime_state_accessors!(
        hamburger,
        hamburger_mut,
        Hamburger,
        codec_impl::HamburgerUpgradeState
    );
    runtime_state_accessors!(pizza, pizza_mut, Pizza, codec_impl::PizzaUpgradeState);
    runtime_state_accessors!(
        demolition_hammer,
        demolition_hammer_mut,
        DemolitionHammer,
        codec_impl::DemolitionHammerUpgradeState
    );
    runtime_state_accessors!(
        metronome,
        metronome_mut,
        Metronome,
        codec_impl::MetronomeUpgradeState
    );
    runtime_state_accessors!(tape, tape_mut, Tape, codec_impl::TapeUpgradeState);
    runtime_state_accessors!(
        name_tag,
        name_tag_mut,
        NameTag,
        codec_impl::NameTagUpgradeState
    );
    runtime_state_accessors!(
        shopping_bag,
        shopping_bag_mut,
        ShoppingBag,
        codec_impl::ShoppingBagUpgradeState
    );
    runtime_state_accessors!(
        resolution,
        resolution_mut,
        Resolution,
        codec_impl::ResolutionUpgradeState
    );
    runtime_state_accessors!(mirror, mirror_mut, Mirror, codec_impl::MirrorUpgradeState);
    runtime_state_accessors!(
        ice_cream,
        ice_cream_mut,
        IceCream,
        codec_impl::IceCreamUpgradeState
    );
    runtime_state_accessors!(
        spanner,
        spanner_mut,
        Spanner,
        codec_impl::SpannerUpgradeState
    );
    runtime_state_accessors!(pea, pea_mut, Pea, codec_impl::PeaUpgradeState);
    runtime_state_accessors!(
        slot_machine,
        slot_machine_mut,
        SlotMachine,
        codec_impl::SlotMachineUpgradeState
    );
    runtime_state_accessors!(
        piggy_bank,
        piggy_bank_mut,
        PiggyBank,
        codec_impl::PiggyBankUpgradeState
    );
    runtime_state_accessors!(camera, camera_mut, Camera, codec_impl::CameraUpgradeState);
    runtime_state_accessors!(
        gift_box,
        gift_box_mut,
        GiftBox,
        codec_impl::GiftBoxUpgradeState
    );
    runtime_state_accessors!(fang, fang_mut, Fang, codec_impl::FangUpgradeState);
    runtime_state_accessors!(
        perfect_pottery,
        perfect_pottery_mut,
        PerfectPottery,
        codec_impl::PerfectPotteryUpgradeState
    );
    runtime_state_accessors!(
        membership_card,
        membership_card_mut,
        MembershipCard,
        codec_impl::MembershipCardUpgradeState
    );
    runtime_state_accessors!(
        broken_pottery,
        broken_pottery_mut,
        BrokenPottery,
        codec_impl::BrokenPotteryUpgradeState
    );
    runtime_state_accessors!(
        strawberry,
        strawberry_mut,
        Strawberry,
        codec_impl::StrawberryUpgradeState
    );
    runtime_state_accessors!(
        watermelon,
        watermelon_mut,
        Watermelon,
        codec_impl::WatermelonUpgradeState
    );
}

impl Serialize for UpgradeEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        codec::encode_upgrade_entry(self, serializer)
    }
}

impl<'de> Deserialize<'de> for UpgradeEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        codec::decode_upgrade_entry(deserializer)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct UpgradeCollection {
    pub upgrades: Vec<UpgradeEntry>,
    pub(crate) revision: usize,
}

impl UpgradeCollection {
    pub fn from_entries(entries: Vec<UpgradeEntry>, revision: usize) -> Self {
        Self {
            upgrades: entries,
            revision,
        }
    }

    pub fn entries(&self) -> &[UpgradeEntry] {
        &self.upgrades
    }

    pub fn len(&self) -> usize {
        self.upgrades.len()
    }

    pub fn is_empty(&self) -> bool {
        self.upgrades.is_empty()
    }

    pub fn entries_mut(&mut self) -> &mut Vec<UpgradeEntry> {
        &mut self.upgrades
    }

    pub fn revision(&self) -> usize {
        self.revision
    }

    pub fn remove_by_id(&mut self, upgrade_id: u64) -> Option<UpgradeEntry> {
        let index = self
            .upgrades
            .iter()
            .position(|upgrade| upgrade.id == upgrade_id)?;
        let removed = self.upgrades.remove(index);
        self.revision = self.revision.wrapping_add(1);
        Some(removed)
    }

    pub(crate) fn shorten_straight_flush_to_4_cards(&self) -> bool {
        self.cache_state().shorten_straight_flush_to_4_cards
    }

    pub(crate) fn skip_rank_for_straight(&self) -> bool {
        self.cache_state().skip_rank_for_straight
    }

    pub(crate) fn treat_suits_as_same(&self) -> bool {
        self.cache_state().treat_suits_as_same
    }

    pub(crate) fn next_id(&self) -> u64 {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1)
    }

    pub fn cache_state(&self) -> UpgradeCacheState {
        let contributions = self
            .upgrades
            .iter()
            .map(|upgrade| upgrade.behavior.cache(upgrade))
            .fold(
                UpgradeCacheContribution {
                    clear_shield_on_stage_start: true,
                    ..UpgradeCacheContribution::default()
                },
                |mut total, value| {
                    total.max_hp_plus_raw =
                        total.max_hp_plus_raw.saturating_add(value.max_hp_plus_raw);
                    total.item_capacity_bonus = total
                        .item_capacity_bonus
                        .saturating_add(value.item_capacity_bonus);
                    total.treasure_capacity_bonus = total
                        .treasure_capacity_bonus
                        .saturating_add(value.treasure_capacity_bonus);
                    total.shop_slot_expand = total
                        .shop_slot_expand
                        .saturating_add(value.shop_slot_expand);
                    total.dice_chance_plus = total
                        .dice_chance_plus
                        .saturating_add(value.dice_chance_plus);
                    total.shop_item_price_minus = total
                        .shop_item_price_minus
                        .saturating_add(value.shop_item_price_minus);
                    total.shorten_straight_flush_to_4_cards |=
                        value.shorten_straight_flush_to_4_cards;
                    total.skip_rank_for_straight |= value.skip_rank_for_straight;
                    total.treat_suits_as_same |= value.treat_suits_as_same;
                    total.clear_shield_on_stage_start &= value.clear_shield_on_stage_start;
                    total
                },
            );
        UpgradeCacheState {
            max_hp_plus_raw: contributions.max_hp_plus_raw,
            item_capacity_bonus: contributions.item_capacity_bonus,
            treasure_capacity_bonus: contributions.treasure_capacity_bonus,
            shop_slot_expand: contributions.shop_slot_expand,
            dice_chance_plus: contributions.dice_chance_plus,
            shop_item_price_minus: contributions.shop_item_price_minus,
            shorten_straight_flush_to_4_cards: contributions.shorten_straight_flush_to_4_cards,
            skip_rank_for_straight: contributions.skip_rank_for_straight,
            treat_suits_as_same: contributions.treat_suits_as_same,
            removed_number_rank_count: 0,
            clear_shield_on_stage_start: contributions.clear_shield_on_stage_start,
        }
    }

    pub fn tower_damage_bonus_raw(&self, tower: &crate::TowerState) -> i64 {
        let upgrade_bonus = self
            .upgrades
            .iter()
            .map(|upgrade| upgrade.behavior.tower_bonus(upgrade, tower))
            .fold(0_i64, i64::saturating_add);
        let polish_bonus = tower
            .template
            .used_cards
            .iter()
            .map(|card| card.polish_pct_raw)
            .fold(0_i64, i64::saturating_add);
        upgrade_bonus.saturating_add(polish_bonus)
    }

    pub fn tower_damage_bonus_raw_for_template(&self, template: &crate::TowerTemplateState) -> i64 {
        let upgrade_bonus = self
            .upgrades
            .iter()
            .map(|upgrade| upgrade.behavior.tower_bonus_for_template(upgrade, template))
            .fold(0_i64, i64::saturating_add);
        let polish_bonus = template
            .used_cards
            .iter()
            .map(|card| card.polish_pct_raw)
            .fold(0_i64, i64::saturating_add);
        upgrade_bonus.saturating_add(polish_bonus)
    }

    pub fn tower_upgrade_bonus_raw(&self, tower: &crate::TowerState) -> i64 {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.behavior.tower_bonus(upgrade, tower))
            .fold(0_i64, i64::saturating_add)
    }

    pub fn tower_upgrade_bonus_raw_for_template(
        &self,
        template: &crate::TowerTemplateState,
    ) -> i64 {
        self.upgrades
            .iter()
            .map(|upgrade| upgrade.behavior.tower_bonus_for_template(upgrade, template))
            .fold(0_i64, i64::saturating_add)
    }
}

impl Serialize for UpgradeCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        codec::encode_upgrade_collection(self, serializer)
    }
}

impl<'de> Deserialize<'de> for UpgradeCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        codec::decode_upgrade_collection(deserializer)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct UpgradeCacheContribution {
    pub(crate) max_hp_plus_raw: i64,
    pub(crate) item_capacity_bonus: usize,
    pub(crate) treasure_capacity_bonus: usize,
    pub(crate) shop_slot_expand: usize,
    pub(crate) dice_chance_plus: usize,
    pub(crate) shop_item_price_minus: usize,
    pub(crate) shorten_straight_flush_to_4_cards: bool,
    pub(crate) skip_rank_for_straight: bool,
    pub(crate) treat_suits_as_same: bool,
    pub(crate) clear_shield_on_stage_start: bool,
}

pub fn generated_upgrade(kind: crate::UpgradeKind) -> crate::UpgradeEntry {
    let behavior = UpgradeBehaviorImpl::for_kind(kind);
    UpgradeEntry {
        id: 0,
        behavior,
        upgrade: behavior.generate(),
    }
}

pub fn generated_upgrade_raw(raw: u8) -> Result<crate::UpgradeEntry, crate::CommandError> {
    let kind =
        crate::UpgradeKind::from_raw(raw).ok_or(crate::CommandError::InvalidUpgradeKind { raw })?;
    Ok(generated_upgrade(kind))
}

/// Returns the definition rarity for a validated upgrade kind.
pub fn upgrade_rarity(kind: crate::UpgradeKind) -> crate::Rarity {
    UpgradeBehaviorImpl::for_kind(kind).rarity()
}

/// Returns the definition rarity after decoding a persisted or wire raw kind.
pub fn upgrade_rarity_raw(raw: u8) -> Result<crate::Rarity, crate::CommandError> {
    let kind =
        crate::UpgradeKind::from_raw(raw).ok_or(crate::CommandError::InvalidUpgradeKind { raw })?;
    Ok(upgrade_rarity(kind))
}

fn current_and_max(core: &crate::CoreState, kind: crate::UpgradeKind) -> Option<(usize, usize)> {
    UpgradeBehaviorImpl::for_kind(kind).current_and_max(core)
}

pub fn generate_boss_reward_option(core: &mut crate::CoreState) -> crate::UpgradeEntry {
    let mut rng = core.rng.next_rng(
        crate::deterministic_rng::domain::REWARD_UPGRADE,
        &[core.progress.stage as u64],
    );
    let kinds = crate::UpgradeKind::ALL.to_vec();
    let kind = kinds
        .choose_weighted(&mut rng, |kind| {
            if current_and_max(core, *kind).is_some_and(|(current, max)| current >= max) {
                return 0.0_f32;
            }
            match upgrade_rarity(*kind) {
                crate::Rarity::Common => 5.0_f32,
                crate::Rarity::Rare => 10.0_f32,
                crate::Rarity::Epic => 25.0_f32,
                crate::Rarity::Legendary => 50.0_f32,
            }
        })
        .expect("at least one upgrade reward must be eligible");
    generated_upgrade(*kind)
}

pub(crate) fn generate_boss_reward_options(
    core: &mut crate::CoreState,
) -> Vec<crate::UpgradeEntry> {
    (0..3).map(|_| generate_boss_reward_option(core)).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeAcquireRecovery {
    None,
    Amount(i64),
    ToFull,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpgradeAcquireOutput {
    pub recovery: UpgradeAcquireRecovery,
    pub additional_shop_slots: usize,
}

pub trait UpgradeAcquireInput {
    fn into_runtime(self) -> Result<UpgradeEntry, crate::CommandError>;
}

impl UpgradeAcquireInput for UpgradeEntry {
    fn into_runtime(self) -> Result<UpgradeEntry, crate::CommandError> {
        Ok(self)
    }
}

impl crate::CoreState {
    pub fn acquire_upgrade(
        &mut self,
        upgrade: impl UpgradeAcquireInput,
    ) -> Result<UpgradeAcquireOutput, crate::CommandError> {
        if self.upgrades.len() >= self.treasure_capacity() {
            return Err(crate::CommandError::TreasureCapacityReached);
        }
        let upgrade = upgrade.into_runtime()?;
        let kind = upgrade.kind();
        let recovery = upgrade_acquire_recovery(kind);
        let behavior = upgrade.behavior;
        let additional_shop_slots = behavior.acquire(self, upgrade);

        self.refresh_upgrade_damage_multipliers();
        self.bump_upgrade_revision();
        Ok(UpgradeAcquireOutput {
            recovery,
            additional_shop_slots,
        })
    }

    pub fn apply_upgrade_recovery(&mut self, recovery: UpgradeAcquireRecovery) {
        match recovery {
            UpgradeAcquireRecovery::None => {}
            UpgradeAcquireRecovery::Amount(amount) => {
                self.hp_raw = self.hp_raw.saturating_add(amount).min(self.max_hp_raw());
            }
            UpgradeAcquireRecovery::ToFull => {
                self.hp_raw = self.max_hp_raw();
            }
        }
    }

    pub fn trigger_monster_death_upgrades(&mut self) {
        let mut gold = 0_usize;
        let mut healing = 0_i64;
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for entry in &mut self.upgrades.upgrades {
            let behavior = entry.behavior;
            behavior.monster_death(&mut context, entry, &mut gold, &mut healing);
        }
        if gold > 0 {
            self.earn_gold(gold);
        }
        if healing > 0 {
            self.hp_raw = self.hp_raw.saturating_add(healing).min(self.max_hp_raw());
        }
    }

    pub fn trigger_gold_earned_upgrades(&mut self) {
        let mut changed = false;
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for entry in &mut self.upgrades.upgrades {
            let behavior = entry.behavior;
            changed |= behavior.gold_earned(&mut context, entry);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_gold_spent_upgrades(&mut self) {
        self.trigger_gold_earned_upgrades();
    }

    pub fn trigger_card_reroll_upgrades(&mut self) {
        let mut changed = false;
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for entry in &mut self.upgrades.upgrades {
            let behavior = entry.behavior;
            changed |= behavior.card_rerolled(&mut context, entry);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_shop_purchase_upgrades(&mut self, item_purchase: bool) {
        let mut changed = false;
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for entry in &mut self.upgrades.upgrades {
            let behavior = entry.behavior;
            changed |= behavior.shop_purchase(&mut context, entry, item_purchase);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_tower_placed_upgrades(
        &mut self,
        tower_id: u64,
        is_face: bool,
        tower_template: &crate::TowerTemplateState,
    ) {
        let mut changed = false;
        let mut camera_reward: usize = 0;
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for entry in &mut self.upgrades.upgrades {
            let behavior = entry.behavior;
            changed |= behavior.tower_placed(
                &mut context,
                entry,
                tower_id,
                is_face,
                tower_template,
                &mut camera_reward,
            );
        }
        if camera_reward > 0 {
            self.progress.gold = self.progress.gold.saturating_add(camera_reward);
            self.metrics.total_gold_earned =
                self.metrics.total_gold_earned.saturating_add(camera_reward);
            self.trigger_gold_earned_upgrades();
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_tower_removed_upgrades(&mut self, rerolled_count: usize) {
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for entry in &mut self.upgrades.upgrades {
            let behavior = entry.behavior;
            behavior.tower_removed(&mut context, entry, rerolled_count);
        }
    }

    pub fn trigger_stage_start_upgrades(&mut self, stage: usize) {
        let mut changed = false;
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for entry in &mut self.upgrades.upgrades {
            let behavior = entry.behavior;
            changed |= behavior.stage_start(&mut context, entry, stage);
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    pub fn trigger_stage_end_upgrades(
        &mut self,
        perfect_clear: bool,
        gold: usize,
        item_count: usize,
    ) {
        let mut changed = false;
        let mut gold_reward: usize = 0;
        let mut context = UpgradeTriggerContext {
            progress: &mut self.progress,
            stage_modifiers: &mut self.stage_modifiers,
            hand: &mut self.hand,
        };
        for upgrade in &mut self.upgrades.upgrades {
            let behavior = upgrade.behavior;
            let (upgrade_changed, reward) =
                behavior.stage_end(&mut context, upgrade, perfect_clear, gold, item_count);
            changed |= upgrade_changed;
            gold_reward = gold_reward.saturating_add(reward);
        }
        if gold_reward > 0 {
            self.progress.gold = self.progress.gold.saturating_add(gold_reward);
            self.metrics.total_gold_earned =
                self.metrics.total_gold_earned.saturating_add(gold_reward);
            self.trigger_gold_earned_upgrades();
        }
        if changed {
            self.refresh_upgrade_damage_multipliers();
            self.bump_upgrade_revision();
        }
    }

    fn bump_upgrade_revision(&mut self) {
        self.upgrades.revision = self.upgrades.revision.wrapping_add(1);
    }

    pub(crate) fn refresh_upgrade_damage_multipliers(&mut self) {
        for tower in &mut self.towers {
            let bonus_raw = self.upgrades.tower_damage_bonus_raw(tower);
            tower.damage_multiplier_raw = crate::RATIO_SCALE.saturating_add(bonus_raw).max(0);
        }
    }

    fn next_upgrade_id(&self) -> u64 {
        self.upgrades.next_id()
    }
}

fn upgrade_acquire_recovery(kind: crate::UpgradeKind) -> UpgradeAcquireRecovery {
    UpgradeBehaviorImpl::for_kind(kind).recovery()
}

#[cfg(test)]
mod tests {
    use super::codec_impl::UpgradeWireEntry;
    use super::*;

    fn typed_collection(entries: Vec<UpgradeWireEntry>) -> UpgradeCollection {
        UpgradeCollection::from_entries(
            entries
                .into_iter()
                .map(UpgradeEntry::from_raw)
                .collect::<Result<Vec<_>, _>>()
                .expect("test upgrades must decode"),
            0,
        )
    }

    fn tower(id: u64, rerolled_count: usize, polish_pct_raw: i64) -> crate::TowerState {
        crate::TowerState {
            id: Some(id),
            left_top: [0, 0],
            cooldown: 0,
            template: crate::TowerTemplateState {
                kind: 1,
                rerolled_count,
                shoot_interval: 60,
                default_attack_range_radius_raw: 1,
                default_damage_raw: 1,
                suit: Some(0),
                rank: Some(11),
                skill_templates: Vec::new(),
                default_status_effects: Vec::new(),
                used_cards: vec![crate::CardState {
                    id: 1,
                    suit: 0,
                    rank: 11,
                    polish_pct_raw,
                    engraving: None,
                }],
            },
            status_effects: Vec::new(),
            skills: Vec::new(),
            damage_multiplier_raw: crate::RATIO_SCALE,
            attack_range_radius_raw: 1,
            effective_shoot_interval: 60,
            on_hit_splashes: Vec::new(),
            on_attack_splashes: Vec::new(),
        }
    }

    #[test]
    fn tower_damage_bonus_sums_upgrade_states_and_card_polish_once() {
        let upgrades = typed_collection(vec![
            UpgradeWireEntry {
                id: 1,
                kind: 7,
                scalar_values: Vec::new(),
                ratio_values_raw: vec![125_000],
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            },
            UpgradeWireEntry {
                id: 2,
                kind: 20,
                scalar_values: Vec::new(),
                ratio_values_raw: vec![250_000],
                bool_values: Vec::new(),
                optional_ids: vec![Some(9)],
            },
            UpgradeWireEntry {
                id: 3,
                kind: 22,
                scalar_values: vec![2],
                ratio_values_raw: vec![50_000],
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            },
        ]);
        let tower = tower(9, 0, 75_000);

        assert_eq!(
            upgrades.tower_damage_bonus_raw(&tower),
            125_000 + 250_000 + 100_000 + 75_000
        );
    }

    #[test]
    fn tower_damage_bonus_excludes_no_reroll_bonus_after_reroll() {
        let upgrades = typed_collection(vec![UpgradeWireEntry {
            id: 1,
            kind: 7,
            scalar_values: Vec::new(),
            ratio_values_raw: vec![125_000],
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        }]);

        assert_eq!(upgrades.tower_damage_bonus_raw(&tower(1, 1, 0)), 0);
    }

    #[test]
    fn template_damage_observation_matches_placed_tower_for_previewable_upgrades() {
        let upgrades = typed_collection(vec![
            UpgradeWireEntry {
                id: 1,
                kind: 7,
                scalar_values: Vec::new(),
                ratio_values_raw: vec![125_000],
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            },
            UpgradeWireEntry {
                id: 2,
                kind: 22,
                scalar_values: vec![2],
                ratio_values_raw: vec![50_000],
                bool_values: Vec::new(),
                optional_ids: Vec::new(),
            },
        ]);
        let placed = tower(9, 0, 75_000);

        assert_eq!(
            upgrades.tower_damage_bonus_raw(&placed),
            upgrades.tower_damage_bonus_raw_for_template(&placed.template)
        );
    }

    #[test]
    fn cache_observation_matches_legacy_upgrade_effect_values() {
        let upgrades = typed_collection(vec![
            generated_upgrade(crate::UpgradeKind::Apple).to_wire(),
            generated_upgrade(crate::UpgradeKind::Backpack).to_wire(),
            generated_upgrade(crate::UpgradeKind::DiceBundle).to_wire(),
            generated_upgrade(crate::UpgradeKind::EnergyDrink).to_wire(),
            generated_upgrade(crate::UpgradeKind::FourLeafClover).to_wire(),
            generated_upgrade(crate::UpgradeKind::Rabbit).to_wire(),
            generated_upgrade(crate::UpgradeKind::BlackWhite).to_wire(),
            generated_upgrade(crate::UpgradeKind::Spanner).to_wire(),
        ]);
        let cache = upgrades.cache_state();

        assert_eq!(cache.max_hp_plus_raw, 4_000);
        assert_eq!(cache.shop_slot_expand, 1);
        assert_eq!(cache.dice_chance_plus, 1);
        assert_eq!(cache.shop_item_price_minus, 5);
        assert!(cache.shorten_straight_flush_to_4_cards);
        assert!(cache.skip_rank_for_straight);
        assert!(cache.treat_suits_as_same);
        assert!(!cache.clear_shield_on_stage_start);
    }

    fn test_core() -> crate::CoreState {
        let config = crate::GameConfigState {
            player: crate::PlayerConfigState {
                max_hp_raw: 60_000,
                starting_gold: 100,
                starting_hp_raw: 60_000,
                base_dice_chance: 3,
                max_stages: 5,
                base_hand_slots: 5,
            },
            towers: crate::TowerConfigState {
                entries: (0..=10)
                    .map(|kind| crate::TowerConfigEntryState {
                        kind,
                        damage_raw: 1_000,
                        range_raw: 1_000_000,
                        cooldown_ms: 1_000,
                    })
                    .collect(),
            },
            monsters: crate::MonsterConfigState {
                stats: vec![crate::MonsterConfigEntryState {
                    kind: 0,
                    base_hp_raw: 1_000,
                    velocity_mul_raw: crate::RATIO_SCALE,
                    damage_raw: 100,
                    reward: 1,
                }],
                stage_waves: vec![crate::StageWaveState {
                    stage: 1,
                    entries: vec![crate::StageWaveEntryState { kind: 0, count: 1 }],
                }],
            },
        };
        let mut state = crate::CoreState::new_initial(config, 7);
        state.start_stage(1);
        state
    }

    fn with_upgrades(core: &mut crate::CoreState, upgrades: Vec<UpgradeWireEntry>) {
        core.edit_snapshot(|parts| {
            parts.upgrades = UpgradeCollection::from_entries(
                upgrades
                    .into_iter()
                    .map(UpgradeEntry::from_raw)
                    .collect::<Result<Vec<_>, _>>()
                    .expect("test upgrades must decode"),
                0,
            )
        })
        .expect("test upgrade state must be valid");
    }

    #[test]
    fn item_and_treasure_grants_respect_the_default_capacity() {
        let mut core = test_core();
        core.drain_events().for_each(|_| {});

        for _ in 0..2 {
            core.grant_inventory_item(
                crate::generated_item(crate::ItemKind::Bread).expect("bread"),
            )
            .expect("two additional items fit");
        }
        assert_eq!(core.item_capacity(), 5);
        assert_eq!(
            core.grant_inventory_item(
                crate::generated_item(crate::ItemKind::Bread).expect("bread")
            ),
            Err(crate::CommandError::ItemCapacityReached)
        );

        for _ in 0..5 {
            core.acquire_upgrade(crate::generated_upgrade(crate::UpgradeKind::Apple))
                .expect("five treasures fit");
        }
        assert_eq!(core.treasure_capacity(), 5);
        assert_eq!(
            core.acquire_upgrade(crate::generated_upgrade(crate::UpgradeKind::Apple)),
            Err(crate::CommandError::TreasureCapacityReached)
        );
    }

    #[test]
    fn discarding_treasure_rebuilds_ownership_effects_without_reversing_recovery() {
        let mut core = test_core();
        core.drain_events().for_each(|_| {});
        core.edit_snapshot(|parts| parts.hp_raw = 60_000)
            .expect("test HP edit must preserve a valid snapshot");

        let recovery = core
            .acquire_upgrade(crate::generated_upgrade(crate::UpgradeKind::Pea))
            .expect("pea acquisition")
            .recovery;
        core.apply_upgrade_recovery(recovery);
        let upgrade_id = core.upgrades().entries()[0].id();
        assert!(core.max_hp_raw() > 60_000);
        assert_eq!(core.hp_raw(), core.max_hp_raw());

        core.discard_treasure(upgrade_id)
            .expect("pea should be discardable");

        assert_eq!(core.max_hp_raw(), 60_000);
        assert_eq!(core.hp_raw(), 60_000);
        assert!(
            core.drain_events()
                .any(|event| { matches!(event, crate::CoreEvent::TreasureDiscarded { .. }) })
        );
    }

    #[test]
    fn acquisition_applies_kind_state_and_recovery() {
        let mut core = test_core();
        core.edit_snapshot(|parts| parts.hp_raw = 10_000)
            .expect("test health must be valid");

        let recovery = core
            .acquire_upgrade(generated_upgrade(crate::UpgradeKind::Apple))
            .expect("apple kind is valid")
            .recovery;
        assert_eq!(recovery, UpgradeAcquireRecovery::Amount(6_000));
        core.apply_upgrade_recovery(recovery);
        assert_eq!(core.hp_raw(), 16_000);

        let output = core
            .acquire_upgrade(generated_upgrade(crate::UpgradeKind::Metronome))
            .expect("metronome kind is valid");
        assert_eq!(output.recovery, UpgradeAcquireRecovery::None);
        assert_eq!(
            core.upgrades()
                .upgrades
                .last()
                .expect("metronome must be acquired")
                .to_wire()
                .scalar_values,
            vec![core.progress().stage as u64]
        );
    }

    #[test]
    fn monster_death_trigger_updates_gold_metrics_and_health() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeWireEntry {
                    id: 1,
                    kind: 3,
                    scalar_values: vec![7],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 2,
                    kind: 31,
                    scalar_values: vec![2],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        core.edit_snapshot(|parts| {
            parts.progress.gold = 10;
            parts.hp_raw = 10_000;
        })
        .expect("test trigger state must be valid");

        core.trigger_monster_death_upgrades();

        assert_eq!(core.progress().gold, 17);
        assert_eq!(core.metrics().total_gold_earned, 7);
        assert_eq!(core.hp_raw(), 12_000);
    }

    #[test]
    fn gold_trigger_refreshes_crock_state() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![crate::UpgradeWireEntry {
                id: 1,
                kind: 12,
                scalar_values: vec![0],
                ratio_values_raw: vec![],
                bool_values: vec![],
                optional_ids: vec![],
            }],
        );
        core.edit_snapshot(|parts| parts.progress.gold = 250)
            .expect("test gold state must be valid");

        core.trigger_gold_earned_upgrades();

        assert_eq!(
            core.upgrades().entries()[0].to_wire().scalar_values,
            vec![2]
        );
    }

    #[test]
    fn card_reroll_trigger_updates_resolution_and_broken_pottery() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeWireEntry {
                    id: 1,
                    kind: 22,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![250_000],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 2,
                    kind: 34,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        core.edit_snapshot(|parts| {
            parts.progress.left_dice = 3;
            parts.progress.rerolled_count = 4;
        })
        .expect("test reroll state must be valid");

        core.trigger_card_reroll_upgrades();

        assert_eq!(
            core.upgrades().entries()[0].to_wire().scalar_values,
            vec![3]
        );
        assert_eq!(core.progress().left_dice, 4);
    }

    #[test]
    fn shop_purchase_trigger_awards_item_reroll() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![crate::UpgradeWireEntry {
                id: 1,
                kind: 21,
                scalar_values: vec![],
                ratio_values_raw: vec![],
                bool_values: vec![],
                optional_ids: vec![],
            }],
        );
        core.edit_snapshot(|parts| parts.progress.left_dice = 2)
            .expect("test purchase state must be valid");

        core.trigger_shop_purchase_upgrades(true);

        assert_eq!(core.progress().left_dice, 3);
    }

    #[test]
    fn tower_placement_and_removal_triggers_apply_kind_rules() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeWireEntry {
                    id: 1,
                    kind: 20,
                    scalar_values: vec![],
                    ratio_values_raw: vec![2_000_000],
                    bool_values: vec![],
                    optional_ids: vec![None],
                },
                crate::UpgradeWireEntry {
                    id: 2,
                    kind: 23,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![true],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 3,
                    kind: 29,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 4,
                    kind: 17,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        let template = crate::TowerTemplateState {
            kind: 0,
            rerolled_count: 0,
            shoot_interval: 60,
            default_attack_range_radius_raw: 1,
            default_damage_raw: 1,
            suit: Some(0),
            rank: Some(12),
            skill_templates: vec![],
            default_status_effects: vec![],
            used_cards: vec![],
        };

        core.trigger_tower_placed_upgrades(9, true, &template);
        assert_eq!(
            core.upgrades().entries()[0].to_wire().optional_ids,
            vec![Some(9)]
        );
        assert!(!core.upgrades().entries()[1].to_wire().bool_values[0]);
        assert_eq!(core.progress().gold, 150);
        assert_eq!(core.hand().slots.len(), 6);

        core.edit_snapshot(|parts| parts.progress.left_dice = 0)
            .expect("test removal state must be valid");
        core.trigger_tower_removed_upgrades(3);
        assert_eq!(core.progress().left_dice, 3);
    }

    #[test]
    fn stage_triggers_apply_start_and_end_states() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeWireEntry {
                    id: 1,
                    kind: 18,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 2,
                    kind: 19,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 3,
                    kind: 11,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 4,
                    kind: 30,
                    scalar_values: vec![5],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
            ],
        );
        core.edit_snapshot(|parts| parts.progress.left_dice = 0)
            .expect("test stage state must be valid");

        core.trigger_stage_start_upgrades(1);
        assert_eq!(core.progress().left_dice, 2);
        core.trigger_stage_start_upgrades(3);
        assert_eq!(
            core.stage_modifiers().enemy_speed_multipliers_raw,
            vec![750_000]
        );

        core.trigger_stage_end_upgrades(true, 0, 2);
        assert_eq!(core.stage_modifiers().free_card_services, 1);
        assert_eq!(core.progress().gold, 110);
    }

    #[test]
    fn generated_upgrade_states_cover_every_catalog_kind() {
        let mut seen_rarities = [false; 4];
        assert_eq!(crate::UpgradeKind::ALL.len(), crate::UpgradeKind::COUNT);
        for &kind in crate::UpgradeKind::ALL {
            let behavior = UpgradeBehaviorImpl::for_kind(kind);
            assert_eq!(behavior.kind(), kind);
            let upgrade = generated_upgrade(kind);
            assert_eq!(upgrade.kind().raw(), kind.raw());
            assert!(upgrade_rarity(kind).index() < crate::Rarity::ALL.len());
            seen_rarities[upgrade_rarity(kind).index()] = true;
        }
        assert_eq!(seen_rarities, [true, true, true, true]);
    }

    #[test]
    fn raw_upgrade_boundaries_round_trip_and_reject_unknown_kinds() {
        for &kind in crate::UpgradeKind::ALL {
            let generated = generated_upgrade_raw(kind.raw()).expect("catalog kind is valid");
            assert_eq!(generated.kind().raw(), kind.raw());
            assert_eq!(
                generated.upgrade_kind().expect("catalog kind is valid"),
                kind
            );
            assert_eq!(
                generated_upgrade_raw(kind.raw()).expect("catalog kind is valid"),
                generated
            );
            assert_eq!(
                upgrade_rarity_raw(kind.raw()).expect("catalog kind is valid"),
                upgrade_rarity(kind)
            );
        }
        assert_eq!(
            generated_upgrade_raw(u8::MAX),
            Err(crate::CommandError::InvalidUpgradeKind { raw: u8::MAX })
        );
        assert_eq!(
            upgrade_rarity_raw(u8::MAX),
            Err(crate::CommandError::InvalidUpgradeKind { raw: u8::MAX })
        );
        let invalid = crate::UpgradeWireEntry {
            id: 0,
            kind: u8::MAX,
            scalar_values: Vec::new(),
            ratio_values_raw: Vec::new(),
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        };
        assert!(UpgradeEntry::from_raw(invalid).is_err());
    }

    #[test]
    fn generated_upgrade_serde_preserves_every_canonical_raw_kind() {
        for &kind in crate::UpgradeKind::ALL {
            let generated = generated_upgrade(kind);
            let encoded = serde_json::to_string(&generated).expect("upgrade serializes");
            let decoded: UpgradeWireEntry =
                serde_json::from_str(&encoded).expect("upgrade deserializes");
            assert_eq!(decoded, generated.to_wire());
            assert_eq!(decoded.upgrade_kind(), Ok(kind));
        }
    }

    #[test]
    fn acquisition_and_trigger_families_are_deterministic_across_core_states() {
        let mut first = test_core();
        let mut second = test_core();
        for &kind in crate::UpgradeKind::ALL {
            let first_output = first.acquire_upgrade(generated_upgrade(kind));
            let second_output = second.acquire_upgrade(generated_upgrade(kind));
            assert_eq!(first_output, second_output, "acquisition kind {kind:?}");
        }

        first
            .edit_snapshot(|parts| {
                parts.progress.gold = 250;
                parts.progress.left_dice = 4;
                parts.progress.rerolled_count = 4;
            })
            .expect("first trigger fixture");
        second
            .edit_snapshot(|parts| {
                parts.progress.gold = 250;
                parts.progress.left_dice = 4;
                parts.progress.rerolled_count = 4;
            })
            .expect("second trigger fixture");

        for state in [&mut first, &mut second] {
            state.trigger_monster_death_upgrades();
            state.trigger_gold_earned_upgrades();
            state.trigger_gold_spent_upgrades();
            state.trigger_card_reroll_upgrades();
            state.trigger_shop_purchase_upgrades(true);
            state.trigger_stage_start_upgrades(2);
            state.trigger_stage_end_upgrades(true, 1, 1);
        }

        assert_eq!(first, second);
        assert_eq!(
            crate::authoritative_hash(&first),
            crate::authoritative_hash(&second)
        );
    }

    #[test]
    fn typed_trigger_dispatch_preserves_entry_order_and_identity() {
        let mut core = test_core();
        with_upgrades(
            &mut core,
            vec![
                crate::UpgradeWireEntry {
                    id: 41,
                    kind: 12,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 42,
                    kind: 22,
                    scalar_values: vec![0],
                    ratio_values_raw: vec![250_000],
                    bool_values: vec![],
                    optional_ids: vec![],
                },
                crate::UpgradeWireEntry {
                    id: 43,
                    kind: 33,
                    scalar_values: vec![],
                    ratio_values_raw: vec![],
                    bool_values: vec![true],
                    optional_ids: vec![],
                },
            ],
        );
        let before = core
            .upgrades()
            .entries()
            .iter()
            .map(|entry| (entry.id(), entry.kind()))
            .collect::<Vec<_>>();

        core.edit_snapshot(|parts| {
            parts.progress.gold = 250;
            parts.progress.left_dice = 3;
        })
        .expect("typed trigger fixture");
        core.trigger_gold_earned_upgrades();
        core.trigger_card_reroll_upgrades();
        core.trigger_stage_start_upgrades(1);

        let after = core
            .upgrades()
            .entries()
            .iter()
            .map(|entry| (entry.id(), entry.kind()))
            .collect::<Vec<_>>();
        assert_eq!(before, after);
        assert_eq!(core.upgrades().entries()[0].crock().damage_steps, 2);
        assert_eq!(core.upgrades().entries()[1].resolution().saved_rerolls, 3);
        assert!(!core.upgrades().entries()[2].membership_card().pending);
        assert!(core.stage_modifiers().free_shop_this_stage);
    }
}
