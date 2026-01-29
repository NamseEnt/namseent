use crate::MapCoord;
use crate::game_state::{
    GameState, MAP_SIZE, TRAVEL_POINTS, can_place_tower::can_place_tower, flow::GameFlow,
    mutate_game_state, tower::Tower,
};
use crate::route::calculate_routes;
use crate::theme::button::{Button, ButtonVariant};
use crate::theme::typography::memoized_text;
use namui::*;

const BUTTON_HEIGHT: Px = px(40.);

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

pub struct PlaceSelectedTowerInSpiralButton {
    pub width: Px,
}

pub fn place_selected_tower_in_spiral(gs: &mut GameState) {
    let (slot_id, template) = {
        let GameFlow::PlacingTower { hand } = &mut gs.flow else {
            return;
        };

        let Some(&slot_id) = hand.selected_slot_ids().first() else {
            return;
        };
        let Some(template) = hand.get_item(slot_id).cloned() else {
            return;
        };
        (slot_id, template)
    };

    let now = gs.now();
    let mut placed_coords = gs.towers.coords();
    let mut route_coords: Vec<MapCoord> = gs.route.iter_coords().to_vec();
    let mut placed_at: Option<MapCoord> = None;

    for step in placement_plan() {
        match step {
            PlanStep::Remove(coord) => {
                if let Some(tower_id) = gs.towers.find_by_xy(coord).map(|tower| tower.id()) {
                    gs.towers.remove_tower(tower_id);
                    gs.route = calculate_routes(&gs.towers.coords(), &TRAVEL_POINTS, MAP_SIZE)
                        .expect("route should exist after removing a tower");
                    placed_coords = gs.towers.coords();
                    route_coords = gs.route.iter_coords().to_vec();
                    println!("[Spiral Place] Removed tower at ({}, {})", coord.x, coord.y);
                }
            }
            PlanStep::Place(left_top) => {
                if left_top.x + 1 >= MAP_SIZE.width || left_top.y + 1 >= MAP_SIZE.height {
                    continue;
                }

                if can_place_tower(
                    left_top,
                    Wh::new(2, 2),
                    &TRAVEL_POINTS,
                    &placed_coords,
                    &route_coords,
                    MAP_SIZE,
                ) {
                    let tower = Tower::new(&template, left_top, now);
                    gs.place_tower(tower);
                    placed_at = Some(left_top);
                    println!(
                        "[Spiral Place] Placed tower at ({}, {})",
                        left_top.x, left_top.y
                    );
                    break;
                }
            }
        }
    }

    if placed_at.is_some() {
        if let GameFlow::PlacingTower { hand } = &mut gs.flow {
            hand.delete_slots(&[slot_id]);

            if let Some(first_slot_id) = hand.get_slot_id_by_index(0)
                && hand.get_item(first_slot_id).is_some()
            {
                hand.select_slot(first_slot_id);
            }

            if hand.is_empty() {
                gs.goto_defense();
            }
        }
    } else {
        println!("[Spiral Place] No placement available for this plan iteration.");
    }
}

impl Component for PlaceSelectedTowerInSpiralButton {
    fn render(self, ctx: &RenderCtx) {
        let Self { width } = self;

        ctx.add(
            Button::new(
                Wh::new(width, BUTTON_HEIGHT),
                &|| {
                    mutate_game_state(place_selected_tower_in_spiral);
                },
                &|wh, text_color, ctx| {
                    ctx.add(memoized_text((&text_color, &wh), |builder| {
                        builder
                            .paragraph()
                            .color(text_color)
                            .text("Place selected tower in spiral")
                            .render_center(wh)
                    }));
                },
            )
            .variant(ButtonVariant::Contained),
        );
    }
}
