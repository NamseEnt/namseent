//! Canonical dense policy action index contract for ML.
//!
//! Bridges `policy_action_index <-> AgentAction` for a single decision
//! state, so a future PPO/BC network can address every legal semantic
//! action through one flat, deterministic index space instead of a
//! candidate-proposal list. This module does not change the network,
//! trajectory schema, or the legacy teacher/PPO candidate path - it is a
//! new, separately-verified contract (see `PolicyActionSpace`) that a later
//! step will wire the model into.
//!
//! # Layout
//!
//! Exactly one of two shapes, chosen by
//! `GameEnvironment::semantic_card_decision_available()` (the same gate
//! `environment::semantic_legal_actions_with_position_limit` and
//! `teacher::dense_semantic_candidates` already use):
//!
//! - **Card decision available** (`Shop`/`CardSelection`, no card
//!   selection in progress): `BuildTower` dense region (reusing
//!   `joint_action::DenseBuildTowerScoreTable`'s `subset_index x
//!   hand_slot_index x position_index` joint space verbatim - not
//!   reimplemented), then `Reroll` (one per card subset, in the same
//!   id-sorted `CardSubsetTable` order - never hand-slot order), then
//!   `PurchaseShopItem` (one per shop slot, only when the decision point is
//!   `Shop`), `UseInventoryItem` (one per inventory item), and
//!   `DiscardTreasure` (one per discardable treasure, sorted by
//!   `upgrade_id`). Every region's size reflects "how many indices this
//!   axis has right now" - not a fixed global upper bound - since card
//!   count, shop size, inventory size, and treasure count are all
//!   state-dependent in this game's contract; legality of a given index is
//!   a separate concern, carried by `legal_mask`.
//! - **Otherwise** (`Defense`, `PreDefenseItem`, `DamageResponseItem`,
//!   `Terminal`, `CardServiceSelection`, or mid card-selection): the small
//!   set `GameEnvironment::legal_actions()` already returns, canonically
//!   reordered by `(ActionKind, per-kind stable key)` - never by
//!   `action_id()` string sort, which is an opaque, format-fragile
//!   ordering, not a semantically meaningful one.
//!
//! Card identity throughout is stable card IDs (via `CardSubsetTable`,
//! itself sorted by ID), never a hand slot offset. Two actions that differ
//! only in `card_ids`' *order* are the same action for indexing purposes
//! (`canonicalize` normalizes this before any comparison).

use crate::environment::{ActionKind, AgentAction, DecisionPoint, GameEnvironment, LegalAction};
use crate::joint_action::DenseBuildTowerScoreTable;

/// `policy_action_index -> AgentAction` and back, plus a legality mask,
/// for one decision state. Construct fresh per decision via
/// [`PolicyActionSpace::compute`] - it borrows nothing and is cheap enough
/// to rebuild every step (dominated by `DenseBuildTowerScoreTable::compute`,
/// the same cost `teacher::dense_semantic_candidates` already pays).
pub struct PolicyActionSpace {
    layout: Layout,
    legal_mask: Vec<bool>,
}

enum Layout {
    CardDecision {
        build_table: DenseBuildTowerScoreTable,
        build_tower_len: usize,
        reroll_len: usize,
        shop_len: usize,
        inventory_len: usize,
        /// Sorted ascending - `DiscardTreasure`'s stable identity
        /// (`upgrade_id`), not an array position.
        treasure_upgrade_ids: Vec<u64>,
    },
    /// Canonically-ordered, already-legal actions for any decision point
    /// where a card decision isn't available. Small by construction (see
    /// module docs), so no dense sub-layout is needed.
    Other { actions: Vec<AgentAction> },
}

impl PolicyActionSpace {
    pub fn compute(environment: &GameEnvironment) -> Self {
        let observation = environment.snapshot();
        let layout = if environment.semantic_card_decision_available() {
            let build_table = DenseBuildTowerScoreTable::compute(environment, &observation);
            let build_tower_len = build_table.subsets.subset_count()
                * build_table.build_slot_count
                * build_table.position_count;
            let reroll_len = build_table.subsets.subset_count();
            let shop_len = if matches!(environment.decision_point(), DecisionPoint::Shop) {
                observation.shop.len()
            } else {
                0
            };
            let inventory_len = observation.inventory.len();
            let mut treasure_upgrade_ids = observation.discardable_treasure_ids.clone();
            treasure_upgrade_ids.sort_unstable();
            Layout::CardDecision {
                build_table,
                build_tower_len,
                reroll_len,
                shop_len,
                inventory_len,
                treasure_upgrade_ids,
            }
        } else {
            let mut actions = environment
                .legal_actions()
                .into_iter()
                .map(|legal: LegalAction| legal.action)
                .collect::<Vec<_>>();
            for action in &actions {
                assert!(
                    is_other_layout_policy_visible(action.kind()),
                    "PolicyActionSpace invariant violated: a UI/FSM micro-action {:?} \
                     (decision_point={:?}) reached the Other layout. PolicyActionSpace::compute \
                     must only be called at semantic decision boundaries reached via \
                     GameEnvironment::semantic_step, which never externally returns control mid \
                     card-selection (e.g. after a bare BeginTowerSelection/BeginRerollSelection) - \
                     see docs/game-ai/02-action-contract.md and \
                     policy_action_space_never_exposes_ui_micro_actions_along_semantic_rollouts",
                    action.kind(),
                    environment.decision_point(),
                );
            }
            actions.sort_by_key(canonical_sort_key);
            Layout::Other { actions }
        };
        let legal_mask = compute_legal_mask(environment, &layout);
        Self { layout, legal_mask }
    }

