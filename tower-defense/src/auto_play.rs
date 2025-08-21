use crate::{
    MapCoord,
    game_state::{
        GameState, fast_forward::FastForwardMultiplier, flow::GameFlow, mutate_game_state,
        place_tower,
    },
};

pub fn auto_play() {
    mutate_game_state(|game_state| {
        game_state.fast_forward_multiplier = FastForwardMultiplier::X16;
        match &game_state.flow {
            GameFlow::Initializing => {}
            GameFlow::SelectingTower => {
                let using_cards = {
                    let selected_cards = game_state.hand.selected_cards();
                    if !selected_cards.is_empty() {
                        selected_cards
                    } else {
                        game_state.hand.all_cards()
                    }
                };
                game_state.goto_placing_tower(
                    crate::tower_selecting_hand::get_highest_tower::get_highest_tower_template(
                        &using_cards,
                        &game_state.upgrade_state,
                        game_state.rerolled_count,
                    ),
                );
            }
            GameFlow::PlacingTower => {
                let tower_slot_id = game_state.hand.get_first_tower_slot_id().unwrap();
                let tower_template = game_state
                    .hand
                    .get_tower_template_by_id(tower_slot_id)
                    .unwrap();
                let is_barricade =
                    tower_template.kind == crate::game_state::tower::TowerKind::Barricade;

                let target_position = if is_barricade {
                    find_corner_position(game_state)
                } else {
                    find_route_nearby_position(game_state)
                }
                .unwrap();

                let can_place = crate::game_state::can_place_tower::can_place_tower(
                    target_position,
                    namui::Wh::new(2, 2), // 타워 크기
                    &crate::game_state::TRAVEL_POINTS,
                    &game_state.towers.coords(),
                    game_state.route.iter_coords(),
                    crate::game_state::MAP_SIZE,
                );

                if can_place {
                    let tower = crate::game_state::tower::Tower::new(
                        tower_template,
                        target_position,
                        game_state.now(),
                    );
                    place_tower(tower, tower_slot_id);
                }
            }
            GameFlow::Defense => {}
            GameFlow::SelectingUpgrade { upgrades } => {
                game_state.upgrade(upgrades[0]);
                game_state.goto_selecting_tower();
            }
            GameFlow::Result => {}
        }
    });
}

fn find_corner_position(game_state: &GameState) -> Option<MapCoord> {
    let map_size = crate::game_state::MAP_SIZE;
    let placed_tower_coords = game_state.towers.coords();

    for y in 0..map_size.height - 1 {
        for x in 0..map_size.width - 1 {
            let position = MapCoord::new(x, y);
            if crate::game_state::can_place_tower::can_place_tower(
                position,
                namui::Wh::new(2, 2),
                &crate::game_state::TRAVEL_POINTS,
                &placed_tower_coords,
                game_state.route.iter_coords(),
                map_size,
            ) {
                return Some(position);
            }
        }
    }
    None
}

fn find_route_nearby_position(game_state: &GameState) -> Option<MapCoord> {
    let map_size = crate::game_state::MAP_SIZE;
    let placed_tower_coords = game_state.towers.coords();
    let route_coords = game_state.route.iter_coords();

    let mut best_position = None;
    let mut min_distance = f32::INFINITY;

    for y in 0..map_size.height - 1 {
        for x in 0..map_size.width - 1 {
            let position = MapCoord::new(x, y);
            if crate::game_state::can_place_tower::can_place_tower(
                position,
                namui::Wh::new(2, 2),
                &crate::game_state::TRAVEL_POINTS,
                &placed_tower_coords,
                route_coords,
                map_size,
            ) {
                let center_x = x as f32 + 1.0;
                let center_y = y as f32 + 1.0;

                let distance_to_route = route_coords
                    .iter()
                    .map(|route_coord| {
                        let dx = center_x - route_coord.x as f32;
                        let dy = center_y - route_coord.y as f32;
                        (dx * dx + dy * dy).sqrt()
                    })
                    .fold(f32::INFINITY, f32::min);

                if distance_to_route < min_distance {
                    min_distance = distance_to_route;
                    best_position = Some(position);
                }
            }
        }
    }

    best_position
}
