use crate::deterministic_rng::{self, domain};
use crate::game_state::rng::RngState;
use crate::{
    GameFlowState, ItemEntryState, ShopSlotDataState, ShopSlotState, ShopState, UpgradeEntryState,
};

pub const ITEM_CATEGORY: usize = 0;
pub const CARD_SERVICE_CATEGORY: usize = 1;
pub const UPGRADE_CATEGORY: usize = 2;

const COMMON: u8 = 0;
const RARE: u8 = 1;
const EPIC: u8 = 2;
const LEGENDARY: u8 = 3;
const RARITY_COUNT: usize = 4;
const BASE_COST_ITEM: u64 = 20;
const BASE_COST_UPGRADE: u64 = 95;
const BASE_COST_CARD_SERVICE: u64 = 45;
const BASIS_POINTS: u64 = 10_000;
const RARE_PRICE_FACTOR_BASIS_POINTS: u64 = 14_000;
const EPIC_PRICE_FACTOR_BASIS_POINTS: u64 = 18_000;
const LEGENDARY_PRICE_FACTOR_BASIS_POINTS: u64 = 22_000;
const MAX_ADDITIONAL_COST_BASIS_POINTS: u64 = 5_000;
const MAX_SHOP_SLOT_EXPAND: usize = 2;
const MAX_SHOP_ITEM_PRICE_MINUS: usize = 15;

#[derive(Clone, Debug)]
struct Candidate {
    key: String,
    rarity: u8,
    kind: u8,
}

fn stable_key(category: usize, key: &str) -> String {
    let prefix = match category {
        ITEM_CATEGORY => "item",
        CARD_SERVICE_CATEGORY => "card_service",
        UPGRADE_CATEGORY => "upgrade",
        _ => unreachable!("invalid shop category"),
    };
    format!("{prefix}:{key}")
}

fn item_key(kind: crate::ItemKind) -> &'static str {
    kind.key()
}

fn legacy_card_service_key(kind: crate::CardServiceKind) -> &'static str {
    match kind {
        crate::CardServiceKind::LongSword => "LongSword",
        crate::CardServiceKind::Staff => "Staff",
        crate::CardServiceKind::Mace => "Mace",
        crate::CardServiceKind::ClubSword => "ClubSword",
        crate::CardServiceKind::Brush => "Brush",
        crate::CardServiceKind::FountainPen => "FountainPen",
        crate::CardServiceKind::Tricycle => "Tricycle",
        crate::CardServiceKind::Eraser => "Eraser",
        crate::CardServiceKind::MagicWand => "MagicWand",
        crate::CardServiceKind::Pliers => "Pliers",
        crate::CardServiceKind::Screwdriver => "Screwdriver",
        crate::CardServiceKind::Copier => "Copier",
        crate::CardServiceKind::Magnet => "Magnet",
        crate::CardServiceKind::Cactus => "Battery",
        crate::CardServiceKind::SpinningTop => "Cactus",
        crate::CardServiceKind::Battery => "SpinningTop",
    }
}

fn upgrade_key(kind: crate::UpgradeKind) -> &'static str {
    kind.key()
}

fn item_rarity(kind: crate::ItemKind) -> u8 {
    crate::item_rarity(kind)
        .map(|rarity| rarity.raw())
        .expect("invalid item kind")
}

fn card_service_rarity(kind: crate::CardServiceKind) -> u8 {
    match kind {
        crate::CardServiceKind::Brush
        | crate::CardServiceKind::FountainPen
        | crate::CardServiceKind::Tricycle => COMMON,
        crate::CardServiceKind::LongSword
        | crate::CardServiceKind::Staff
        | crate::CardServiceKind::Mace
        | crate::CardServiceKind::ClubSword
        | crate::CardServiceKind::Eraser
        | crate::CardServiceKind::Pliers
        | crate::CardServiceKind::Screwdriver => RARE,
        crate::CardServiceKind::MagicWand
        | crate::CardServiceKind::Copier
        | crate::CardServiceKind::Magnet
        | crate::CardServiceKind::Cactus
        | crate::CardServiceKind::SpinningTop
        | crate::CardServiceKind::Battery => EPIC,
    }
}

fn scalar(upgrade: &UpgradeEntryState, index: usize) -> usize {
    upgrade
        .scalar_values
        .get(index)
        .copied()
        .and_then(|value| usize::try_from(value).ok())
        .unwrap_or(0)
}