    pub fn action_count(&self) -> usize {
        self.legal_mask.len()
    }

    /// The legality mask, aligned 1:1 with `policy_action_index`. `true`
    /// means `index_to_action(index)` is legal in the state this space was
    /// computed from.
    ///
    /// `BuildTower` entries are read directly from
    /// `DenseBuildTowerScoreTable` (already computed, O(1) per index) -
    /// this never re-derives legality by enumerating `AgentAction`s.
    /// Non-`BuildTower` entries are checked via
    /// `GameEnvironment::semantic_action_is_legal`, which is authoritative
    /// and doesn't require materializing more than the one action being
    /// checked; region sizes here are always small (card subset count,
    /// shop/inventory/treasure counts), never the O(subset x position)
    /// `BuildTower` space.
    pub fn legal_mask(&self) -> &[bool] {
        &self.legal_mask
    }

    /// `policy_action_index -> AgentAction`, independent of legality (an
    /// out-of-range index still returns `None`; an in-range but currently
    /// illegal index still returns `Some` - check `legal_mask` separately,
    /// same split `DenseBuildTowerScoreTable` already uses between
    /// `score`/materialization and its legality mask).
    pub fn index_to_action(&self, index: usize) -> Option<AgentAction> {
        match &self.layout {
            Layout::CardDecision {
                build_table,
                build_tower_len,
                reroll_len,
                shop_len,
                inventory_len,
                treasure_upgrade_ids,
            } => {
                let mut index = index;
                if index < *build_tower_len {
                    let (subset_index, hand_slot_index, position_index) =
                        decompose_build_tower_index(index, build_table);
                    return crate::joint_action::build_tower_action(
                        &build_table.subsets,
                        subset_index,
                        hand_slot_index,
                        position_index,
                    );
                }
                index -= build_tower_len;
                if index < *reroll_len {
                    return build_table
                        .subsets
                        .explicit_card_ids_for_subset(index)
                        .map(|card_ids| AgentAction::Reroll { card_ids });
                }
                index -= reroll_len;
                if index < *shop_len {
                    return Some(AgentAction::PurchaseShopItem { slot_index: index });
                }
                index -= shop_len;
                if index < *inventory_len {
                    return Some(AgentAction::UseInventoryItem { item_index: index });
                }
                index -= inventory_len;
                treasure_upgrade_ids
                    .get(index)
                    .map(|&upgrade_id| AgentAction::DiscardTreasure { upgrade_id })
            }
            Layout::Other { actions } => actions.get(index).cloned(),
        }
    }

    /// `AgentAction -> policy_action_index`, rejecting (`None`) anything
    /// that isn't legal in the state this space was computed from -
    /// whether because it isn't representable at all (wrong action kind
    /// for this decision point, out-of-range field) or because it's
    /// in-range but currently illegal. `card_ids` order never matters:
    /// this canonicalizes before comparing.
    pub fn action_to_index(&self, action: &AgentAction) -> Option<usize> {
        let index = match &self.layout {
            Layout::CardDecision {
                build_table,
                build_tower_len,
                reroll_len,
                shop_len,
                inventory_len,
                treasure_upgrade_ids,
            } => match action {
                AgentAction::BuildTower {
                    card_ids,
                    hand_slot_index,
                    left,
                    top,
                } => {
                    let subset_index = build_table.subsets.subset_index_for_card_ids(card_ids)?;
                    let position_index = crate::joint_action::position_index(*left, *top)?;
                    if *hand_slot_index >= build_table.build_slot_count {
                        return None;
                    }
                    Some(compose_build_tower_index(
                        subset_index,
                        *hand_slot_index,
                        position_index,
                        build_table,
                    ))
                }
                AgentAction::Reroll { card_ids } => build_table
                    .subsets
                    .subset_index_for_card_ids(card_ids)
                    .map(|subset_index| build_tower_len + subset_index),
                AgentAction::PurchaseShopItem { slot_index } => {
                    (*slot_index < *shop_len).then(|| build_tower_len + reroll_len + slot_index)
                }
                AgentAction::UseInventoryItem { item_index } => (*item_index < *inventory_len)
                    .then(|| build_tower_len + reroll_len + shop_len + item_index),
                AgentAction::DiscardTreasure { upgrade_id } => treasure_upgrade_ids
                    .iter()
                    .position(|id| id == upgrade_id)
                    .map(|position| {
                        build_tower_len + reroll_len + shop_len + inventory_len + position
                    }),
                _ => None,
            },
            Layout::Other { actions } => {
                let normalized = canonicalize(action);
                actions
                    .iter()
                    .position(|candidate| canonicalize(candidate) == normalized)
            }
        };
        index.filter(|&index| self.legal_mask.get(index).copied().unwrap_or(false))
    }
}

fn decompose_build_tower_index(
    index: usize,
    build_table: &DenseBuildTowerScoreTable,
) -> (usize, usize, usize) {
    let per_subset = build_table.build_slot_count * build_table.position_count;
    let subset_index = index / per_subset;
    let remainder = index % per_subset;
    (
        subset_index,
        remainder / build_table.position_count,
        remainder % build_table.position_count,
    )
}

