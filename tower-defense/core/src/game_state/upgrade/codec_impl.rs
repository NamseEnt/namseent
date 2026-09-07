#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub(crate) struct UpgradeWireEntry {
    pub(crate) id: u64,
    pub(crate) kind: u8,
    pub(crate) scalar_values: Vec<u64>,
    pub(crate) ratio_values_raw: Vec<i64>,
    pub(crate) bool_values: Vec<bool>,
    pub(crate) optional_ids: Vec<Option<u64>>,
}

impl UpgradeWireEntry {
    pub(super) fn upgrade_kind(&self) -> Result<crate::UpgradeKind, crate::CommandError> {
        crate::UpgradeKind::from_raw(self.kind)
            .ok_or(crate::CommandError::InvalidUpgradeKind { raw: self.kind })
    }
}

use super::behaviors::UpgradeRuntimeState;
pub use super::behaviors::{
    apple::AppleUpgradeState, backpack::BackpackUpgradeState, banana::BananaUpgradeState,
    black_white::BlackWhiteUpgradeState, broken_pottery::BrokenPotteryUpgradeState,
    camera::CameraUpgradeState, carrot::CarrotUpgradeState, cat::CatUpgradeState,
    crock::CrockUpgradeState, cup_noodles::CupNoodlesUpgradeState,
    demolition_hammer::DemolitionHammerUpgradeState, dice_bundle::DiceBundleUpgradeState,
    energy_drink::EnergyDrinkUpgradeState, fang::FangUpgradeState,
    four_leaf_clover::FourLeafCloverUpgradeState, french_fries::FrenchFriesUpgradeState,
    gift_box::GiftBoxUpgradeState, hamburger::HamburgerUpgradeState,
    ice_cream::IceCreamUpgradeState, membership_card::MembershipCardUpgradeState,
    metronome::MetronomeUpgradeState, mirror::MirrorUpgradeState, name_tag::NameTagUpgradeState,
    pea::PeaUpgradeState, perfect_pottery::PerfectPotteryUpgradeState,
    piggy_bank::PiggyBankUpgradeState, pizza::PizzaUpgradeState, popcorn::PopcornUpgradeState,
    rabbit::RabbitUpgradeState, resolution::ResolutionUpgradeState,
    shopping_bag::ShoppingBagUpgradeState, slot_machine::SlotMachineUpgradeState,
    spanner::SpannerUpgradeState, strawberry::StrawberryUpgradeState, tape::TapeUpgradeState,
    trophy::TrophyUpgradeState, watermelon::WatermelonUpgradeState,
};

pub(crate) trait UpgradeCodecSource {
    fn to_codec(&self) -> UpgradeWireEntry;
}

impl UpgradeCodecSource for UpgradeWireEntry {
    fn to_codec(&self) -> UpgradeWireEntry {
        self.clone()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpgradeCodecError {
    UnknownKind {
        actual: u8,
    },
    KindMismatch {
        expected: crate::UpgradeKind,
        actual: u8,
    },
    InvalidShape {
        kind: crate::UpgradeKind,
        field: &'static str,
        expected: usize,
        actual: usize,
    },
    IntegerOverflow {
        kind: crate::UpgradeKind,
        field: &'static str,
        value: u64,
    },
    UnsupportedRuntimeKind {
        kind: crate::UpgradeKind,
    },
}

macro_rules! empty_codec {
    ($name:ident, $kind:ident) => {
        impl $name {
            pub(crate) fn decode<T: UpgradeCodecSource>(
                entry: &T,
            ) -> Result<Self, UpgradeCodecError> {
                let entry = entry.to_codec();
                check_shape(&entry, crate::UpgradeKind::$kind, 0, 0, 0, 0)?;
                Ok(Self)
            }

            pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
                UpgradeWireEntry {
                    id,
                    kind: crate::UpgradeKind::$kind.raw(),
                    scalar_values: Vec::new(),
                    ratio_values_raw: Vec::new(),
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                }
            }
        }
    };
}

macro_rules! scalar_codec {
    ($name:ident, $kind:ident, $field:ident) => {
        impl $name {
            pub(crate) fn decode<T: UpgradeCodecSource>(
                entry: &T,
            ) -> Result<Self, UpgradeCodecError> {
                let entry = entry.to_codec();
                check_shape(&entry, crate::UpgradeKind::$kind, 1, 0, 0, 0)?;
                Ok(Self {
                    $field: usize_value(
                        crate::UpgradeKind::$kind,
                        stringify!($field),
                        entry.scalar_values[0],
                    )?,
                })
            }

            pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
                UpgradeWireEntry {
                    id,
                    kind: crate::UpgradeKind::$kind.raw(),
                    scalar_values: vec![self.$field as u64],
                    ratio_values_raw: Vec::new(),
                    bool_values: Vec::new(),
                    optional_ids: Vec::new(),
                }
            }
        }
    };
}

