use super::PreviewKind;
use crate::{
    game_state::{
        can_place_tower::can_place_tower,
        flow::GameFlow,
        mutate_game_state, place_tower,
        tower::{tower_image_resource_location, AnimationKind, Tower, TowerTemplate},
        use_game_state, MAP_SIZE, TILE_PX_SIZE, TRAVEL_POINTS,
    },
    palette,
    tower_placing_hand::PlacingTowerSlot,
    MapCoordF32,
};
use namui::*;
use std::ops::Deref;

pub(super) struct TowerCursorPreview<'a> {
    pub tower_template: &'a TowerTemplate,
    pub map_coord: MapCoordF32,
    pub placing_tower_slot_index: usize,
}
impl Component for TowerCursorPreview<'_> {
    fn render(self, ctx: &namui::RenderCtx) {
        let Self {
            tower_template,
            map_coord,
            placing_tower_slot_index,
        } = self;

        let game_state = use_game_state(ctx);

        let rounded_center_xy = ctx.track_eq(&map_coord.map(|f| f.round() as usize));
        let placed_tower_coords = &game_state.towers.coords();
        let route_coords = &game_state.route.iter_coords();

        let can_place_tower = ctx.memo(|| {
            let out_of_map = rounded_center_xy.x < 1
                || rounded_center_xy.y < 1
                || rounded_center_xy.x >= MAP_SIZE.width - 1
                || rounded_center_xy.y >= MAP_SIZE.height - 1;

            if out_of_map {
                return false;
            }
            can_place_tower(
                *rounded_center_xy - Xy::single(1),
                Wh::single(2),
                &TRAVEL_POINTS,
                &placed_tower_coords,
                &route_coords,
                MAP_SIZE,
            )
        });

        let cancel_placing_tower_selection = || {
            mutate_game_state(|game_state| {
                game_state.cursor_preview.kind = PreviewKind::None;
            });
        };

        let place_tower = || {
            let left_top = *rounded_center_xy - Xy::single(1);
            place_tower(Tower::new(tower_template, left_top));
            mutate_game_state(move |game_state| {
                let GameFlow::PlacingTower {
                    placing_tower_slots,
                } = &mut game_state.flow
                else {
                    panic!("Expected GameFlow::PlacingTower");
                };
                placing_tower_slots[placing_tower_slot_index] = PlacingTowerSlot::Empty;
                game_state.cursor_preview.kind = PreviewKind::None;
            });
        };

        let ctx = ctx.translate(TILE_PX_SIZE.as_xy() * *rounded_center_xy);

        ctx.add(TowerImage { tower_template });
        ctx.add(TowerAttackRange { tower_template });
        // TODO: Add TowerSkillRange
        // ctx.add(TowerSkillRange { tower_template });
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
            Event::KeyDown { event } => match event.code {
                Code::Escape => {
                    cancel_placing_tower_selection();
                    event.stop_propagation();
                }
                _ => {}
            },
            _ => {}
        });
    }
}

struct TowerImage<'a> {
    tower_template: &'a TowerTemplate,
}
impl Component for TowerImage<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { tower_template } = self;

        let tower_image = ctx.image(tower_image_resource_location(
            tower_template.kind,
            AnimationKind::Idle1,
        ));

        let Some(Ok(tower_image)) = tower_image.deref() else {
            return;
        };

        let image_wh = tower_image.info.wh();
        let rect = Rect::from_xy_wh(image_wh.as_xy() / -2.0, image_wh);
        let paint = Paint::new(Color::grayscale_alpha_f01(0.0, 0.5));
        ctx.add(namui::image(ImageParam {
            rect,
            image: tower_image.clone(),
            style: ImageStyle {
                fit: ImageFit::None,
                paint: Some(paint),
            },
        }));
    }
}

struct TowerAttackRange<'a> {
    tower_template: &'a TowerTemplate,
}
impl Component for TowerAttackRange<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { tower_template } = self;

        let range_radius_px = TILE_PX_SIZE.width * tower_template.default_attack_range_radius;
        let paint = Paint::new(palette::OUTLINE)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(2.px());
        let path = Path::new().add_oval(Rect::Ltrb {
            left: -range_radius_px,
            top: -range_radius_px,
            right: range_radius_px,
            bottom: range_radius_px,
        });
        ctx.add(namui::path(path, paint));
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
            rect: Rect::from_xy_wh(Xy::single(-TILE_PX_SIZE.width), TILE_PX_SIZE * 2.0),
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