fn shop_slot_expand(core: &crate::CoreState) -> usize {
    core.upgrades
        .upgrades
        .iter()
        .filter(|upgrade| {
            upgrade
                .upgrade_kind()
                .is_ok_and(|kind| kind == crate::UpgradeKind::Backpack)
        })
        .map(|upgrade| scalar(upgrade, 0))
        .sum()
}

pub fn max_shop_slot_count(core: &crate::CoreState) -> usize {
    shop_slot_expand(core).saturating_add(2)
}

fn shop_item_price_minus(core: &crate::CoreState) -> usize {
    core.upgrades
        .upgrades
        .iter()
        .filter(|upgrade| {
            upgrade
                .upgrade_kind()
                .is_ok_and(|kind| kind == crate::UpgradeKind::EnergyDrink)
        })
        .map(|upgrade| scalar(upgrade, 0))
        .sum()
}

fn has_upgrade(core: &crate::CoreState, kind: crate::UpgradeKind) -> bool {
    core.upgrades.upgrades.iter().any(|upgrade| {
        upgrade
            .upgrade_kind()
            .is_ok_and(|upgrade_kind| upgrade_kind == kind)
    })
}

fn upgrade_is_eligible(core: &crate::CoreState, kind: crate::UpgradeKind) -> bool {
    match kind {
        crate::UpgradeKind::Backpack => shop_slot_expand(core) < MAX_SHOP_SLOT_EXPAND,
        crate::UpgradeKind::EnergyDrink => shop_item_price_minus(core) < MAX_SHOP_ITEM_PRICE_MINUS,
        crate::UpgradeKind::FourLeafClover
        | crate::UpgradeKind::Rabbit
        | crate::UpgradeKind::BlackWhite => !has_upgrade(core, kind),
        _ => true,
    }
}

fn all_candidates(core: &crate::CoreState, category: usize, rarity: Option<u8>) -> Vec<Candidate> {
    let kinds: Vec<u8> = match category {
        ITEM_CATEGORY => crate::ItemKind::ALL
            .iter()
            .copied()
            .map(crate::ItemKind::raw)
            .collect(),
        CARD_SERVICE_CATEGORY => crate::CardServiceKind::ALL
            .iter()
            .copied()
            .map(crate::CardServiceKind::raw)
            .collect(),
        UPGRADE_CATEGORY => crate::UpgradeKind::ALL
            .iter()
            .copied()
            .filter(|kind| upgrade_is_eligible(core, *kind))
            .map(crate::UpgradeKind::raw)
            .collect(),
        _ => unreachable!("invalid shop category"),
    };
    let mut candidates = kinds
        .into_iter()
        .map(|kind| {
            let key = match category {
                ITEM_CATEGORY => stable_key(
                    category,
                    item_key(crate::ItemKind::from_raw(kind).expect("invalid item kind")),
                ),
                CARD_SERVICE_CATEGORY => stable_key(
                    category,
                    legacy_card_service_key(
                        crate::CardServiceKind::from_raw(kind).expect("catalog kind is valid"),
                    ),
                ),
                UPGRADE_CATEGORY => stable_key(
                    category,
                    upgrade_key(crate::UpgradeKind::from_raw(kind).expect("catalog kind is valid")),
                ),
                _ => unreachable!("invalid shop category"),
            };
            let rarity = match category {
                ITEM_CATEGORY => {
                    item_rarity(crate::ItemKind::from_raw(kind).expect("invalid item kind"))
                }
                CARD_SERVICE_CATEGORY => card_service_rarity(
                    crate::CardServiceKind::from_raw(kind).expect("catalog kind is valid"),
                ),
                UPGRADE_CATEGORY => crate::game_state::upgrade::upgrade_rarity(
                    crate::UpgradeKind::from_raw(kind).expect("catalog kind is valid"),
                )
                .raw(),
                _ => unreachable!("invalid shop category"),
            };
            Candidate { key, rarity, kind }
        })
        .collect::<Vec<_>>();
    candidates.sort_by(|left, right| left.key.cmp(&right.key));
    if let Some(rarity) = rarity {
        candidates.retain(|candidate| candidate.rarity == rarity);
    }
    candidates
}

fn candidate_keys(candidates: &[Candidate]) -> Vec<String> {
    candidates
        .iter()
        .map(|candidate| candidate.key.clone())
        .collect()
}