scalar_codec!(BackpackUpgradeState, Backpack, shop_slot_expand);
scalar_codec!(CatUpgradeState, Cat, gold_per_kill);
scalar_codec!(DiceBundleUpgradeState, DiceBundle, dice_chance_plus);
scalar_codec!(EnergyDrinkUpgradeState, EnergyDrink, discount);
scalar_codec!(FangUpgradeState, Fang, heal_per_kill);
scalar_codec!(GiftBoxUpgradeState, GiftBox, gold_per_item);
scalar_codec!(PiggyBankUpgradeState, PiggyBank, gold_per_step);
scalar_codec!(SlotMachineUpgradeState, SlotMachine, dice);
scalar_codec!(CrockUpgradeState, Crock, damage_steps);
scalar_codec!(MetronomeUpgradeState, Metronome, acquired_stage);
scalar_codec!(TapeUpgradeState, Tape, acquired_stage);

empty_codec!(AppleUpgradeState, Apple);
empty_codec!(BananaUpgradeState, Banana);
empty_codec!(CarrotUpgradeState, Carrot);
empty_codec!(BlackWhiteUpgradeState, BlackWhite);
empty_codec!(BrokenPotteryUpgradeState, BrokenPottery);
empty_codec!(CameraUpgradeState, Camera);
empty_codec!(CupNoodlesUpgradeState, CupNoodles);
empty_codec!(DemolitionHammerUpgradeState, DemolitionHammer);
empty_codec!(FourLeafCloverUpgradeState, FourLeafClover);
empty_codec!(FrenchFriesUpgradeState, FrenchFries);
empty_codec!(HamburgerUpgradeState, Hamburger);
empty_codec!(PeaUpgradeState, Pea);
empty_codec!(PizzaUpgradeState, Pizza);
empty_codec!(RabbitUpgradeState, Rabbit);
empty_codec!(ShoppingBagUpgradeState, ShoppingBag);
empty_codec!(SpannerUpgradeState, Spanner);
empty_codec!(StrawberryUpgradeState, Strawberry);
empty_codec!(TrophyUpgradeState, Trophy);
empty_codec!(WatermelonUpgradeState, Watermelon);

fn check_kind(
    entry: &UpgradeWireEntry,
    expected: crate::UpgradeKind,
) -> Result<(), UpgradeCodecError> {
    if entry.kind == expected.raw() {
        Ok(())
    } else {
        Err(UpgradeCodecError::KindMismatch {
            expected,
            actual: entry.kind,
        })
    }
}

fn check_len(
    _: &UpgradeWireEntry,
    kind: crate::UpgradeKind,
    field: &'static str,
    actual: usize,
    expected: usize,
) -> Result<(), UpgradeCodecError> {
    if actual == expected {
        Ok(())
    } else {
        Err(UpgradeCodecError::InvalidShape {
            kind,
            field,
            expected,
            actual,
        })
    }
}

fn check_shape(
    entry: &UpgradeWireEntry,
    kind: crate::UpgradeKind,
    scalar: usize,
    ratio: usize,
    boolean: usize,
    optional: usize,
) -> Result<(), UpgradeCodecError> {
    check_kind(entry, kind)?;
    check_len(
        entry,
        kind,
        "scalar_values",
        entry.scalar_values.len(),
        scalar,
    )?;
    check_len(
        entry,
        kind,
        "ratio_values_raw",
        entry.ratio_values_raw.len(),
        ratio,
    )?;
    check_len(entry, kind, "bool_values", entry.bool_values.len(), boolean)?;
    check_len(
        entry,
        kind,
        "optional_ids",
        entry.optional_ids.len(),
        optional,
    )
}

fn usize_value(
    kind: crate::UpgradeKind,
    field: &'static str,
    value: u64,
) -> Result<usize, UpgradeCodecError> {
    usize::try_from(value).map_err(|_| UpgradeCodecError::IntegerOverflow { kind, field, value })
}

