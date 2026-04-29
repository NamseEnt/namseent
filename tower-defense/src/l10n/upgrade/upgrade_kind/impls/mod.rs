use crate::game_state::upgrade::UpgradeKind;
use crate::l10n::Locale;
use crate::theme::typography::TypographyBuilder;

mod combat;
mod rule;
mod special;
mod treasure;

pub trait UpgradeKindL10n {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale);
    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale);
}

impl UpgradeKindL10n for UpgradeKind {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        use crate::game_state::upgrade::UpgradeKind::*;

        match self {
            Cat(u) => u.l10n_name(builder, locale),
            Staff(u) => u.l10n_name(builder, locale),
            LongSword(u) => u.l10n_name(builder, locale),
            Mace(u) => u.l10n_name(builder, locale),
            ClubSword(u) => u.l10n_name(builder, locale),
            Backpack(u) => u.l10n_name(builder, locale),
            DiceBundle(u) => u.l10n_name(builder, locale),
            Tricycle(u) => u.l10n_name(builder, locale),
            EnergyDrink(u) => u.l10n_name(builder, locale),
            PerfectPottery(u) => u.l10n_name(builder, locale),
            SingleChopstick(u) => u.l10n_name(builder, locale),
            PairChopsticks(u) => u.l10n_name(builder, locale),
            FountainPen(u) => u.l10n_name(builder, locale),
            Brush(u) => u.l10n_name(builder, locale),
            BrokenPottery(u) => u.l10n_name(builder, locale),
            FourLeafClover(u) => u.l10n_name(builder, locale),
            Rabbit(u) => u.l10n_name(builder, locale),
            BlackWhite(u) => u.l10n_name(builder, locale),
            Eraser(u) => u.l10n_name(builder, locale),
            Trophy(u) => u.l10n_name(builder, locale),
            Crock(u) => u.l10n_name(builder, locale),
            DemolitionHammer(u) => u.l10n_name(builder, locale),
            Metronome(u) => u.l10n_name(builder, locale),
            Tape(u) => u.l10n_name(builder, locale),
            NameTag(u) => u.l10n_name(builder, locale),
            ShoppingBag(u) => u.l10n_name(builder, locale),
            Resolution(u) => u.l10n_name(builder, locale),
            Mirror(u) => u.l10n_name(builder, locale),
            IceCream(u) => u.l10n_name(builder, locale),
            Spanner(u) => u.l10n_name(builder, locale),
            Pea(u) => u.l10n_name(builder, locale),
            SlotMachine(u) => u.l10n_name(builder, locale),
            PiggyBank(u) => u.l10n_name(builder, locale),
            Camera(u) => u.l10n_name(builder, locale),
            GiftBox(u) => u.l10n_name(builder, locale),
            Fang(u) => u.l10n_name(builder, locale),
            Popcorn(u) => u.l10n_name(builder, locale),
            MembershipCard(u) => u.l10n_name(builder, locale),
        }
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        use crate::game_state::upgrade::UpgradeKind::*;

        match self {
            Cat(u) => u.l10n_description(builder, locale),
            Staff(u) => u.l10n_description(builder, locale),
            LongSword(u) => u.l10n_description(builder, locale),
            Mace(u) => u.l10n_description(builder, locale),
            ClubSword(u) => u.l10n_description(builder, locale),
            Backpack(u) => u.l10n_description(builder, locale),
            DiceBundle(u) => u.l10n_description(builder, locale),
            Tricycle(u) => u.l10n_description(builder, locale),
            EnergyDrink(u) => u.l10n_description(builder, locale),
            PerfectPottery(u) => u.l10n_description(builder, locale),
            SingleChopstick(u) => u.l10n_description(builder, locale),
            PairChopsticks(u) => u.l10n_description(builder, locale),
            FountainPen(u) => u.l10n_description(builder, locale),
            Brush(u) => u.l10n_description(builder, locale),
            BrokenPottery(u) => u.l10n_description(builder, locale),
            FourLeafClover(u) => u.l10n_description(builder, locale),
            Rabbit(u) => u.l10n_description(builder, locale),
            BlackWhite(u) => u.l10n_description(builder, locale),
            Eraser(u) => u.l10n_description(builder, locale),
            Trophy(u) => u.l10n_description(builder, locale),
            Crock(u) => u.l10n_description(builder, locale),
            DemolitionHammer(u) => u.l10n_description(builder, locale),
            Metronome(u) => u.l10n_description(builder, locale),
            Tape(u) => u.l10n_description(builder, locale),
            NameTag(u) => u.l10n_description(builder, locale),
            ShoppingBag(u) => u.l10n_description(builder, locale),
            Resolution(u) => u.l10n_description(builder, locale),
            Mirror(u) => u.l10n_description(builder, locale),
            IceCream(u) => u.l10n_description(builder, locale),
            Spanner(u) => u.l10n_description(builder, locale),
            Pea(u) => u.l10n_description(builder, locale),
            SlotMachine(u) => u.l10n_description(builder, locale),
            PiggyBank(u) => u.l10n_description(builder, locale),
            Camera(u) => u.l10n_description(builder, locale),
            GiftBox(u) => u.l10n_description(builder, locale),
            Fang(u) => u.l10n_description(builder, locale),
            Popcorn(u) => u.l10n_description(builder, locale),
            MembershipCard(u) => u.l10n_description(builder, locale),
        }
    }
}
