use crate::game_state::upgrade::*;
use crate::l10n::Locale;
use crate::theme::typography::TypographyBuilder;

mod combat;
mod rule;
mod special;
mod treasure;

pub trait UpgradeTypeL10n {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale);
    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale);
}

impl UpgradeTypeL10n for crate::game_state::upgrade::Upgrade {
    fn l10n_name<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match self {
            Upgrade::Cat(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Staff(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::LongSword(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Mace(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::ClubSword(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Backpack(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::DiceBundle(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Tricycle(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::EnergyDrink(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::PerfectPottery(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::SingleChopstick(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::PairChopsticks(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::FountainPen(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Brush(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::FourLeafClover(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Rabbit(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::BlackWhite(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Trophy(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Crock(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::DemolitionHammer(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Metronome(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Tape(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::NameTag(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::ShoppingBag(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Resolution(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Mirror(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::IceCream(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Spanner(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Pea(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::SlotMachine(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::PiggyBank(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Camera(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::GiftBox(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Fang(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Popcorn(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::MembershipCard(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::Eraser(upgrade) => upgrade.l10n_name(builder, locale),
            Upgrade::BrokenPottery(upgrade) => upgrade.l10n_name(builder, locale),
        }
    }

    fn l10n_description<'a>(&self, builder: &mut TypographyBuilder<'a>, locale: &Locale) {
        match self {
            Upgrade::Cat(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Staff(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::LongSword(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Mace(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::ClubSword(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Backpack(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::DiceBundle(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Tricycle(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::EnergyDrink(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::PerfectPottery(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::SingleChopstick(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::PairChopsticks(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::FountainPen(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Brush(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::FourLeafClover(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Rabbit(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::BlackWhite(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Trophy(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Crock(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::DemolitionHammer(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Metronome(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Tape(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::NameTag(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::ShoppingBag(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Resolution(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Mirror(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::IceCream(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Spanner(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Pea(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::SlotMachine(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::PiggyBank(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Camera(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::GiftBox(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Fang(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Popcorn(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::MembershipCard(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::Eraser(upgrade) => upgrade.l10n_description(builder, locale),
            Upgrade::BrokenPottery(upgrade) => upgrade.l10n_description(builder, locale),
        }
    }
}
