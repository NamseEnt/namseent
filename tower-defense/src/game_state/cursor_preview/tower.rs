use crate::game_state::tower::render::{TowerAttackRange, TowerImage, TowerSpriteWithOverlay};
use crate::{
    MapCoordF32,
    game_state::{
        MAP_SIZE, TILE_PX_SIZE, TRAVEL_POINTS,
        action::GameStateAction,
        can_place_tower::can_place_tower,
        flow::GameFlow,
        hand::HandSlotId,
        mutate_game_state,
        tower::{AnimationKind, TowerTemplate, Tower},
        use_game_state,
    },
    palette,
};
use namui::*;

pub struct TowerCursorPreview<'a> {
    pub tower_template: &'a TowerTemplate,
    pub map_coord: MapCoordF32,
    pub placing_tower_slot_id: HandSlotId,
}
impl Component for TowerCursorPreview<'_> {
    fn render(self, ctx: &namui::RenderCtx) {
        let Self {
            tower_template,
            map_coord,
            placing_tower_slot_id,
        } = self;

        let game_state = use_game_state(ctx);

        let tower_template = tower_template.clone();
        let tower_template_for_placement = tower_template.clone();
        let rounded_left_top_xy =
            ctx.track_eq(&map_coord.map(|f| (f.round() as usize).saturating_sub(1)));
        let placed_tower_coords = ctx.track_eq(&game_state.towers.coords());
        let route_coords = &game_state.route.iter_coords();

        let can_place_tower = ctx.memo(|| {
            let out_of_map = rounded_left_top_xy.x < 1
                || rounded_left_top_xy.y < 1
                || rounded_left_top_xy.x >= MAP_SIZE.width
                || rounded_left_top_xy.y >= MAP_SIZE.height;

            if out_of_map {
                return false;
            }
            can_place_tower(
                *rounded_left_top_xy,
                Wh::single(2),
                &TRAVEL_POINTS,
                &placed_tower_coords,
                route_coords,
                MAP_SIZE,
            )
        });

        let cancel_placing_tower_selection = move || {
            mutate_game_state(move |game_state| {
                if !matches!(game_state.flow, GameFlow::PlacingTower) {
                    unreachable!()
                }
                game_state.hand.deselect_slot(placing_tower_slot_id);
            });
        };

        let left_top = *rounded_left_top_xy;
        let place_tower = || {
            let tower_template_for_placement = tower_template_for_placement.clone();
            mutate_game_state(move |game_state| {
                game_state.action(GameStateAction::PlaceTower(
                    Box::new(Tower::new(
                        &tower_template_for_placement,
                        left_top,
                        game_state.now(),
                    )),
                    Some(placing_tower_slot_id),
                ));
            });
        };

        let tower_image = (tower_template.kind, AnimationKind::Idle1).image();

        let ctx = ctx.translate(TILE_PX_SIZE.to_xy() * left_top);

        ctx.add(TowerSpriteWithOverlay {
            image: tower_image,
            wh: tower_image.info().wh(),
            suit: tower_template.suit,
            rank: tower_template.rank,
            alpha: 0.5,
        });
        ctx.add(TowerAttackRange {
            tower_template: &tower_template,
        });
        // TODO: Add TowerSkillRange
        // ctx.add(TowerSkillRange { tower_template: &tower_template });
        ctx.add(TowerArea {
            can_place_tower: *can_place_tower,
        });

        ctx.attach_event(|event| match event {
            Event::MouseDown { event } => match event.button {
                Some(MouseButton::Left) => {
                    if *can_place_tower {
                        place_tower();
                    }
                    event.stop_propagation();
                }
                Some(MouseButton::Right) => {
                    cancel_placing_tower_selection();
                    event.stop_propagation();
                }
                _ => {}
            },
            Event::KeyDown { event } if event.code == Code::Escape => {
                cancel_placing_tower_selection();
                event.stop_propagation();
            }
            _ => {}
        });
    }
}

struct TowerArea {
    can_place_tower: bool,
}
impl Component for TowerArea {
    fn render(self, ctx: &RenderCtx) {
        let Self { can_place_tower } = self;

        let color = match can_place_tower {
            true => palette::PRIMARY,
            false => palette::SURFACE_CONTAINER_HIGH,
        }
        .with_alpha(127);
        ctx.add(rect(RectParam {
            rect: Rect::from_xy_wh(Xy::zero(), TILE_PX_SIZE * 2.0),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color,
                    width: 2.px(),
                    border_position: BorderPosition::Inside,
                }),
                fill: Some(RectFill { color }),
                round: None,
            },
        }));
    }
}
