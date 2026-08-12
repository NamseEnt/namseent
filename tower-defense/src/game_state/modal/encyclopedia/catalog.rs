use crate::game_state::card_service::{CardService, CardServiceBehavior, CardServiceDiscriminants};
use crate::game_state::item::{Item, ItemBehavior, ItemDiscriminants};
use crate::game_state::upgrade::{Upgrade, UpgradeBehavior, UpgradeDiscriminants, UpgradeState};
use crate::rarity::Rarity;
use crate::tooltip::TooltipContent;
use namui::*;
use rand::{SeedableRng, rngs::StdRng};
use strum::IntoEnumIterator;
#[derive(Clone, Copy, Debug, PartialEq, Eq, State)]
pub(super) enum EntryKind {
    Item,
    CardService,
    Treasure,
}

#[derive(Clone, Debug, PartialEq, State)]
pub(super) enum EntryContent {
    Item(Item),
    CardService(CardService),
    Treasure(Upgrade),
}

#[derive(Clone, Debug, PartialEq, State)]
pub(super) struct Entry {
    pub(super) kind: EntryKind,
    pub(super) rarity: Rarity,
    pub(super) key: String,
    pub(super) content: EntryContent,
}

impl Entry {
    pub(super) fn tooltip(&self) -> TooltipContent {
        match &self.content {
            EntryContent::Item(item) => TooltipContent::Item(item.clone()),
            EntryContent::CardService(card_service) => {
                TooltipContent::CardService(card_service.clone())
            }
            EntryContent::Treasure(treasure) => TooltipContent::Upgrade(*treasure),
        }
    }
}

pub(super) fn all_entries() -> Vec<Entry> {
    let mut rng = StdRng::seed_from_u64(0x454E_4359_434C_4F50);
    let default_upgrade_state = UpgradeState::default();
    let mut entries = Vec::new();

    for discriminant in ItemDiscriminants::iter() {
        let item = discriminant.generate(&mut rng);
        entries.push(Entry {
            kind: EntryKind::Item,
            rarity: discriminant.rarity(),
            key: item.key().to_string(),
            content: EntryContent::Item(item),
        });
    }
    for discriminant in CardServiceDiscriminants::iter() {
        let card_service = discriminant.generate();
        entries.push(Entry {
            kind: EntryKind::CardService,
            rarity: discriminant.rarity(),
            key: card_service.key().to_string(),
            content: EntryContent::CardService(card_service),
        });
    }
    for discriminant in UpgradeDiscriminants::iter() {
        let treasure = discriminant.generate(&default_upgrade_state);
        entries.push(Entry {
            kind: EntryKind::Treasure,
            rarity: discriminant.rarity(),
            key: treasure.key().to_string(),
            content: EntryContent::Treasure(treasure),
        });
    }
    entries
}
