//! Tower placement strategies.

use super::TowerPlacementStrategy;
use crate::MapCoord;
use crate::game_state::can_place_tower::can_place_tower;
use crate::game_state::flow::GameFlow;
use crate::game_state::tower::{Tower, TowerKind};
use crate::game_state::{GameState, MAP_SIZE, TRAVEL_POINTS};
use crate::hand::HandSlotId;
use namui::*;

/// Heuristic placement strategy that uses the spiral plan and replaces central barricades with remaining towers.
pub struct HeuristicPlacementStrategy;

impl TowerPlacementStrategy for HeuristicPlacementStrategy {
    fn name(&self) -> &str {
        "heuristic_placement"
    }

    fn execute_placement(&self, game_state: &mut GameState) {
        while matches!(game_state.flow, GameFlow::PlacingTower) {
            let (slot_id, template) = match self.current_tower_template(game_state) {
                Some(value) => value,
                None => break,
            };

            let now = game_state.now();
            let mut placed = false;

            for step in placement_plan() {
                match step {
                    PlanStep::Remove(coord) => {
                        if let Some(tower_id) = game_state.towers.find_by_xy(coord).map(|t| t.id())
                        {
                            game_state.remove_tower(tower_id);
                        }
                    }
                    PlanStep::Place(left_top) => {
                        if left_top.x + 1 >= MAP_SIZE.width || left_top.y + 1 >= MAP_SIZE.height {
                            continue;
                        }
                        let placed_coords = game_state.towers.coords();
                        let route_coords: Vec<MapCoord> = game_state.route.iter_coords().to_vec();

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

                            if let Some(first_id) = game_state.hand.get_slot_id_by_index(0)
                                && game_state
                                    .hand
                                    .get_item(first_id)
                                    .and_then(|item| item.as_tower())
                                    .is_some()
                            {
                                game_state.hand.select_slot(first_id);
                            }

                            placed = true;
                            break;
                        }
                    }
                }
            }

            if !placed && self.replace_central_barricade(game_state, &template, slot_id, now) {
                placed = true;
            }

            if !placed {
                break;
            }

            if game_state.hand.is_empty() {
                game_state.goto_defense();
                break;
            }
        }

        if matches!(game_state.flow, GameFlow::PlacingTower) {
            game_state.goto_defense();
        }
    }
}

impl HeuristicPlacementStrategy {
    fn current_tower_template(
        &self,
        game_state: &mut GameState,
    ) -> Option<(HandSlotId, crate::game_state::tower::TowerTemplate)> {
        let Some(&slot_id) = game_state.hand.selected_slot_ids().first() else {
            if let Some(first_id) = game_state.hand.get_slot_id_by_index(0)
                && game_state
                    .hand
                    .get_item(first_id)
                    .and_then(|item| item.as_tower())
                    .is_some()
            {
                game_state.hand.select_slot(first_id);
                return self.current_tower_template(game_state);
            }
            return None;
        };

        let template = game_state
            .hand
            .get_item(slot_id)
            .and_then(|item| item.as_tower())
            .cloned()?;

        Some((slot_id, template))
    }

    fn replace_central_barricade(
        &self,
        game_state: &mut GameState,
        template: &crate::game_state::tower::TowerTemplate,
        slot_id: HandSlotId,
        now: namui::Instant,
    ) -> bool {
        let center = MapCoord::new(MAP_SIZE.width / 2, MAP_SIZE.height / 2);

        let mut barricades: Vec<(i32, usize, MapCoord)> = game_state
            .towers
            .iter()
            .filter(|tower| tower.kind == TowerKind::Barricade)
            .map(|tower| {
                let dx = tower.left_top.x as i32 - center.x as i32;
                let dy = tower.left_top.y as i32 - center.y as i32;
                ((dx * dx + dy * dy), tower.id(), tower.left_top)
            })
            .collect();

        barricades.sort_by_key(|(dist, _, _)| *dist);

        for (_, tower_id, left_top) in barricades {
            if !game_state.remove_tower(tower_id) {
                continue;
            }

            let placed_coords = game_state.towers.coords();
            let route_coords: Vec<MapCoord> = game_state.route.iter_coords().to_vec();
            if can_place_tower(
                left_top,
                Wh::new(2, 2),
                &TRAVEL_POINTS,
                &placed_coords,
                &route_coords,
                MAP_SIZE,
            ) {
                let tower = Tower::new(template, left_top, now);
                game_state.place_tower(tower);
                game_state.hand.delete_slots(&[slot_id]);

                if let Some(first_id) = game_state.hand.get_slot_id_by_index(0)
                    && game_state
                        .hand
                        .get_item(first_id)
                        .and_then(|item| item.as_tower())
                        .is_some()
                {
                    game_state.hand.select_slot(first_id);
                }

                return true;
            }
        }

        false
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
