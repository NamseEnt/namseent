//! Tower placement strategies.

use super::TowerPlacementStrategy;
use crate::game_state::can_place_tower::can_place_tower;
use crate::game_state::flow::GameFlow;
use crate::game_state::tower::Tower;
use crate::game_state::{GameState, MAP_SIZE, TRAVEL_POINTS};
use crate::MapCoord;
use namui::*;

/// Spiral placement strategy from debug tools.
/// Places towers in a predefined spiral pattern from center outward.
pub struct SpiralPlacementStrategy;

impl TowerPlacementStrategy for SpiralPlacementStrategy {
    fn name(&self) -> &str {
        "spiral"
    }

    fn execute_placement(&self, game_state: &mut GameState) {
        loop {
            if !matches!(game_state.flow, GameFlow::PlacingTower) {
                break;
            }

            let (slot_id, template) = {
                let Some(&slot_id) = game_state.hand.selected_slot_ids().first() else {
                    // Try to select the first tower slot
                    if let Some(first_id) = game_state.hand.get_slot_id_by_index(0) {
                        if game_state
                            .hand
                            .get_item(first_id)
                            .and_then(|item| item.as_tower())
                            .is_some()
                        {
                            game_state.hand.select_slot(first_id);
                            continue;
                        }
                    }
                    break;
                };
                let Some(template) = game_state
                    .hand
                    .get_item(slot_id)
                    .and_then(|item| item.as_tower())
                    .cloned()
                else {
                    break;
                };
                (slot_id, template)
            };

            let now = game_state.now();
            let mut placed = false;

            for step in placement_plan() {
                match step {
                    PlanStep::Remove(coord) => {
                        if let Some(tower_id) =
                            game_state.towers.find_by_xy(coord).map(|t| t.id())
                        {
                            game_state.remove_tower(tower_id);
                        }
                    }
                    PlanStep::Place(left_top) => {
                        if left_top.x + 1 >= MAP_SIZE.width || left_top.y + 1 >= MAP_SIZE.height {
                            continue;
                        }
                        let placed_coords = game_state.towers.coords();
                        let route_coords: Vec<MapCoord> =
                            game_state.route.iter_coords().to_vec();

                        if can_place_tower(
                            left_top,
                            Wh::new(2, 2),
                            &TRAVEL_POINTS,
                            &placed_coords,
                            &route_coords,
                            MAP_SIZE,
                        ) {
                            let tower = Tower::new(&template, left_top, now);
                            game_state.place_tower(tower);
                            game_state.hand.delete_slots(&[slot_id]);

                            // Select next tower if available
                            if let Some(first_id) = game_state.hand.get_slot_id_by_index(0) {
                                if game_state
                                    .hand
                                    .get_item(first_id)
                                    .and_then(|item| item.as_tower())
                                    .is_some()
                                {
                                    game_state.hand.select_slot(first_id);
                                }
                            }

                            placed = true;
                            break;
                        }
                    }
                }
            }

            if !placed {
                // Can't place anywhere, skip remaining towers
                break;
            }

            // Check if hand is empty -> go to defense
            if game_state.hand.is_empty() {
                game_state.goto_defense();
                break;
            }
        }

        // If still in PlacingTower and has items, go to defense anyway
        if matches!(game_state.flow, GameFlow::PlacingTower) {
            game_state.goto_defense();
        }
    }
}

#[derive(Clone, Copy)]
enum PlanStep {
    Place(MapCoord),
    Remove(MapCoord),
}

fn placement_plan() -> Vec<PlanStep> {
    use PlanStep::{Place, Remove};
    vec![
        Place(MapCoord::new(18, 18)),
        Place(MapCoord::new(18, 16)),
        Place(MapCoord::new(20, 17)),
        Place(MapCoord::new(16, 17)),
        Place(MapCoord::new(14, 17)),
        Place(MapCoord::new(12, 17)),
        Place(MapCoord::new(10, 17)),
        Place(MapCoord::new(8, 17)),
        Place(MapCoord::new(6, 17)),
        Place(MapCoord::new(4, 15)),
        Place(MapCoord::new(2, 17)),
        Place(MapCoord::new(0, 17)),
        Place(MapCoord::new(15, 20)),
        Place(MapCoord::new(17, 21)),
        Place(MapCoord::new(19, 21)),
        Place(MapCoord::new(21, 20)),
        Place(MapCoord::new(23, 19)),
        Place(MapCoord::new(23, 17)),
        Place(MapCoord::new(23, 15)),
        Place(MapCoord::new(21, 14)),
        Place(MapCoord::new(19, 13)),
        Place(MapCoord::new(17, 13)),
        Place(MapCoord::new(15, 14)),
        Place(MapCoord::new(12, 19)),
        Place(MapCoord::new(12, 21)),
        Place(MapCoord::new(18, 11)),
        Place(MapCoord::new(18, 9)),
        Place(MapCoord::new(18, 7)),
        Place(MapCoord::new(19, 5)),
        Place(MapCoord::new(18, 3)),
        Place(MapCoord::new(18, 1)),
        Place(MapCoord::new(20, 0)),
        Place(MapCoord::new(14, 23)),
        Place(MapCoord::new(16, 24)),
        Place(MapCoord::new(18, 24)),
        Place(MapCoord::new(20, 24)),
        Place(MapCoord::new(22, 23)),
        Place(MapCoord::new(24, 22)),
        Place(MapCoord::new(26, 20)),
        Place(MapCoord::new(26, 18)),
        Place(MapCoord::new(26, 16)),
        Place(MapCoord::new(26, 14)),
        Place(MapCoord::new(24, 12)),
        Place(MapCoord::new(22, 11)),
        Place(MapCoord::new(12, 15)),
        Place(MapCoord::new(12, 13)),
        Place(MapCoord::new(14, 11)),
        Place(MapCoord::new(3, 19)),
        Place(MapCoord::new(5, 20)),
        Place(MapCoord::new(7, 20)),
        Place(MapCoord::new(9, 20)),
        Place(MapCoord::new(9, 22)),
        Place(MapCoord::new(11, 24)),
        Place(MapCoord::new(13, 26)),
        Place(MapCoord::new(15, 27)),
        Place(MapCoord::new(16, 29)),
        Place(MapCoord::new(16, 31)),
        Place(MapCoord::new(18, 32)),
        Place(MapCoord::new(19, 30)),
        Place(MapCoord::new(19, 28)),
        Place(MapCoord::new(21, 27)),
        Place(MapCoord::new(23, 26)),
        Place(MapCoord::new(25, 25)),
        Place(MapCoord::new(27, 23)),
        Place(MapCoord::new(29, 21)),
        Place(MapCoord::new(29, 19)),
        Place(MapCoord::new(29, 17)),
        Place(MapCoord::new(29, 15)),
        Place(MapCoord::new(31, 15)),
        Place(MapCoord::new(32, 17)),
        Remove(MapCoord::new(0, 17)),
        Place(MapCoord::new(34, 17)),
    ]
}
