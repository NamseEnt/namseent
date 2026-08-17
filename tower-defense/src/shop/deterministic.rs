use super::{ShopSlot, ShopSlotData};
use crate::deterministic_rng::{self, domain};
use crate::game_state::rng::GameRngState;
use crate::game_state::{
    GameState,
    card_service::CardServiceDiscriminants,
    item::ItemDiscriminants,
    upgrade::{UpgradeDiscriminants, UpgradeState},
};
use crate::rarity::Rarity;
use crate::shop::bag;
use namui::*;
use strum::IntoEnumIterator;

pub const ITEM_CATEGORY: usize = 0;
pub const CARD_SERVICE_CATEGORY: usize = 1;
pub const UPGRADE_CATEGORY: usize = 2;

#[derive(Clone, Debug, Default, State)]
pub struct ShopGenerationStats {
    pub exact_match_count: usize,
    pub rarity_fallback_count: usize,
    pub category_fallback_count: usize,
    pub unavoidable_duplicate_count: usize,
}

#[derive(Clone, Debug, State)]
pub struct GeneratedShopScreen {
    pub slots: Vec<ShopSlotData>,
    pub stats: ShopGenerationStats,
}

#[derive(Clone, Debug)]
struct Candidate {
    key: String,
    rarity: Rarity,
    kind: CandidateKind,
}

#[derive(Clone, Copy, Debug)]
enum CandidateKind {
    Item(ItemDiscriminants),
    CardService(CardServiceDiscriminants),
    Upgrade(UpgradeDiscriminants),
}

impl Candidate {
    fn stable_key(category: usize, key: &str) -> String {
        let prefix = match category {
            ITEM_CATEGORY => "item",
            CARD_SERVICE_CATEGORY => "card_service",
            UPGRADE_CATEGORY => "upgrade",
            _ => unreachable!(),
        };
        format!("{prefix}:{key}")
    }
}

fn all_candidates(
    game_state: &GameState,
    category: usize,
    rarity: Option<Rarity>,
) -> Vec<Candidate> {
    let mut candidates: Vec<Candidate> = match category {
        ITEM_CATEGORY => ItemDiscriminants::iter()
            .map(|kind| Candidate {
                key: Candidate::stable_key(category, kind.as_ref()),
                rarity: kind.rarity(),
                kind: CandidateKind::Item(kind),
            })
            .collect(),
        CARD_SERVICE_CATEGORY => CardServiceDiscriminants::iter()
            .map(|kind| Candidate {
                key: Candidate::stable_key(category, kind.as_ref()),
                rarity: kind.rarity(),
                kind: CandidateKind::CardService(kind),
            })
            .collect(),
        UPGRADE_CATEGORY => UpgradeDiscriminants::iter()
            .filter(|kind| {
                kind.current_and_max(&game_state.upgrade_state)
                    .is_none_or(|(current, max)| current < max)
            })
            .map(|kind| Candidate {
                key: Candidate::stable_key(category, kind.as_ref()),
                rarity: kind.rarity(),
                kind: CandidateKind::Upgrade(kind),
            })
            .collect(),
        _ => unreachable!(),
    };
    candidates.sort_by(|left, right| left.key.cmp(&right.key));
    if let Some(rarity) = rarity {
        candidates.retain(|candidate| candidate.rarity == rarity);
    }
    candidates
}

fn eligible_rarities(game_state: &GameState, category: usize) -> Vec<Rarity> {
    [
        Rarity::Common,
        Rarity::Rare,
        Rarity::Epic,
        Rarity::Legendary,
    ]
    .into_iter()
    .filter(|rarity| !all_candidates(game_state, category, Some(*rarity)).is_empty())
    .collect()
}

fn candidate_for_key<'a>(candidates: &'a [Candidate], key: &str) -> Option<&'a Candidate> {
    candidates.iter().find(|candidate| candidate.key == key)
}

fn candidate_keys(candidates: &[Candidate]) -> Vec<String> {
    candidates
        .iter()
        .map(|candidate| candidate.key.clone())
        .collect()
}

fn generate_candidate(
    candidate: &Candidate,
    upgrade_state: &UpgradeState,
    rng_state: &GameRngState,
    sequence: u64,
    slot_index: usize,
) -> ShopSlot {
    match candidate.kind {
        CandidateKind::Item(kind) => {
            let mut rng = rng_state.rng_for(
                domain::SHOP_ITEM_PAYLOAD,
                &[
                    sequence,
                    slot_index as u64,
                    deterministic_rng::stable_key_hash(&candidate.key),
                ],
            );
            ShopSlot::Item {
                item: kind.generate(&mut rng),
                cost: 0,
            }
        }
        CandidateKind::CardService(kind) => ShopSlot::CardService {
            card_service: kind.generate(),
            cost: 0,
        },
        CandidateKind::Upgrade(kind) => ShopSlot::Upgrade {
            upgrade: kind.generate(upgrade_state),
            cost: 0,
        },
    }
}

fn content_key_for_slot(slot: &ShopSlot) -> String {
    match slot {
        ShopSlot::Item { item, .. } => {
            Candidate::stable_key(ITEM_CATEGORY, item.discriminant().as_ref())
        }
        ShopSlot::CardService { card_service, .. } => Candidate::stable_key(
            CARD_SERVICE_CATEGORY,
            CardServiceDiscriminants::from(card_service).as_ref(),
        ),
        ShopSlot::Upgrade { upgrade, .. } => {
            Candidate::stable_key(UPGRADE_CATEGORY, upgrade.discriminant().as_ref())
        }
    }
}