impl PopcornUpgradeState {
    pub(crate) fn decode<T: UpgradeCodecSource>(entry: &T) -> Result<Self, UpgradeCodecError> {
        let entry = entry.to_codec();
        check_shape(&entry, crate::UpgradeKind::Popcorn, 2, 2, 0, 0)?;
        Ok(Self {
            max_multiplier_raw: entry.ratio_values_raw[0],
            duration_waves: usize_value(
                crate::UpgradeKind::Popcorn,
                "duration_waves",
                entry.scalar_values[0],
            )?,
            active_multiplier_raw: entry.ratio_values_raw[1],
            waves_remaining: usize_value(
                crate::UpgradeKind::Popcorn,
                "waves_remaining",
                entry.scalar_values[1],
            )?,
        })
    }

    pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
        UpgradeWireEntry {
            id,
            kind: crate::UpgradeKind::Popcorn.raw(),
            scalar_values: vec![self.duration_waves as u64, self.waves_remaining as u64],
            ratio_values_raw: vec![self.max_multiplier_raw, self.active_multiplier_raw],
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        }
    }
}

impl NameTagUpgradeState {
    pub(crate) fn decode<T: UpgradeCodecSource>(entry: &T) -> Result<Self, UpgradeCodecError> {
        let entry = entry.to_codec();
        check_shape(&entry, crate::UpgradeKind::NameTag, 0, 1, 0, 1)?;
        Ok(Self {
            bonus_raw: entry.ratio_values_raw[0],
            tower_id: entry.optional_ids[0],
        })
    }

    pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
        UpgradeWireEntry {
            id,
            kind: crate::UpgradeKind::NameTag.raw(),
            scalar_values: Vec::new(),
            ratio_values_raw: vec![self.bonus_raw],
            bool_values: Vec::new(),
            optional_ids: vec![self.tower_id],
        }
    }
}

impl ResolutionUpgradeState {
    pub(crate) fn decode<T: UpgradeCodecSource>(entry: &T) -> Result<Self, UpgradeCodecError> {
        let entry = entry.to_codec();
        check_shape(&entry, crate::UpgradeKind::Resolution, 1, 1, 0, 0)?;
        Ok(Self {
            reroll_damage_raw: entry.ratio_values_raw[0],
            saved_rerolls: usize_value(
                crate::UpgradeKind::Resolution,
                "saved_rerolls",
                entry.scalar_values[0],
            )?,
        })
    }

    pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
        UpgradeWireEntry {
            id,
            kind: crate::UpgradeKind::Resolution.raw(),
            scalar_values: vec![self.saved_rerolls as u64],
            ratio_values_raw: vec![self.reroll_damage_raw],
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        }
    }
}

impl IceCreamUpgradeState {
    pub(crate) fn decode<T: UpgradeCodecSource>(entry: &T) -> Result<Self, UpgradeCodecError> {
        let entry = entry.to_codec();
        check_shape(&entry, crate::UpgradeKind::IceCream, 1, 1, 0, 0)?;
        Ok(Self {
            damage_bonus_raw: entry.ratio_values_raw[0],
            waves_remaining: usize_value(
                crate::UpgradeKind::IceCream,
                "waves_remaining",
                entry.scalar_values[0],
            )?,
        })
    }

    pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
        UpgradeWireEntry {
            id,
            kind: crate::UpgradeKind::IceCream.raw(),
            scalar_values: vec![self.waves_remaining as u64],
            ratio_values_raw: vec![self.damage_bonus_raw],
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        }
    }
}

impl PerfectPotteryUpgradeState {
    pub(crate) fn decode<T: UpgradeCodecSource>(entry: &T) -> Result<Self, UpgradeCodecError> {
        let entry = entry.to_codec();
        check_shape(&entry, crate::UpgradeKind::PerfectPottery, 0, 1, 0, 0)?;
        Ok(Self {
            damage_bonus_raw: entry.ratio_values_raw[0],
        })
    }

    pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
        UpgradeWireEntry {
            id,
            kind: crate::UpgradeKind::PerfectPottery.raw(),
            scalar_values: Vec::new(),
            ratio_values_raw: vec![self.damage_bonus_raw],
            bool_values: Vec::new(),
            optional_ids: Vec::new(),
        }
    }
}