fn compose_build_tower_index(
    subset_index: usize,
    hand_slot_index: usize,
    position_index: usize,
    build_table: &DenseBuildTowerScoreTable,
) -> usize {
    (subset_index * build_table.build_slot_count + hand_slot_index) * build_table.position_count
        + position_index
}

fn compute_legal_mask(environment: &GameEnvironment, layout: &Layout) -> Vec<bool> {
    match layout {
        Layout::CardDecision {
            build_table,
            build_tower_len,
            reroll_len,
            shop_len,
            inventory_len,
            treasure_upgrade_ids,
        } => {
            let mut mask = Vec::with_capacity(
                build_tower_len
                    + reroll_len
                    + shop_len
                    + inventory_len
                    + treasure_upgrade_ids.len(),
            );
            for subset_index in 0..build_table.subsets.subset_count() {
                for hand_slot_index in 0..build_table.build_slot_count {
                    for position_index in 0..build_table.position_count {
                        mask.push(
                            build_table
                                .score(subset_index, hand_slot_index, position_index)
                                .is_some(),
                        );
                    }
                }
            }
            debug_assert_eq!(mask.len(), *build_tower_len);
            for subset_index in 0..*reroll_len {
                let legal = build_table
                    .subsets
                    .explicit_card_ids_for_subset(subset_index)
                    .is_some_and(|card_ids| {
                        environment.semantic_action_is_legal(&AgentAction::Reroll { card_ids })
                    });
                mask.push(legal);
            }
            for slot_index in 0..*shop_len {
                mask.push(
                    environment
                        .semantic_action_is_legal(&AgentAction::PurchaseShopItem { slot_index }),
                );
            }
            for item_index in 0..*inventory_len {
                mask.push(
                    environment
                        .semantic_action_is_legal(&AgentAction::UseInventoryItem { item_index }),
                );
            }
            for &upgrade_id in treasure_upgrade_ids {
                mask.push(
                    environment
                        .semantic_action_is_legal(&AgentAction::DiscardTreasure { upgrade_id }),
                );
            }
            mask
        }
        // GameEnvironment::legal_actions() already returns exclusively
        // legal actions.
        Layout::Other { actions } => vec![true; actions.len()],
    }
}

/// Two actions are the same for indexing purposes iff they're equal after
/// sorting `card_ids` - the only field whose *order* isn't part of
/// semantic identity (see `CardSubsetTable`'s module docs).
fn canonicalize(action: &AgentAction) -> AgentAction {
    match action {
        AgentAction::Reroll { card_ids } => AgentAction::Reroll {
            card_ids: sorted(card_ids),
        },
        AgentAction::SelectTower { card_ids } => AgentAction::SelectTower {
            card_ids: sorted(card_ids),
        },
        AgentAction::BuildTower {
            card_ids,
            hand_slot_index,
            left,
            top,
        } => AgentAction::BuildTower {
            card_ids: sorted(card_ids),
            hand_slot_index: *hand_slot_index,
            left: *left,
            top: *top,
        },
        other => other.clone(),
    }
}

fn sorted(card_ids: &[usize]) -> Vec<usize> {
    let mut sorted = card_ids.to_vec();
    sorted.sort_unstable();
    sorted
}

/// `ActionKind`s that may legitimately appear in the `Other` layout: every
/// non-card-decision point that is still a semantic macro-action decision
/// per `docs/game-ai/02-action-contract.md`. UI/FSM micro-action kinds with
/// no strategic content of their own - `StartSelectingTower`,
/// `BeginRerollSelection`, `BeginTowerSelection`, `SelectHandCard`,
/// `DeselectHandCard`, `ConfirmCardSelection`, `CancelCardSelection` (the
/// hand-card-selection UI dance `BuildTower`/`Reroll` exist to collapse into
/// one macro decision) - are deliberately excluded.
///
/// `PlaceTower` *is* included: when `stage_modifiers.extra_tower_cards` is
/// non-empty, `SelectTower` queues more than one already-built tower (the
/// chosen subset's own template plus one fixed template per extra-card
/// entry - see `GameEnvironment::build_tower_slot_count`), and one
/// `BuildTower` macro only places one of them (`hand_slot_index`). The
/// remaining pre-built towers have no card content left to choose - the only
/// remaining decision is *where* to place an already-fixed tower, which is
/// exactly as strategic as a normal placement and genuinely cannot be
/// bundled into the original `BuildTower` decision (a single macro step
/// carries only one position). This is why `DecisionPoint::TowerPlacement`
/// is ever externally observed at all along a semantic rollout - see
/// `teacher.rs`'s `extra_slot_actions_differing_only_by_subset_produce_different_states`
/// and `GameEnvironment::semantic_legal_actions_with_position_limit`, which
/// already falls back to raw `PlaceTower` legal actions in exactly this
/// state today. See the `assert!` in `PolicyActionSpace::compute` and
/// `policy_action_space_never_exposes_ui_micro_actions_along_semantic_rollouts`.
fn is_other_layout_policy_visible(kind: ActionKind) -> bool {
    matches!(
        kind,
        ActionKind::PlaceTower
            | ActionKind::RemoveTower
            | ActionKind::StartDefense
            | ActionKind::SelectTreasure
            | ActionKind::SelectCardServiceCard
            | ActionKind::ConfirmCardServiceSelection
            | ActionKind::UseInventoryItem
            | ActionKind::DiscardTreasure
            | ActionKind::Continue
    )
}

