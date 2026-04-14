//! Card reroll and tower selection strategies.

use super::CardRerollStrategy;
use crate::card::Card;
use crate::config::GameConfig;
use crate::flow_ui::selecting_tower::tower_selecting_hand::get_highest_tower::get_highest_tower_template;
use crate::game_state::GameState;
use crate::game_state::item::ItemKind;
use crate::game_state::tower::TowerKind;
use crate::game_state::upgrade::UpgradeState;
use crate::hand::HandItem;
use rand::RngCore;

/// Holds strong partial hands and simulates reroll outcomes before selecting cards to reroll.
///
/// **행동 수칙 (Behavioral Principles):**
/// - 포커 족보가 **Flush (플러시) 이상**이면 리롤을 중단합니다.
/// - 현재 손패의 전체 혹은 일부를 조합(결합)하는 경우의 수(Masking)를 계산합니다.
/// - 각 경우의 수에 대해 덱에서 남은 슬롯만큼 무작위(Monte Carlo)로 카드를 뽑았을 때의 얻을 수 있는 타워의 "기대 점수(Expected Value)"를 도출합니다.
/// - 최고 기대 점수를 주는 남길 카드의 인덱스(Hold cards)를 구한 뒤, 나머지만 폐기하고 다시 가져옵니다.
/// - 이 행위를 주사위가 소진되거나 플러시 이상이 될 때까지 반복합니다.
pub struct SmartRerollStrategy;

impl CardRerollStrategy for SmartRerollStrategy {
    fn name(&self) -> &str {
        "smart_reroll"
    }

    fn execute_card_selection(&self, game_state: &mut GameState, rng: &mut dyn RngCore) {
        while game_state.left_dice > 0 {
            let cards = collect_hand_cards(game_state);
            if cards.is_empty() {
                break;
            }

            let current_template = get_highest_tower_template(
                &cards,
                &game_state.upgrade_state,
                game_state.rerolled_count,
                &game_state.config,
            );
            let current_rating = tower_kind_rating(current_template.kind);

            if current_rating >= tower_kind_rating(TowerKind::Flush) {
                break;
            }

            let hold_indices = choose_best_hold_indices(
                &cards,
                &game_state.deck,
                &game_state.upgrade_state,
                game_state.rerolled_count,
                &game_state.config,
                rng,
            );

            if hold_indices.len() == cards.len() {
                break;
            }

            reroll_selected_cards(game_state, &hold_indices);
        }

        let cards = collect_hand_cards(game_state);
        if cards.is_empty() {
            return;
        }

        let tower_template = get_highest_tower_template(
            &cards,
            &game_state.upgrade_state,
            game_state.rerolled_count,
            &game_state.config,
        );

        game_state.goto_placing_tower(tower_template);
    }
}

/// Uses card-granting items when they complete or improve a strong combo, then falls back to smart rerolling.
///
/// **행동 수칙 (Behavioral Principles):**
/// - 주사위를 굴리기 전, 손패에 카드를 제공하는 아이템(`ItemKind::GrantCard`)이 있다면 아이템 사용 가능성을 평가합니다.
/// - 현재 손패가 Straight 미만이면서, 해당 아이템을 썼을 때 **Straight (스트레이트) 이상의 족보**가 완성되거나 향상되면 주사위 소모 없이 즉시 사용합니다.
/// - 아이템을 사용해 조건을 충족했다면, 다시 타워를 배치하는 흐름으로 전환(종료)합니다.
/// - 위 조건에 맞는 아이템이 없거나 주사위가 남아있다면 `SmartRerollStrategy`로 전환(위임)하여 스마트 리롤(일부 남기기)을 굴립니다.
pub struct ItemAwareRerollStrategy;

impl CardRerollStrategy for ItemAwareRerollStrategy {
    fn name(&self) -> &str {
        "item_aware_reroll"
    }

    fn execute_card_selection(&self, game_state: &mut GameState, rng: &mut dyn RngCore) {
        let cards = collect_hand_cards(game_state);
        if !cards.is_empty()
            && game_state.left_dice > 0
            && try_use_grant_card_item(game_state, &cards)
        {
            let cards = collect_hand_cards(game_state);
            if cards.is_empty() {
                return;
            }
        }

        let strategy = SmartRerollStrategy;
        strategy.execute_card_selection(game_state, rng);
    }
}

fn collect_hand_cards(game_state: &GameState) -> Vec<Card> {
    let slot_ids = game_state.hand.active_slot_ids();
    slot_ids
        .iter()
        .filter_map(|id| {
            game_state
                .hand
                .get_item(*id)
                .and_then(|item| item.as_card().copied())
        })
        .collect()
}

const EXPECTED_REROLL_SAMPLES: usize = 12;

