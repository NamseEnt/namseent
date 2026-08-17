use crate::deterministic_rng::{self, domain};
use crate::game_state::rng::GameRngState;
use crate::rarity::Rarity;

pub(crate) fn rarity_index(rarity: Rarity) -> usize {
    match rarity {
        Rarity::Common => 0,
        Rarity::Rare => 1,
        Rarity::Epic => 2,
        Rarity::Legendary => 3,
    }
}

pub(crate) fn category_index(category: usize) -> usize {
    category
}

pub(crate) fn content_bag_index(category: usize, rarity: Rarity) -> usize {
    category * 4 + rarity_index(rarity)
}

pub(crate) fn rarity_domain(category: usize) -> u64 {
    match category {
        0 => domain::SHOP_RARITY_ITEM,
        1 => domain::SHOP_RARITY_CARD_SERVICE,
        2 => domain::SHOP_RARITY_UPGRADE,
        _ => unreachable!(),
    }
}

pub(crate) fn content_domain(category: usize) -> u64 {
    match category {
        0 => domain::SHOP_CONTENT_ITEM,
        1 => domain::SHOP_CONTENT_CARD_SERVICE,
        2 => domain::SHOP_CONTENT_UPGRADE,
        _ => unreachable!(),
    }
}

pub(crate) fn weighted_quota(weights: &[u32], bag_size: usize) -> Vec<usize> {
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

pub(crate) fn refill_category_bag(state: &mut GameRngState) {
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

pub(crate) fn draw_category(state: &mut GameRngState) -> usize {
    if state.shop.category_bag.cursor >= state.shop.category_bag.entries.len() {
        refill_category_bag(state);
    }
    let category = state.shop.category_bag.entries[state.shop.category_bag.cursor] as usize;
    state.shop.category_bag.cursor += 1;
    category
}

pub(crate) fn refill_rarity_bag(
    state: &mut GameRngState,
    category: usize,
    eligible_rarities: &[Rarity],
) {
    let weights = match category {
        0 => state.shop.config.item_rarity_weights.clone(),
        1 => state.shop.config.card_service_rarity_weights.clone(),
        2 => state.shop.config.upgrade_rarity_weights.clone(),
        _ => unreachable!(),
    };
    let mut effective_weights = vec![0; 4];
    for rarity in eligible_rarities {
        effective_weights[rarity_index(*rarity)] = weights[rarity_index(*rarity)];
    }
    if effective_weights.iter().all(|weight| *weight == 0) {
        for rarity in eligible_rarities {
            effective_weights[rarity_index(*rarity)] = 1;
        }
    }
    let quotas = weighted_quota(&effective_weights, state.shop.config.rarity_bag_size);
    let bag_index = category_index(category);
    let cycle = state.shop.rarity_bags[bag_index].cycle;
    let mut rng = state.rng_for(rarity_domain(category), &[cycle]);
    let bag = &mut state.shop.rarity_bags[bag_index];
    bag.entries.clear();
    for (rarity, quota) in quotas.into_iter().enumerate() {
        bag.entries.extend(std::iter::repeat_n(rarity as u8, quota));
    }
    deterministic_rng::shuffle(&mut bag.entries, &mut rng);
    bag.cursor = 0;
    bag.cycle = cycle.wrapping_add(1);
}

pub(crate) fn draw_rarity(
    state: &mut GameRngState,
    category: usize,
    eligible_rarities: &[Rarity],
) -> Option<Rarity> {
    if eligible_rarities.is_empty() {
        return None;
    }
    if state.shop.config.rarity_bag_size == 0 {
        return Some(eligible_rarities[0]);
    }

    loop {
        let needs_refill = {
            let bag = &state.shop.rarity_bags[category_index(category)];
            bag.cursor >= bag.entries.len()
        };
        if needs_refill {
            refill_rarity_bag(state, category, eligible_rarities);
        }
        let bag = &mut state.shop.rarity_bags[category_index(category)];
        let rarity = bag.entries[bag.cursor] as usize;
        bag.cursor += 1;
        let rarity = match rarity {
            0 => Rarity::Common,
            1 => Rarity::Rare,
            2 => Rarity::Epic,
            3 => Rarity::Legendary,
            _ => continue,
        };
        if eligible_rarities.contains(&rarity) {
            return Some(rarity);
        }
    }
}

pub(crate) fn refill_content_bag(
    state: &mut GameRngState,
    category: usize,
    rarity: Rarity,
    eligible_keys: &[String],
) {
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

pub(crate) fn draw_content_key(
    state: &mut GameRngState,
    category: usize,
    rarity: Rarity,
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