/// Canonical ordering for the `Other` layout: by `ActionKind` in a fixed,
/// explicit priority (mirroring the kind's declaration order in
/// `td_core::game_state::command`, a static source-code property - not
/// runtime generation order or hashmap iteration), then by that kind's
/// *complete* semantic identity - never a partial key such as "the minimum
/// card id" or "the hand slot index alone" that could let two different
/// actions collapse onto the same key. Never `action_id()`'s string sort,
/// which is opaque and fragile to format changes.
///
/// The per-kind identity vector is `sorted(card_ids)` (variable length, and
/// order-insensitive - matching `canonicalize`) followed by a *fixed* number
/// of remaining scalar fields for that kind (e.g. `[hand_slot_index, left,
/// top]` for `BuildTower`). Because the fixed suffix always has the same
/// length for a given kind, two keys of equal length and equal content can
/// only arise from card-id sets of equal size, which (after sorting) forces
/// the sets themselves to be identical - so equal keys imply structurally
/// identical actions (up to `card_ids` order, which is intentional). This is
/// what makes the ordering a true total order over distinct semantic
/// identities rather than a lossy projection of them.
fn canonical_sort_key(action: &AgentAction) -> (u8, Vec<u64>) {
    let kind_rank = match action.kind() {
        ActionKind::PurchaseShopItem => 0,
        ActionKind::StartSelectingTower => 1,
        ActionKind::BeginRerollSelection => 2,
        ActionKind::BeginTowerSelection => 3,
        ActionKind::SelectHandCard => 4,
        ActionKind::DeselectHandCard => 5,
        ActionKind::ConfirmCardSelection => 6,
        ActionKind::CancelCardSelection => 7,
        ActionKind::Reroll => 8,
        ActionKind::SelectTower => 9,
        ActionKind::BuildTower => 10,
        ActionKind::PlaceTower => 11,
        ActionKind::RemoveTower => 12,
        ActionKind::StartDefense => 13,
        ActionKind::SelectTreasure => 14,
        ActionKind::SelectCardServiceCard => 15,
        ActionKind::ConfirmCardServiceSelection => 16,
        ActionKind::UseInventoryItem => 17,
        ActionKind::DiscardTreasure => 18,
        ActionKind::Continue => 19,
    };
    let identity = match action {
        AgentAction::PurchaseShopItem { slot_index } => vec![*slot_index as u64],
        AgentAction::StartSelectingTower
        | AgentAction::BeginRerollSelection
        | AgentAction::BeginTowerSelection
        | AgentAction::ConfirmCardSelection
        | AgentAction::CancelCardSelection
        | AgentAction::StartDefense
        | AgentAction::ConfirmCardServiceSelection
        | AgentAction::Continue => Vec::new(),
        AgentAction::SelectHandCard { hand_slot_index }
        | AgentAction::DeselectHandCard { hand_slot_index } => vec![*hand_slot_index as u64],
        AgentAction::Reroll { card_ids } | AgentAction::SelectTower { card_ids } => {
            sorted_u64(card_ids)
        }
        AgentAction::BuildTower {
            card_ids,
            hand_slot_index,
            left,
            top,
        } => {
            let mut identity = sorted_u64(card_ids);
            identity.push(*hand_slot_index as u64);
            identity.push(*left as u64);
            identity.push(*top as u64);
            identity
        }
        AgentAction::PlaceTower {
            hand_slot_index,
            left,
            top,
        } => vec![*hand_slot_index as u64, *left as u64, *top as u64],
        AgentAction::RemoveTower { tower_id } => vec![*tower_id],
        AgentAction::SelectTreasure { option_index } => vec![*option_index as u64],
        AgentAction::SelectCardServiceCard { card_index } => vec![*card_index as u64],
        AgentAction::UseInventoryItem { item_index } => vec![*item_index as u64],
        AgentAction::DiscardTreasure { upgrade_id } => vec![*upgrade_id],
    };
    (kind_rank, identity)
}