macro_rules! bool_codec {
    ($name:ident, $kind:ident) => {
        impl $name {
            pub(crate) fn decode<T: UpgradeCodecSource>(
                entry: &T,
            ) -> Result<Self, UpgradeCodecError> {
                let entry = entry.to_codec();
                check_shape(&entry, crate::UpgradeKind::$kind, 0, 0, 1, 0)?;
                Ok(Self {
                    pending: entry.bool_values[0],
                })
            }

            pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
                UpgradeWireEntry {
                    id,
                    kind: crate::UpgradeKind::$kind.raw(),
                    scalar_values: Vec::new(),
                    ratio_values_raw: Vec::new(),
                    bool_values: vec![self.pending],
                    optional_ids: Vec::new(),
                }
            }
        }
    };
}

bool_codec!(MirrorUpgradeState, Mirror);
bool_codec!(MembershipCardUpgradeState, MembershipCard);

impl UpgradeRuntimeState {
    pub fn kind(&self) -> crate::UpgradeKind {
        match self {
            Self::Apple(_) => crate::UpgradeKind::Apple,
            Self::Banana(_) => crate::UpgradeKind::Banana,
            Self::Carrot(_) => crate::UpgradeKind::Carrot,
            Self::Cat(_) => crate::UpgradeKind::Cat,
            Self::Backpack(_) => crate::UpgradeKind::Backpack,
            Self::DiceBundle(_) => crate::UpgradeKind::DiceBundle,
            Self::EnergyDrink(_) => crate::UpgradeKind::EnergyDrink,
            Self::Popcorn(_) => crate::UpgradeKind::Popcorn,
            Self::FourLeafClover(_) => crate::UpgradeKind::FourLeafClover,
            Self::Rabbit(_) => crate::UpgradeKind::Rabbit,
            Self::BlackWhite(_) => crate::UpgradeKind::BlackWhite,
            Self::Trophy(_) => crate::UpgradeKind::Trophy,
            Self::Crock(_) => crate::UpgradeKind::Crock,
            Self::CupNoodles(_) => crate::UpgradeKind::CupNoodles,
            Self::FrenchFries(_) => crate::UpgradeKind::FrenchFries,
            Self::Hamburger(_) => crate::UpgradeKind::Hamburger,
            Self::Pizza(_) => crate::UpgradeKind::Pizza,
            Self::DemolitionHammer(_) => crate::UpgradeKind::DemolitionHammer,
            Self::Metronome(_) => crate::UpgradeKind::Metronome,
            Self::Tape(_) => crate::UpgradeKind::Tape,
            Self::NameTag(_) => crate::UpgradeKind::NameTag,
            Self::ShoppingBag(_) => crate::UpgradeKind::ShoppingBag,
            Self::Resolution(_) => crate::UpgradeKind::Resolution,
            Self::Mirror(_) => crate::UpgradeKind::Mirror,
            Self::IceCream(_) => crate::UpgradeKind::IceCream,
            Self::Spanner(_) => crate::UpgradeKind::Spanner,
            Self::Pea(_) => crate::UpgradeKind::Pea,
            Self::SlotMachine(_) => crate::UpgradeKind::SlotMachine,
            Self::PiggyBank(_) => crate::UpgradeKind::PiggyBank,
            Self::Camera(_) => crate::UpgradeKind::Camera,
            Self::GiftBox(_) => crate::UpgradeKind::GiftBox,
            Self::Fang(_) => crate::UpgradeKind::Fang,
            Self::PerfectPottery(_) => crate::UpgradeKind::PerfectPottery,
            Self::MembershipCard(_) => crate::UpgradeKind::MembershipCard,
            Self::BrokenPottery(_) => crate::UpgradeKind::BrokenPottery,
            Self::Strawberry(_) => crate::UpgradeKind::Strawberry,
            Self::Watermelon(_) => crate::UpgradeKind::Watermelon,
        }
    }