fn eligible_rarities(core: &crate::CoreState, category: usize) -> Vec<u8> {
    [COMMON, RARE, EPIC, LEGENDARY]
        .into_iter()
        .filter(|rarity| !all_candidates(core, category, Some(*rarity)).is_empty())
        .collect()
}

fn candidate_for_key<'a>(candidates: &'a [Candidate], key: &str) -> Option<&'a Candidate> {
    candidates.iter().find(|candidate| candidate.key == key)
}

fn draw_content(
    core: &mut crate::CoreState,
    category: usize,
    rarity: u8,
    screen_seen: &[String],
) -> (Option<Candidate>, bool) {
    let candidates = all_candidates(core, category, Some(rarity));
    let keys = candidate_keys(&candidates);
    let key = draw_content_key(&mut core.rng, category, rarity, &keys, screen_seen);
    (
        key.as_deref()
            .and_then(|key| candidate_for_key(&candidates, key))
            .cloned(),
        key.is_some(),
    )
}

fn choose_candidate(
    core: &mut crate::CoreState,
    target_category: usize,
    target_rarity: u8,
    screen_seen: &[String],
) -> (Candidate, bool, bool, bool) {
    if let (Some(candidate), true) = draw_content(core, target_category, target_rarity, screen_seen)
        && !screen_seen.contains(&candidate.key)
    {
        return (candidate, false, false, false);
    }

    for rarity in [COMMON, RARE, EPIC, LEGENDARY] {
        if rarity == target_rarity {
            continue;
        }
        if let (Some(candidate), true) = draw_content(core, target_category, rarity, screen_seen)
            && !screen_seen.contains(&candidate.key)
        {
            return (candidate, true, false, false);
        }
    }

    for category in [ITEM_CATEGORY, CARD_SERVICE_CATEGORY, UPGRADE_CATEGORY] {
        if category == target_category {
            continue;
        }
        for rarity in [COMMON, RARE, EPIC, LEGENDARY] {
            if let (Some(candidate), true) = draw_content(core, category, rarity, screen_seen)
                && !screen_seen.contains(&candidate.key)
            {
                return (candidate, false, true, false);
            }
        }
    }

    let mut all = Vec::new();
    for category in [ITEM_CATEGORY, CARD_SERVICE_CATEGORY, UPGRADE_CATEGORY] {
        all.extend(all_candidates(core, category, None));
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

fn generated_item(kind: crate::ItemKind) -> ItemEntryState {
    crate::generated_item(kind).expect("invalid item kind")
}

fn content_key_for_slot(slot: &ShopSlotState) -> String {
    match slot {
        ShopSlotState::Item { item, .. } => stable_key(
            ITEM_CATEGORY,
            item_key(crate::ItemKind::from_raw(item.kind).expect("invalid item kind")),
        ),
        ShopSlotState::CardService { kind, .. } => stable_key(
            CARD_SERVICE_CATEGORY,
            legacy_card_service_key(
                crate::CardServiceKind::from_raw(*kind).expect("invalid card service kind"),
            ),
        ),
        ShopSlotState::Upgrade { upgrade, .. } => stable_key(
            UPGRADE_CATEGORY,
            upgrade_key(
                upgrade
                    .upgrade_kind()
                    .expect("stored upgrade kind is valid"),
            ),
        ),
    }
}

fn generated_slot(
    candidate: &Candidate,
    sequence: u64,
    slot_index: usize,
    core: &crate::CoreState,
) -> ShopSlotState {
    match candidate.kind {
        kind if candidate.key.starts_with("item:") => {
            let mut rng = core.rng.rng_for(
                domain::SHOP_ITEM_PAYLOAD,
                &[
                    sequence,
                    slot_index as u64,
                    deterministic_rng::stable_key_hash(&candidate.key),
                ],
            );
            let _ = &mut rng;
            ShopSlotState::Item {
                item: generated_item(crate::ItemKind::from_raw(kind).expect("invalid item kind")),
                cost: 0,
            }
        }
        kind if candidate.key.starts_with("card_service:") => {
            ShopSlotState::CardService { kind, cost: 0 }
        }
        kind if candidate.key.starts_with("upgrade:") => ShopSlotState::Upgrade {
            upgrade: crate::game_state::upgrade::generated_upgrade(
                crate::UpgradeKind::from_raw(kind).expect("catalog kind is valid"),
            ),
            cost: 0,
        },
        _ => unreachable!("invalid shop candidate"),
    }
}

fn rarity_price_factor_basis_points(rarity: u8) -> u64 {
    match rarity {
        COMMON => BASIS_POINTS,
        RARE => RARE_PRICE_FACTOR_BASIS_POINTS,
        EPIC => EPIC_PRICE_FACTOR_BASIS_POINTS,
        LEGENDARY => LEGENDARY_PRICE_FACTOR_BASIS_POINTS,
        _ => unreachable!("invalid shop rarity"),
    }
}

fn apply_deterministic_cost(
    slot: &mut ShopSlotState,
    state: &RngState,
    sequence: u64,
    slot_index: usize,
    free: bool,
    discount: usize,
) {
    if free {
        set_cost(slot, 0);
        return;
    }
    let base_cost = match slot {
        ShopSlotState::Item { .. } => BASE_COST_ITEM,
        ShopSlotState::Upgrade { .. } => BASE_COST_UPGRADE,
        ShopSlotState::CardService { .. } => BASE_COST_CARD_SERVICE,
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
    let total_basis_points =
        BASIS_POINTS + additional_basis_points.min(MAX_ADDITIONAL_COST_BASIS_POINTS);
    let numerator = u128::from(base_cost)
        * u128::from(rarity_price_factor_basis_points(slot_rarity(slot)))
        * u128::from(total_basis_points);
    let cost = numerator
        .checked_div(u128::from(BASIS_POINTS) * u128::from(BASIS_POINTS))
        .unwrap_or(u128::MAX)
        .min(usize::MAX as u128)
        .saturating_sub(discount as u128) as usize;
    set_cost(slot, cost);
}

fn slot_rarity(slot: &ShopSlotState) -> u8 {
    match slot {
        ShopSlotState::Item { item, .. } => {
            item_rarity(crate::ItemKind::from_raw(item.kind).expect("invalid item kind"))
        }
        ShopSlotState::CardService { kind, .. } => card_service_rarity(
            crate::CardServiceKind::from_raw(*kind).expect("invalid card service kind"),
        ),
        ShopSlotState::Upgrade { upgrade, .. } => crate::game_state::upgrade::upgrade_rarity(
            upgrade
                .upgrade_kind()
                .expect("stored upgrade kind is valid"),
        )
        .raw(),
    }
}

fn set_cost(slot: &mut ShopSlotState, cost: usize) {
    match slot {
        ShopSlotState::Item {
            cost: slot_cost, ..
        }
        | ShopSlotState::Upgrade {
            cost: slot_cost, ..
        }
        | ShopSlotState::CardService {
            cost: slot_cost, ..
        } => *slot_cost = cost,
    }
}

fn weighted_quota(weights: &[u32], bag_size: usize) -> Vec<usize> {
    if bag_size == 0 || weights.is_empty() {
        return vec![0; weights.len()];
    }
    let total_weight: u64 = weights.iter().map(|weight| u64::from(*weight)).sum();
    if total_weight == 0 {
        return vec![0; weights.len()];
    }
    let mut quotas = vec![0; weights.len()];
    let mut remainders = Vec::with_capacity(weights.len());
    let mut assigned = 0;
    for (index, weight) in weights.iter().enumerate() {
        let numerator = u64::from(*weight) * bag_size as u64;
        quotas[index] = (numerator / total_weight) as usize;
        assigned += quotas[index];
        remainders.push((numerator % total_weight, index));
    }
    remainders.sort_by(|left, right| right.0.cmp(&left.0).then_with(|| left.1.cmp(&right.1)));
    for (_, index) in remainders.into_iter().take(bag_size - assigned) {
        quotas[index] += 1;
    }
    quotas
}

fn rarity_domain(category: usize) -> u64 {
    match category {
        ITEM_CATEGORY => domain::SHOP_RARITY_ITEM,
        CARD_SERVICE_CATEGORY => domain::SHOP_RARITY_CARD_SERVICE,
        UPGRADE_CATEGORY => domain::SHOP_RARITY_UPGRADE,
        _ => unreachable!("invalid shop category"),
    }
}

fn content_domain(category: usize) -> u64 {
    match category {
        ITEM_CATEGORY => domain::SHOP_CONTENT_ITEM,
        CARD_SERVICE_CATEGORY => domain::SHOP_CONTENT_CARD_SERVICE,
        UPGRADE_CATEGORY => domain::SHOP_CONTENT_UPGRADE,
        _ => unreachable!("invalid shop category"),
    }
}

fn refill_category_bag(state: &mut RngState) {
    if state.shop.config.category_bag_size == 0 {
        state.shop.category_bag.entries = vec![0];
        state.shop.category_bag.cursor = 0;
        return;
    }
    let mut weights = state.shop.config.category_weights.clone();
    if weights.iter().all(|weight| *weight == 0) {
        weights = vec![1, 1, 1];
    }
    let quotas = weighted_quota(&weights, state.shop.config.category_bag_size);
    state.shop.category_bag.entries.clear();
    for (category, quota) in quotas.into_iter().enumerate() {
        state
            .shop
            .category_bag
            .entries
            .extend(std::iter::repeat_n(category as u8, quota));
    }
    let cycle = state.shop.category_bag.cycle;
    let mut rng = state.rng_for(domain::SHOP_CATEGORY_BAG, &[cycle]);
    deterministic_rng::shuffle(&mut state.shop.category_bag.entries, &mut rng);
    state.shop.category_bag.cursor = 0;
    state.shop.category_bag.cycle = cycle.wrapping_add(1);
}

fn draw_category(state: &mut RngState) -> usize {
    if state.shop.category_bag.cursor >= state.shop.category_bag.entries.len() {
        refill_category_bag(state);
    }
    let category = state.shop.category_bag.entries[state.shop.category_bag.cursor] as usize;
    state.shop.category_bag.cursor += 1;
    category
}

fn rarity_index(rarity: u8) -> usize {
    rarity as usize
}

fn content_bag_index(category: usize, rarity: u8) -> usize {
    category * RARITY_COUNT + rarity_index(rarity)
}

fn refill_rarity_bag(state: &mut RngState, category: usize, eligible_rarities: &[u8]) {
    let weights = match category {
        ITEM_CATEGORY => state.shop.config.item_rarity_weights.clone(),
        CARD_SERVICE_CATEGORY => state.shop.config.card_service_rarity_weights.clone(),
        UPGRADE_CATEGORY => state.shop.config.upgrade_rarity_weights.clone(),
        _ => unreachable!("invalid shop category"),
    };
    let mut effective_weights = vec![0; RARITY_COUNT];
    for rarity in eligible_rarities {
        effective_weights[rarity_index(*rarity)] = weights[rarity_index(*rarity)];
    }
    if effective_weights.iter().all(|weight| *weight == 0) {
        for rarity in eligible_rarities {
            effective_weights[rarity_index(*rarity)] = 1;
        }
    }
    let quotas = weighted_quota(&effective_weights, state.shop.config.rarity_bag_size);
    let cycle = state.shop.rarity_bags[category].cycle;
    let mut rng = state.rng_for(rarity_domain(category), &[cycle]);
    let bag = &mut state.shop.rarity_bags[category];
    bag.entries.clear();
    for (rarity, quota) in quotas.into_iter().enumerate() {
        bag.entries.extend(std::iter::repeat_n(rarity as u8, quota));
    }
    deterministic_rng::shuffle(&mut bag.entries, &mut rng);
    bag.cursor = 0;
    bag.cycle = cycle.wrapping_add(1);
}

fn draw_rarity(state: &mut RngState, category: usize, eligible_rarities: &[u8]) -> Option<u8> {
    if eligible_rarities.is_empty() {
        return None;
    }
    if state.shop.config.rarity_bag_size == 0 {
        return Some(eligible_rarities[0]);
    }
    loop {
        if state.shop.rarity_bags[category].cursor >= state.shop.rarity_bags[category].entries.len()
        {
            refill_rarity_bag(state, category, eligible_rarities);
        }
        let bag = &mut state.shop.rarity_bags[category];
        let rarity = bag.entries[bag.cursor];
        bag.cursor += 1;
        if eligible_rarities.contains(&rarity) {
            return Some(rarity);
        }
    }
}

fn refill_content_bag(state: &mut RngState, category: usize, rarity: u8, eligible_keys: &[String]) {
    let bag_index = content_bag_index(category, rarity);
    let cycle = state.shop.content_bags[bag_index].cycle;
    let mut rng = state.rng_for(
        content_domain(category),
        &[rarity_index(rarity) as u64, cycle],
    );
    let bag = &mut state.shop.content_bags[bag_index];
    bag.entries = eligible_keys.to_vec();
    bag.entries.sort();
    deterministic_rng::shuffle(&mut bag.entries, &mut rng);
    bag.cursor = 0;
    bag.cycle = cycle.wrapping_add(1);
}

fn draw_content_key(
    state: &mut RngState,
    category: usize,
    rarity: u8,
    eligible_keys: &[String],
    screen_seen: &[String],
) -> Option<String> {
    if eligible_keys.is_empty() {
        return None;
    }
    let bag_index = content_bag_index(category, rarity);
    if state.shop.content_bags[bag_index].cursor >= state.shop.content_bags[bag_index].entries.len()
    {
        refill_content_bag(state, category, rarity, eligible_keys);
    }
    let bag = &mut state.shop.content_bags[bag_index];
    let mut candidate_index = bag.cursor;
    while candidate_index < bag.entries.len() {
        let key = &bag.entries[candidate_index];
        if eligible_keys.contains(key) && !screen_seen.contains(key) {
            bag.entries.swap(bag.cursor, candidate_index);
            let key = bag.entries[bag.cursor].clone();
            bag.cursor += 1;
            return Some(key);
        }
        candidate_index += 1;
    }
    eligible_keys
        .iter()
        .find(|key| !screen_seen.contains(key))
        .cloned()
}

fn generate_shop_screen(
    core: &mut crate::CoreState,
    slot_count: usize,
    existing_slots: &[ShopSlotDataState],
    forced_category: Option<usize>,
    free: bool,
) -> Vec<ShopSlotDataState> {
    let sequence = core.rng.next_shop_generation_sequence();
    let free = free || core.stage_modifiers.free_shop_this_stage;
    let discount = shop_item_price_minus(core);
    let mut screen_seen = existing_slots
        .iter()
        .map(|slot| content_key_for_slot(&slot.slot))
        .collect::<Vec<_>>();
    let mut next_slot_id = existing_slots
        .iter()
        .map(|slot| slot.id)
        .max()
        .map_or(0, |id| id.saturating_add(1));
    let mut slots = Vec::with_capacity(slot_count);
    for slot_index in 0..slot_count {
        let drawn_category = draw_category(&mut core.rng);
        let category = forced_category.unwrap_or(drawn_category);
        let eligible = eligible_rarities(core, category);
        let rarity = draw_rarity(&mut core.rng, category, &eligible)
            .or_else(|| eligible_rarities(core, drawn_category).first().copied())
            .or_else(|| eligible.first().copied())
            .expect("shop must have an eligible rarity");
        let (candidate, _, _, _) = choose_candidate(core, category, rarity, &screen_seen);
        let mut slot = generated_slot(&candidate, sequence, slot_index, core);
        apply_deterministic_cost(&mut slot, &core.rng, sequence, slot_index, free, discount);
        screen_seen.push(candidate.key);
        slots.push(ShopSlotDataState {
            id: next_slot_id,
            slot,
            purchased: false,
        });
        next_slot_id = next_slot_id.saturating_add(1);
    }
    slots
}

pub(crate) fn start_shopping_flow(core: &mut crate::CoreState) {
    let slot_count = shop_slot_expand(core).saturating_add(2);
    let mut slots = generate_shop_screen(core, slot_count, &[], None, false);
    let free_card_services = core.stage_modifiers.free_card_services;
    core.stage_modifiers.free_card_services = 0;
    for _ in 0..free_card_services {
        let mut free_slots =
            generate_shop_screen(core, 1, &slots, Some(CARD_SERVICE_CATEGORY), true);
        slots.append(&mut free_slots);
    }
    core.flow = GameFlowState::Shopping(ShopState { slots });
}

pub(crate) fn add_shop_slots(core: &mut crate::CoreState, count: usize) {
    if count == 0 {
        return;
    }
    let existing_slots = match &core.flow {
        GameFlowState::Shopping(shop) => shop.slots.clone(),
        _ => return,
    };
    let slots = generate_shop_screen(core, count, &existing_slots, None, false);
    if let GameFlowState::Shopping(shop) = &mut core.flow {
        shop.slots.extend(slots);
    }
}