/// `card_ids`' order isn't part of semantic identity (see `canonicalize`);
/// this returns the sorted, order-insensitive identity as `u64` for use in
/// `canonical_sort_key`.
fn sorted_u64(card_ids: &[usize]) -> Vec<u64> {
    let mut ids: Vec<u64> = card_ids.iter().map(|&id| id as u64).collect();
    ids.sort_unstable();
    ids
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::GameConfig;
    use std::sync::Arc;

    fn environment(seed: u64) -> GameEnvironment {
        let mut environment = GameEnvironment::new(Arc::new(GameConfig::default_config()), seed);
        environment
            .step(AgentAction::StartSelectingTower)
            .expect("start selecting tower should be legal");
        environment
    }

    fn extra_tower_cards_environment(extra_count: usize) -> GameEnvironment {
        let mut environment = environment(0);
        environment
            .test_only_seed_extra_tower_cards(extra_count)
            .expect("extra tower card fixture should be a valid snapshot");
        environment
    }

    #[test]
    fn round_trip_holds_for_every_legal_action_in_card_decision_states() {
        for environment in [
            environment(0),
            environment(1),
            extra_tower_cards_environment(1),
        ] {
            let space = PolicyActionSpace::compute(&environment);
            let mut checked = 0usize;
            for index in 0..space.action_count() {
                if !space.legal_mask()[index] {
                    continue;
                }
                let action = space
                    .index_to_action(index)
                    .unwrap_or_else(|| panic!("legal index {index} should materialize an action"));
                let round_tripped = space.action_to_index(&action);
                assert_eq!(
                    round_tripped,
                    Some(index),
                    "index {index} (action {action:?}) did not round-trip"
                );
                checked += 1;
            }
            assert!(checked > 0, "expected at least one legal action");
        }
    }

    #[test]
    fn no_two_legal_indices_materialize_the_same_action() {
        for environment in [environment(0), extra_tower_cards_environment(2)] {
            let space = PolicyActionSpace::compute(&environment);
            let mut seen = std::collections::HashSet::new();
            for index in 0..space.action_count() {
                if !space.legal_mask()[index] {
                    continue;
                }
                let action = space
                    .index_to_action(index)
                    .expect("legal index materializes");
                assert!(
                    seen.insert(action.action_id()),
                    "index {index} duplicates an earlier action: {action:?}"
                );
            }
        }
    }

    #[test]
    fn illegal_and_nonexistent_actions_are_rejected_by_reverse_mapping() {
        let environment = environment(0);
        let space = PolicyActionSpace::compute(&environment);
        // Out-of-range hand_slot_index for BuildTower.
        assert_eq!(
            space.action_to_index(&AgentAction::BuildTower {
                card_ids: vec![],
                hand_slot_index: 999,
                left: 0,
                top: 0,
            }),
            None
        );
        // Card ids that aren't a subset of the current hand.
        assert_eq!(
            space.action_to_index(&AgentAction::Reroll {
                card_ids: vec![999_999],
            }),
            None
        );
        // Action kind that never appears at a card decision point.
        assert_eq!(space.action_to_index(&AgentAction::StartDefense), None);
    }

    #[test]
    fn legal_mask_support_matches_exhaustive_semantic_oracle_bidirectionally() {
        let environments = [
            environment(0),
            environment(1),
            environment(2),
            extra_tower_cards_environment(1),
            extra_tower_cards_environment(2),
        ];
        for (index, environment) in environments.iter().enumerate() {
            let space = PolicyActionSpace::compute(environment);
            let oracle = environment
                .semantic_legal_actions()
                .into_iter()
                .map(|legal| canonicalize(&legal.action).action_id())
                .collect::<std::collections::HashSet<_>>();
            assert!(
                !oracle.is_empty(),
                "environment {index}: expected a non-empty oracle set"
            );

            let mut space_actions = std::collections::HashSet::new();
            for policy_index in 0..space.action_count() {
                if !space.legal_mask()[policy_index] {
                    continue;
                }
                let action = space
                    .index_to_action(policy_index)
                    .expect("legal index materializes");
                space_actions.insert(canonicalize(&action).action_id());
            }

            assert_eq!(
                space_actions, oracle,
                "environment {index}: legal policy action support must match the semantic oracle exactly"
            );
        }
    }

    #[test]
    fn layout_and_mask_are_deterministic_across_repeated_computation() {
        let environment = environment(0);
        let first = PolicyActionSpace::compute(&environment);
        let second = PolicyActionSpace::compute(&environment);
        assert_eq!(first.action_count(), second.action_count());
        assert_eq!(first.legal_mask(), second.legal_mask());
        for index in 0..first.action_count() {
            assert_eq!(
                first.index_to_action(index).map(|a| a.action_id()),
                second.index_to_action(index).map(|a| a.action_id()),
                "index {index} differed between repeated computations"
            );
        }
    }

    #[test]
    fn other_layout_is_used_and_correct_outside_card_decisions() {
        // Advance past the initial card decision into a non-card-decision
        // state (Defense), where the Other layout applies.
        let mut environment = environment(0);
        loop {
            let observation = environment.snapshot();
            if !environment.semantic_card_decision_available() {
                break;
            }
            let legal = environment.semantic_legal_actions();
            let action = crate::policy_runner::scripted_expert_action(&observation, &legal)
                .expect("scripted expert should find an action");
            let outcome = environment
                .semantic_step(action)
                .expect("scripted step should be accepted");
            if outcome.terminated || outcome.truncated {
                return;
            }
        }
        assert!(!environment.semantic_card_decision_available());
        let space = PolicyActionSpace::compute(&environment);
        let oracle = environment
            .legal_actions()
            .into_iter()
            .map(|legal| canonicalize(&legal.action).action_id())
            .collect::<std::collections::HashSet<_>>();
        let mut space_actions = std::collections::HashSet::new();
        for index in 0..space.action_count() {
            assert!(
                space.legal_mask()[index],
                "Other layout entries must all be legal"
            );
            space_actions.insert(canonicalize(&space.index_to_action(index).unwrap()).action_id());
        }
        assert_eq!(space_actions, oracle);
    }

    /// Problem 1 (semantic policy boundary): drives full episodes exclusively
    /// through `GameEnvironment::semantic_step` - the actual policy decision
    /// boundary a rollout uses (see `policy_runner`'s `semantic_actions=true`
    /// loop, which calls `semantic_legal_actions_with_position_limit` then
    /// `semantic_step`, never raw `step` for a policy-chosen action) - and
    /// calls `PolicyActionSpace::compute` at every single decision point
    /// reached along the way, exactly as a future model-inference call site
    /// would. `PolicyActionSpace::compute`'s internal `assert!` (see the
    /// `Other` layout branch and `is_other_layout_policy_visible`) would
    /// panic and fail this test if a UI/FSM micro-action with no strategic
    /// content of its own - a mid card-selection step
    /// (`SelectHandCard`/`DeselectHandCard`/`ConfirmCardSelection`/
    /// `CancelCardSelection`, or a bare `BeginRerollSelection`/
    /// `BeginTowerSelection`/`StartSelectingTower`) - ever reached
    /// policy-visible territory. This never happens because `semantic_step`'s
    /// `Reroll`/`BuildTower` handling always drives its internal
    /// `StartSelectingTower`/`SelectTower`/`Reroll`/`PlaceTower` sub-steps to
    /// completion within a single call, never returning control to the
    /// caller mid card-selection.
    ///
    /// `PlaceTower` at a `TowerPlacement` decision point legitimately does
    /// appear here (covered by the `extra_tower_cards` seeds below) - it's
    /// not a UI micro-action, see `is_other_layout_policy_visible`'s doc
    /// comment.
    #[test]
    fn policy_action_space_never_exposes_ui_micro_actions_along_semantic_rollouts() {
        let scenarios = (0..6u64)
            .map(|seed| (format!("seed {seed}"), environment(seed)))
            .chain((1..3usize).map(|extra_count| {
                (
                    format!("extra_tower_cards({extra_count})"),
                    extra_tower_cards_environment(extra_count),
                )
            }));
        for (label, mut environment) in scenarios {
            let mut decisions = 0usize;
            loop {
                // The actual policy decision boundary: compute the dense
                // action space before every semantic step, just as a wired-in
                // model would.
                let space = PolicyActionSpace::compute(&environment);
                assert!(
                    space.action_count() > 0,
                    "{label} decision {decisions}: expected a non-empty policy action space"
                );

                let observation = environment.snapshot();
                let legal = environment.semantic_legal_actions();
                let Ok(action) = crate::policy_runner::scripted_expert_action(&observation, &legal)
                else {
                    break;
                };
                let Ok(outcome) = environment.semantic_step(action) else {
                    break;
                };
                decisions += 1;
                if outcome.terminated || outcome.truncated || decisions > 300 {
                    break;
                }
            }
            assert!(decisions > 0, "{label}: expected at least one decision");
        }
    }

    /// The flip side of the invariant above: `PolicyActionSpace::compute`
    /// does not merely happen to avoid UI micro-actions along the semantic
    /// contract - it actively rejects (via `assert!`) any state where they'd
    /// leak, so a future regression in `semantic_step`'s atomicity (or a
    /// caller bypassing it) fails loudly instead of silently training a
    /// policy over UI/FSM steps. This reaches that forbidden state
    /// deliberately, via the legacy raw `step` UI dance
    /// (`BeginTowerSelection` leaves `decision_context` as a mid-selection
    /// `CardSelection`), which `semantic_step` itself never does.
    #[test]
    #[should_panic(expected = "PolicyActionSpace invariant violated")]
    fn other_layout_rejects_a_mid_card_selection_state_reached_outside_the_semantic_contract() {
        let mut environment = environment(0);
        environment
            .step(AgentAction::BeginTowerSelection)
            .expect("begin tower selection should be legal");
        assert!(!environment.semantic_card_decision_available());
        let _ = PolicyActionSpace::compute(&environment);
    }

    /// Problem 2 (canonical ordering): two different `Reroll`/`SelectTower`
    /// card subsets, and two different `PlaceTower` positions, must never
    /// collapse onto the same canonical sort key. The only key collisions
    /// allowed are between actions that `canonicalize` already considers the
    /// same semantic identity (i.e. `card_ids` differing only in order).
    #[test]
    fn canonical_sort_key_distinguishes_all_semantic_identities() {
        let actions = vec![
            AgentAction::PurchaseShopItem { slot_index: 0 },
            AgentAction::PurchaseShopItem { slot_index: 1 },
            AgentAction::SelectHandCard { hand_slot_index: 0 },
            AgentAction::SelectHandCard { hand_slot_index: 1 },
            AgentAction::DeselectHandCard { hand_slot_index: 0 },
            AgentAction::DeselectHandCard { hand_slot_index: 1 },
            AgentAction::Reroll {
                card_ids: vec![1, 2],
            },
            AgentAction::Reroll {
                card_ids: vec![2, 1],
            },
            AgentAction::Reroll {
                card_ids: vec![1, 3],
            },
            AgentAction::Reroll { card_ids: vec![1] },
            AgentAction::Reroll { card_ids: vec![3] },
            AgentAction::SelectTower {
                card_ids: vec![1, 2],
            },
            AgentAction::SelectTower {
                card_ids: vec![2, 1],
            },
            AgentAction::SelectTower {
                card_ids: vec![1, 3],
            },
            AgentAction::BuildTower {
                card_ids: vec![1, 2],
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::BuildTower {
                card_ids: vec![1, 2],
                hand_slot_index: 0,
                left: 0,
                top: 1,
            },
            AgentAction::BuildTower {
                card_ids: vec![1, 2],
                hand_slot_index: 0,
                left: 1,
                top: 0,
            },
            AgentAction::BuildTower {
                card_ids: vec![1, 2],
                hand_slot_index: 1,
                left: 0,
                top: 0,
            },
            AgentAction::BuildTower {
                card_ids: vec![1, 2, 3],
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::BuildTower {
                card_ids: vec![2, 1],
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 0,
            },
            AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 0,
                top: 1,
            },
            AgentAction::PlaceTower {
                hand_slot_index: 0,
                left: 1,
                top: 0,
            },
            AgentAction::PlaceTower {
                hand_slot_index: 1,
                left: 0,
                top: 0,
            },
            AgentAction::RemoveTower { tower_id: 1 },
            AgentAction::RemoveTower { tower_id: 2 },
            AgentAction::SelectTreasure { option_index: 0 },
            AgentAction::SelectTreasure { option_index: 1 },
            AgentAction::SelectCardServiceCard { card_index: 0 },
            AgentAction::SelectCardServiceCard { card_index: 1 },
            AgentAction::UseInventoryItem { item_index: 0 },
            AgentAction::UseInventoryItem { item_index: 1 },
            AgentAction::DiscardTreasure { upgrade_id: 1 },
            AgentAction::DiscardTreasure { upgrade_id: 2 },
            AgentAction::StartSelectingTower,
            AgentAction::BeginRerollSelection,
            AgentAction::BeginTowerSelection,
            AgentAction::ConfirmCardSelection,
            AgentAction::CancelCardSelection,
            AgentAction::StartDefense,
            AgentAction::ConfirmCardServiceSelection,
            AgentAction::Continue,
        ];

        let mut groups: std::collections::HashMap<(u8, Vec<u64>), Vec<AgentAction>> =
            std::collections::HashMap::new();
        for action in &actions {
            groups
                .entry(canonical_sort_key(action))
                .or_default()
                .push(action.clone());
        }
        for (key, group) in &groups {
            let distinct_identities = group
                .iter()
                .map(|action| format!("{:?}", canonicalize(action)))
                .collect::<std::collections::HashSet<_>>();
            assert_eq!(
                distinct_identities.len(),
                1,
                "distinct semantic actions collapsed onto the same canonical sort key {key:?}: {group:?}"
            );
        }
    }

    /// Problem 2 (canonical ordering): sorting by `canonical_sort_key` must
    /// produce the same resulting sequence no matter what order the legal
    /// actions were generated in. This is strictly stronger than repeating
    /// `PolicyActionSpace::compute` on the same environment twice (which
    /// wouldn't catch an ordering bug tied to input order, since
    /// `GameEnvironment::legal_actions` itself generates in a fixed order
    /// every time) - it directly shuffles the *input* to the sort.
    #[test]
    fn canonical_sort_key_orders_actions_independent_of_input_permutation() {
        let base_actions = vec![
            AgentAction::PurchaseShopItem { slot_index: 2 },
            AgentAction::PurchaseShopItem { slot_index: 0 },
            AgentAction::Reroll {
                card_ids: vec![5, 1],
            },
            AgentAction::Reroll { card_ids: vec![2] },
            AgentAction::Reroll {
                card_ids: vec![5, 1, 9],
            },
            AgentAction::SelectTower {
                card_ids: vec![3, 1],
            },
            AgentAction::BuildTower {
                card_ids: vec![4, 2],
                hand_slot_index: 1,
                left: 3,
                top: 2,
            },
            AgentAction::BuildTower {
                card_ids: vec![4, 2],
                hand_slot_index: 0,
                left: 3,
                top: 2,
            },
            AgentAction::PlaceTower {
                hand_slot_index: 1,
                left: 3,
                top: 2,
            },
            AgentAction::PlaceTower {
                hand_slot_index: 1,
                left: 2,
                top: 3,
            },
            AgentAction::RemoveTower { tower_id: 42 },
            AgentAction::SelectTreasure { option_index: 1 },
            AgentAction::SelectCardServiceCard { card_index: 4 },
            AgentAction::UseInventoryItem { item_index: 3 },
            AgentAction::DiscardTreasure { upgrade_id: 7 },
            AgentAction::StartDefense,
            AgentAction::Continue,
        ];

        let sorted_key = |actions: &mut Vec<AgentAction>| {
            actions.sort_by_key(canonical_sort_key);
        };

        let mut expected = base_actions.clone();
        sorted_key(&mut expected);

        let mut permutations = Vec::new();
        let mut reversed = base_actions.clone();
        reversed.reverse();
        permutations.push(reversed);
        for shift in [1usize, 4, 9] {
            let mut rotated = base_actions.clone();
            let shift = shift % rotated.len();
            rotated.rotate_left(shift);
            permutations.push(rotated);
        }
        for seed in 0..5u64 {
            let mut shuffled = base_actions.clone();
            let mut state = seed.wrapping_add(0x9E3779B97F4A7C15);
            for i in (1..shuffled.len()).rev() {
                state = state
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
                let j = ((state >> 33) as usize) % (i + 1);
                shuffled.swap(i, j);
            }
            permutations.push(shuffled);
        }

        for mut permuted in permutations {
            sorted_key(&mut permuted);
            assert_eq!(
                permuted, expected,
                "canonical ordering depended on the input action order"
            );
        }
    }

    #[derive(serde::Serialize)]
    struct PolicyActionBenchmarkSamplePoint {
        seed: u64,
        decision_index: usize,
        card_decision_available: bool,
        policy_layout_generation_seconds: f64,
        policy_action_count: usize,
        legacy_enumeration_seconds: f64,
        legacy_action_count: usize,
    }

    #[derive(serde::Serialize)]
    struct PolicyActionBenchmarkReport {
        methodology: String,
        sample_points: Vec<PolicyActionBenchmarkSamplePoint>,
        mean_policy_layout_generation_seconds: f64,
        mean_legacy_enumeration_seconds: f64,
        layout_generation_speedup: f64,
        mean_policy_action_count: f64,
        mean_legacy_action_count: f64,
    }

    /// Compares `PolicyActionSpace::compute` (the new dense contract) against
    /// `GameEnvironment::semantic_legal_actions()` (the legacy exhaustive
    /// enumeration, which materializes every `BuildTower` action up front)
    /// on the same sampled decision states. This isn't a performance-tuning
    /// benchmark - it exists to catch a catastrophic regression back to
    /// upfront `BuildTower` materialization, matching the same shape as
    /// `joint_action::tests::dense_scorer_vs_exhaustive_heuristic_benchmark`
    /// and `teacher::tests::dense_candidate_migration_benchmark`.
    ///
    /// Run with: cargo test --release -- --ignored policy_action_space_benchmark --nocapture
    #[test]
    #[ignore = "manual release benchmark; writes artifacts/benchmarks/policy-action-space-benchmark.json"]
    fn policy_action_space_benchmark() {
        use std::time::Instant;

        const SEEDS: std::ops::Range<u64> = 0..8;
        const SAMPLES_PER_SEED: usize = 6;
        const MAX_DECISIONS_PER_SEED: usize = 40;

        let mut sample_points = Vec::new();
        for seed in SEEDS {
            let mut environment = environment(seed);
            let mut samples_collected = 0usize;
            let mut decision_index = 0usize;
            while samples_collected < SAMPLES_PER_SEED && decision_index < MAX_DECISIONS_PER_SEED {
                decision_index += 1;
                let observation = environment.snapshot();

                let policy_start = Instant::now();
                let space = PolicyActionSpace::compute(&environment);
                let policy_layout_generation_seconds = policy_start.elapsed().as_secs_f64();

                let legacy_start = Instant::now();
                let legacy_action_count = environment.semantic_legal_actions().len();
                let legacy_enumeration_seconds = legacy_start.elapsed().as_secs_f64();

                sample_points.push(PolicyActionBenchmarkSamplePoint {
                    seed,
                    decision_index,
                    card_decision_available: environment.semantic_card_decision_available(),
                    policy_layout_generation_seconds,
                    policy_action_count: space.action_count(),
                    legacy_enumeration_seconds,
                    legacy_action_count,
                });
                samples_collected += 1;

                let legal_actions = environment.semantic_legal_actions();
                let Ok(action) =
                    crate::policy_runner::scripted_expert_action(&observation, &legal_actions)
                else {
                    break;
                };
                let Ok(outcome) = environment.semantic_step(action) else {
                    break;
                };
                if outcome.terminated || outcome.truncated {
                    break;
                }
            }
        }

        let count = sample_points.len().max(1) as f64;
        let mean = |values: Vec<f64>| values.iter().sum::<f64>() / count;
        let mean_policy_layout_generation_seconds = mean(
            sample_points
                .iter()
                .map(|s| s.policy_layout_generation_seconds)
                .collect(),
        );
        let mean_legacy_enumeration_seconds = mean(
            sample_points
                .iter()
                .map(|s| s.legacy_enumeration_seconds)
                .collect(),
        );
        let mean_policy_action_count = mean(
            sample_points
                .iter()
                .map(|s| s.policy_action_count as f64)
                .collect(),
        );
        let mean_legacy_action_count = mean(
            sample_points
                .iter()
                .map(|s| s.legacy_action_count as f64)
                .collect(),
        );
        let layout_generation_speedup = mean_legacy_enumeration_seconds
            / mean_policy_layout_generation_seconds.max(f64::EPSILON);

        println!(
            "sample_count={} mean_policy_layout_generation_seconds={:.6} \
            mean_legacy_enumeration_seconds={:.6} layout_generation_speedup={:.2}x \
            mean_policy_action_count={:.1} mean_legacy_action_count={:.1}",
            sample_points.len(),
            mean_policy_layout_generation_seconds,
            mean_legacy_enumeration_seconds,
            layout_generation_speedup,
            mean_policy_action_count,
            mean_legacy_action_count,
        );

        let report = PolicyActionBenchmarkReport {
            methodology: "policy_layout_generation_seconds times PolicyActionSpace::compute (the \
                new dense contract: DenseBuildTowerScoreTable::compute for the BuildTower region \
                plus small per-kind legality checks for Reroll/PurchaseShopItem/UseInventoryItem/ \
                DiscardTreasure, never materializing an AgentAction until index_to_action is called \
                for a specific index). legacy_enumeration_seconds times \
                GameEnvironment::semantic_legal_actions() (position_candidate_limit=None), the \
                pre-dense-migration exhaustive generator, which materializes every BuildTower \
                AgentAction across every card subset x every legal position up front. Same sampled \
                decision states for both (SEEDS x SAMPLES_PER_SEED, scripted-expert trajectory, \
                unrelated to and not affected by candidate_limit/position_candidate_limit)."
                .to_string(),
            sample_points,
            mean_policy_layout_generation_seconds,
            mean_legacy_enumeration_seconds,
            layout_generation_speedup,
            mean_policy_action_count,
            mean_legacy_action_count,
        };

        let json = serde_json::to_string_pretty(&report).expect("report should serialize");
        std::fs::create_dir_all("../artifacts/benchmarks")
            .expect("benchmark artifact directory should be creatable");
        std::fs::write(
            "../artifacts/benchmarks/policy-action-space-benchmark.json",
            &json,
        )
        .expect("benchmark report should be written");
    }
}