    pub(crate) fn decode(entry: &UpgradeWireEntry) -> Result<Self, UpgradeCodecError> {
        match entry
            .upgrade_kind()
            .map_err(|_| UpgradeCodecError::UnknownKind { actual: entry.kind })?
        {
            crate::UpgradeKind::Apple => Ok(Self::Apple(AppleUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Banana => Ok(Self::Banana(BananaUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Carrot => Ok(Self::Carrot(CarrotUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Cat => Ok(Self::Cat(CatUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Backpack => {
                Ok(Self::Backpack(BackpackUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::DiceBundle => {
                Ok(Self::DiceBundle(DiceBundleUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::EnergyDrink => {
                Ok(Self::EnergyDrink(EnergyDrinkUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Popcorn => Ok(Self::Popcorn(PopcornUpgradeState::decode(entry)?)),
            crate::UpgradeKind::FourLeafClover => Ok(Self::FourLeafClover(
                FourLeafCloverUpgradeState::decode(entry)?,
            )),
            crate::UpgradeKind::Rabbit => Ok(Self::Rabbit(RabbitUpgradeState::decode(entry)?)),
            crate::UpgradeKind::BlackWhite => {
                Ok(Self::BlackWhite(BlackWhiteUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Trophy => Ok(Self::Trophy(TrophyUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Crock => Ok(Self::Crock(CrockUpgradeState::decode(entry)?)),
            crate::UpgradeKind::CupNoodles => {
                Ok(Self::CupNoodles(CupNoodlesUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::FrenchFries => {
                Ok(Self::FrenchFries(FrenchFriesUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Hamburger => {
                Ok(Self::Hamburger(HamburgerUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Pizza => Ok(Self::Pizza(PizzaUpgradeState::decode(entry)?)),
            crate::UpgradeKind::DemolitionHammer => Ok(Self::DemolitionHammer(
                DemolitionHammerUpgradeState::decode(entry)?,
            )),
            crate::UpgradeKind::Metronome => {
                Ok(Self::Metronome(MetronomeUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Tape => Ok(Self::Tape(TapeUpgradeState::decode(entry)?)),
            crate::UpgradeKind::NameTag => Ok(Self::NameTag(NameTagUpgradeState::decode(entry)?)),
            crate::UpgradeKind::ShoppingBag => {
                Ok(Self::ShoppingBag(ShoppingBagUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Resolution => {
                Ok(Self::Resolution(ResolutionUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::IceCream => {
                Ok(Self::IceCream(IceCreamUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Mirror => Ok(Self::Mirror(MirrorUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Spanner => Ok(Self::Spanner(SpannerUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Pea => Ok(Self::Pea(PeaUpgradeState::decode(entry)?)),
            crate::UpgradeKind::SlotMachine => {
                Ok(Self::SlotMachine(SlotMachineUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::PiggyBank => {
                Ok(Self::PiggyBank(PiggyBankUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Camera => Ok(Self::Camera(CameraUpgradeState::decode(entry)?)),
            crate::UpgradeKind::GiftBox => Ok(Self::GiftBox(GiftBoxUpgradeState::decode(entry)?)),
            crate::UpgradeKind::Fang => Ok(Self::Fang(FangUpgradeState::decode(entry)?)),
            crate::UpgradeKind::PerfectPottery => Ok(Self::PerfectPottery(
                PerfectPotteryUpgradeState::decode(entry)?,
            )),
            crate::UpgradeKind::MembershipCard => Ok(Self::MembershipCard(
                MembershipCardUpgradeState::decode(entry)?,
            )),
            crate::UpgradeKind::BrokenPottery => Ok(Self::BrokenPottery(
                BrokenPotteryUpgradeState::decode(entry)?,
            )),
            crate::UpgradeKind::Strawberry => {
                Ok(Self::Strawberry(StrawberryUpgradeState::decode(entry)?))
            }
            crate::UpgradeKind::Watermelon => {
                Ok(Self::Watermelon(WatermelonUpgradeState::decode(entry)?))
            }
        }
    }

    pub(crate) fn encode(self, id: u64) -> UpgradeWireEntry {
        match self {
            Self::Apple(state) => state.encode(id),
            Self::Banana(state) => state.encode(id),
            Self::Carrot(state) => state.encode(id),
            Self::Cat(state) => state.encode(id),
            Self::Backpack(state) => state.encode(id),
            Self::DiceBundle(state) => state.encode(id),
            Self::EnergyDrink(state) => state.encode(id),
            Self::Popcorn(state) => state.encode(id),
            Self::FourLeafClover(state) => state.encode(id),
            Self::Rabbit(state) => state.encode(id),
            Self::BlackWhite(state) => state.encode(id),
            Self::Trophy(state) => state.encode(id),
            Self::Crock(state) => state.encode(id),
            Self::CupNoodles(state) => state.encode(id),
            Self::FrenchFries(state) => state.encode(id),
            Self::Hamburger(state) => state.encode(id),
            Self::Pizza(state) => state.encode(id),
            Self::DemolitionHammer(state) => state.encode(id),
            Self::Metronome(state) => state.encode(id),
            Self::Tape(state) => state.encode(id),
            Self::NameTag(state) => state.encode(id),
            Self::ShoppingBag(state) => state.encode(id),
            Self::Resolution(state) => state.encode(id),
            Self::Mirror(state) => state.encode(id),
            Self::IceCream(state) => state.encode(id),
            Self::Spanner(state) => state.encode(id),
            Self::Pea(state) => state.encode(id),
            Self::SlotMachine(state) => state.encode(id),
            Self::PiggyBank(state) => state.encode(id),
            Self::Camera(state) => state.encode(id),
            Self::GiftBox(state) => state.encode(id),
            Self::Fang(state) => state.encode(id),
            Self::PerfectPottery(state) => state.encode(id),
            Self::MembershipCard(state) => state.encode(id),
            Self::BrokenPottery(state) => state.encode(id),
            Self::Strawberry(state) => state.encode(id),
            Self::Watermelon(state) => state.encode(id),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trip(entry: UpgradeWireEntry) {
        let runtime = UpgradeRuntimeState::decode(&entry).expect("canonical codec decodes");
        assert_eq!(runtime.encode(entry.id), entry);
    }

    #[test]
    fn migrated_states_have_canonical_round_trips() {
        assert_round_trip(
            PopcornUpgradeState {
                max_multiplier_raw: 5_000_000,
                duration_waves: 5,
                active_multiplier_raw: 4_000_000,
                waves_remaining: 4,
            }
            .encode(1),
        );
        assert_round_trip(
            NameTagUpgradeState {
                bonus_raw: 2_000_000,
                tower_id: Some(7),
            }
            .encode(2),
        );
        assert_round_trip(
            ResolutionUpgradeState {
                reroll_damage_raw: 250_000,
                saved_rerolls: 3,
            }
            .encode(3),
        );
        assert_round_trip(
            IceCreamUpgradeState {
                damage_bonus_raw: 3_000_000,
                waves_remaining: 2,
            }
            .encode(4),
        );
        assert_round_trip(
            PerfectPotteryUpgradeState {
                damage_bonus_raw: 500_000,
            }
            .encode(5),
        );
        assert_round_trip(MirrorUpgradeState { pending: true }.encode(6));
        assert_round_trip(MembershipCardUpgradeState { pending: false }.encode(7));
    }

    #[test]
    fn malformed_codec_shapes_are_rejected() {
        let mut entry = PopcornUpgradeState {
            max_multiplier_raw: 5_000_000,
            duration_waves: 5,
            active_multiplier_raw: 0,
            waves_remaining: 5,
        }
        .encode(1);
        entry.scalar_values.pop();
        assert!(matches!(
            PopcornUpgradeState::decode(&entry),
            Err(UpgradeCodecError::InvalidShape {
                field: "scalar_values",
                expected: 2,
                actual: 1,
                ..
            })
        ));

        let mut entry = MirrorUpgradeState { pending: true }.encode(2);
        entry.ratio_values_raw.push(1);
        assert!(matches!(
            MirrorUpgradeState::decode(&entry),
            Err(UpgradeCodecError::InvalidShape {
                field: "ratio_values_raw",
                expected: 0,
                actual: 1,
                ..
            })
        ));

        let mut entry = BackpackUpgradeState {
            shop_slot_expand: 1,
        }
        .encode(3);
        entry.ratio_values_raw.push(1);
        assert!(matches!(
            BackpackUpgradeState::decode(&entry),
            Err(UpgradeCodecError::InvalidShape {
                field: "ratio_values_raw",
                ..
            })
        ));
    }

    #[test]
    fn every_upgrade_state_has_a_strict_runtime_round_trip() {
        for &kind in crate::UpgradeKind::ALL {
            let entry = crate::game_state::upgrade::generated_upgrade(kind).to_wire();
            let runtime = UpgradeRuntimeState::decode(&entry)
                .unwrap_or_else(|error| panic!("codec entry for {kind:?} must decode: {error:?}"));
            assert_eq!(runtime.encode(entry.id), entry, "codec entry for {kind:?}");
        }
    }

    #[test]
    fn popcorn_state_keeps_legacy_wave_interpolation() {
        let state = PopcornUpgradeState {
            max_multiplier_raw: 5_000_000,
            duration_waves: 5,
            active_multiplier_raw: 0,
            waves_remaining: 5,
        };
        assert_eq!(
            (0..=5)
                .rev()
                .map(|waves| state.damage_bonus_raw(waves))
                .collect::<Vec<_>>(),
            vec![4_000_000, 3_000_000, 2_000_000, 1_000_000, 0, 0]
        );
    }
}