fn apply_deterministic_cost(
    slot: &mut ShopSlot,
    state: &GameRngState,
    sequence: u64,
    slot_index: usize,
    free: bool,
    discount: usize,
) {
    let base_cost = match slot {
        ShopSlot::Item { .. } => 20,
        ShopSlot::Upgrade { .. } => 95,
        ShopSlot::CardService { .. } => 45,
    };
    let key = content_key_for_slot(slot);
    let mut rng = state.rng_for(
        domain::SHOP_PRICE,
        &[
            sequence,
            slot_index as u64,
            deterministic_rng::stable_key_hash(&key),
        ],
    );
    let additional_basis_points = deterministic_rng::uniform_index(&mut rng, 50_001) as u64;
    let cost = super::calculate_cost_basis_points(
        base_cost,
        slot.rarity(),
        additional_basis_points,
        free,
        discount,
    );
    match slot {
        ShopSlot::Item {
            cost: slot_cost, ..
        }
        | ShopSlot::Upgrade {
            cost: slot_cost, ..
        }
        | ShopSlot::CardService {
            cost: slot_cost, ..
        } => *slot_cost = cost,
    }
}

fn draw_content(
    game_state: &mut GameState,
    category: usize,
    rarity: Rarity,
    screen_seen: &[String],
) -> (Option<Candidate>, bool) {
    let candidates = all_candidates(game_state, category, Some(rarity));
    let keys = candidate_keys(&candidates);
    let key = bag::draw_content_key(&mut game_state.rng, category, rarity, &keys, screen_seen);
    (
        key.as_deref()
            .and_then(|key| candidate_for_key(&candidates, key))
            .cloned(),
        key.is_some(),
    )
}

fn choose_candidate(
    game_state: &mut GameState,
    target_category: usize,
    target_rarity: Rarity,
    screen_seen: &[String],
) -> (Candidate, bool, bool, bool) {
    if let (Some(candidate), true) =
        draw_content(game_state, target_category, target_rarity, screen_seen)
        && !screen_seen.contains(&candidate.key)
    {
        return (candidate, false, false, false);
    }

    for rarity in [
        Rarity::Common,
        Rarity::Rare,
        Rarity::Epic,
        Rarity::Legendary,
    ] {
        if rarity == target_rarity {
            continue;
        }
        if let (Some(candidate), true) =
            draw_content(game_state, target_category, rarity, screen_seen)
            && !screen_seen.contains(&candidate.key)
        {
            return (candidate, true, false, false);
        }
    }

    for category in [ITEM_CATEGORY, CARD_SERVICE_CATEGORY, UPGRADE_CATEGORY] {
        if category == target_category {
            continue;
        }
        for rarity in [
            Rarity::Common,
            Rarity::Rare,
            Rarity::Epic,
            Rarity::Legendary,
        ] {
            if let (Some(candidate), true) = draw_content(game_state, category, rarity, screen_seen)
                && !screen_seen.contains(&candidate.key)
            {
                return (candidate, false, true, false);
            }
        }
    }

    let mut all = Vec::new();
    for category in [ITEM_CATEGORY, CARD_SERVICE_CATEGORY, UPGRADE_CATEGORY] {
        all.extend(all_candidates(game_state, category, None));
    }
    all.sort_by(|left, right| left.key.cmp(&right.key));
    let candidate = all
        .iter()
        .find(|candidate| !screen_seen.contains(&candidate.key))
        .or_else(|| all.first())
        .cloned()
        .expect("shop must have at least one eligible candidate");
    (candidate, false, false, true)
}

pub fn generate_shop_screen_with_stats(
    game_state: &mut GameState,
    slot_count: usize,
    existing_slots: &[ShopSlotData],
    forced_category: Option<usize>,
) -> GeneratedShopScreen {
    let sequence = game_state.rng.next_shop_generation_sequence();
    let free = game_state.stage_modifiers.is_free_shop_this_stage();
    let discount = game_state.upgrade_state.shop_item_price_minus();
    let mut screen_seen = existing_slots
        .iter()
        .map(|slot| content_key_for_slot(&slot.slot))
        .collect::<Vec<_>>();
    let mut stats = ShopGenerationStats::default();
    let mut slots = Vec::with_capacity(slot_count);

    for slot_index in 0..slot_count {
        let drawn_category = bag::draw_category(&mut game_state.rng);
        let category = forced_category.unwrap_or(drawn_category);
        let eligible = eligible_rarities(game_state, category);
        let rarity = bag::draw_rarity(&mut game_state.rng, category, &eligible)
            .or_else(|| {
                eligible_rarities(game_state, drawn_category)
                    .into_iter()
                    .next()
            })
            .expect("shop must have an eligible rarity");
        let (candidate, rarity_fallback, category_fallback, duplicate) =
            choose_candidate(game_state, category, rarity, &screen_seen);
        if rarity_fallback {
            stats.rarity_fallback_count += 1;
        } else if category_fallback {
            stats.category_fallback_count += 1;
        } else if duplicate {
            stats.unavoidable_duplicate_count += 1;
        } else {
            stats.exact_match_count += 1;
        }
        let mut slot = generate_candidate(
            &candidate,
            &game_state.upgrade_state,
            &game_state.rng,
            sequence,
            slot_index,
        );
        apply_deterministic_cost(
            &mut slot,
            &game_state.rng,
            sequence,
            slot_index,
            free,
            discount,
        );
        screen_seen.push(candidate.key);
        slots.push(ShopSlotData::new(slot));
    }

    GeneratedShopScreen { slots, stats }
}

#[allow(dead_code)]
pub(crate) fn generate_shop_screen(
    game_state: &mut GameState,
    slot_count: usize,
) -> Vec<ShopSlotData> {
    generate_shop_screen_with_stats(game_state, slot_count, &[], None).slots
}