fn try_use_grant_card_item(game_state: &mut GameState, cards: &[Card]) -> bool {
    let current_template = get_highest_tower_template(
        cards,
        &game_state.upgrade_state,
        game_state.rerolled_count,
        &game_state.config,
    );
    let current_rating = tower_kind_rating(current_template.kind);

    if current_rating >= tower_kind_rating(TowerKind::Straight) {
        return false;
    }

    let mut best_item_idx = None;
    let mut best_item_rating = current_rating;

    let straight_threshold = tower_kind_rating(TowerKind::Straight);

    for (idx, item) in game_state.items.iter().enumerate() {
        if let ItemKind::GrantCard { card } = &item.kind {
            let mut candidate_cards = cards.to_vec();
            candidate_cards.push(*card);

            let candidate_template = get_highest_tower_template(
                &candidate_cards,
                &game_state.upgrade_state,
                game_state.rerolled_count,
                &game_state.config,
            );

            let candidate_rating = tower_kind_rating(candidate_template.kind);
            if candidate_rating >= straight_threshold && candidate_rating > best_item_rating {
                best_item_rating = candidate_rating;
                best_item_idx = Some(idx);
            }
        }
    }

    if let Some(idx) = best_item_idx {
        let item = game_state.items.remove(idx);
        game_state.use_item(&item);
        return true;
    }

    false
}

fn choose_best_hold_indices(
    cards: &[Card],
    deck: &crate::card::Deck,
    upgrade_state: &UpgradeState,
    rerolled_count: usize,
    config: &std::sync::Arc<GameConfig>,
    rng: &mut dyn RngCore,
) -> Vec<usize> {
    let card_count = cards.len();
    let mut best_score = f32::MIN;
    let mut best_hold = Vec::new();

    for mask in 0..(1usize << card_count) {
        let hold_cards: Vec<Card> = cards
            .iter()
            .enumerate()
            .filter_map(|(index, &card)| {
                if mask & (1 << index) != 0 {
                    Some(card)
                } else {
                    None
                }
            })
            .collect();

        let score = estimate_expected_rating(
            &hold_cards,
            card_count.saturating_sub(hold_cards.len()),
            deck,
            upgrade_state,
            rerolled_count,
            config,
            rng,
        );

        if score > best_score || (score == best_score && hold_cards.len() > best_hold.len()) {
            best_score = score;
            best_hold = (0..card_count)
                .filter(|index| mask & (1 << index) != 0)
                .collect();
        }
    }

    best_hold
}

fn estimate_expected_rating(
    hold_cards: &[Card],
    draw_count: usize,
    deck: &crate::card::Deck,
    upgrade_state: &UpgradeState,
    rerolled_count: usize,
    config: &std::sync::Arc<GameConfig>,
    rng: &mut dyn RngCore,
) -> f32 {
    if draw_count == 0 {
        let template =
            get_highest_tower_template(hold_cards, upgrade_state, rerolled_count, config);
        return tower_kind_rating(template.kind) as f32;
    }

    let mut total = 0.0;
    for _ in 0..EXPECTED_REROLL_SAMPLES {
        let mut candidate_cards = hold_cards.to_vec();
        candidate_cards.extend(deck.sample(draw_count, rng));

        let template =
            get_highest_tower_template(&candidate_cards, upgrade_state, rerolled_count, config);
        total += tower_kind_rating(template.kind) as f32;
    }

    total / EXPECTED_REROLL_SAMPLES as f32
}

fn reroll_selected_cards(game_state: &mut GameState, hold_indices: &[usize]) {
    let active_ids = game_state.hand.active_slot_ids();

    let reroll_ids: Vec<_> = active_ids
        .iter()
        .enumerate()
        .filter_map(|(index, id)| {
            if !hold_indices.contains(&index) {
                Some(*id)
            } else {
                None
            }
        })
        .collect();

    if reroll_ids.is_empty() {
        return;
    }

    let old_cards: Vec<Card> = reroll_ids
        .iter()
        .filter_map(|id| {
            game_state
                .hand
                .get_item(*id)
                .and_then(|item| item.as_card().copied())
        })
        .collect();

    game_state.hand.delete_slots(&reroll_ids);
    game_state.deck.put_back(old_cards);
    game_state.left_dice = game_state.left_dice.saturating_sub(1);
    game_state.rerolled_count += 1;

    for _ in 0..reroll_ids.len() {
        let card = game_state.deck.draw().unwrap_or_else(Card::new_random);
        game_state.hand.push(HandItem::Card(card));
    }
}

fn tower_kind_rating(kind: TowerKind) -> u32 {
    match kind {
        TowerKind::Barricade => 0,
        TowerKind::High => 1,
        TowerKind::OnePair => 2,
        TowerKind::TwoPair => 3,
        TowerKind::ThreeOfAKind => 4,
        TowerKind::Straight => 5,
        TowerKind::Flush => 6,
        TowerKind::FullHouse => 7,
        TowerKind::FourOfAKind => 8,
        TowerKind::StraightFlush => 9,
        TowerKind::RoyalFlush => 10,
    }
}
