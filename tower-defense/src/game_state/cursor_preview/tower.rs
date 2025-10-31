use crate::{
    MapCoordF32,
    game_state::{
        MAP_SIZE, TILE_PX_SIZE, TRAVEL_POINTS,
        can_place_tower::can_place_tower,
        flow::GameFlow,
        hand::HandSlotId,
        mutate_game_state, place_tower,
        tower::{AnimationKind, Tower, TowerTemplate, render::TowerImage as TowerImageTrait},
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

        let rounded_center_xy = ctx.track_eq(&map_coord.map(|f| f.round() as usize));
        let placed_tower_coords = ctx.track_eq(&game_state.towers.coords());
        let route_coords = &game_state.route.iter_coords();

        let can_place_tower = ctx.memo(|| {
            let out_of_map = rounded_center_xy.x < 1
                || rounded_center_xy.y < 1
                || rounded_center_xy.x >= MAP_SIZE.width
                || rounded_center_xy.y >= MAP_SIZE.height;

            if out_of_map {
                return false;
            }
            can_place_tower(
                *rounded_center_xy - Xy::single(1),
                Wh::single(2),
                &TRAVEL_POINTS,
                &placed_tower_coords,
                route_coords,
                MAP_SIZE,
            )
        });

        let cancel_placing_tower_selection = move || {
            mutate_game_state(move |game_state| {
                let GameFlow::PlacingTower { hand } = &mut game_state.flow else {
                    unreachable!()
                };
                hand.deselect_slot(placing_tower_slot_id);
            });
        };

        let place_tower = || {
            let left_top = *rounded_center_xy - Xy::single(1);
            place_tower(
                Tower::new(tower_template, left_top, game_state.now()),
                placing_tower_slot_id,
            );
        };

        let ctx = ctx.translate(TILE_PX_SIZE.to_xy() * *rounded_center_xy);

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
            Event::KeyDown { event } => {
                if event.code == Code::Escape {
                    cancel_placing_tower_selection();
                    event.stop_propagation();
                }
            }
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

        let tower_image = (tower_template.kind, AnimationKind::Idle1).image();

        let image_wh = tower_image.info().wh();
        let rect = Rect::from_xy_wh(image_wh.to_xy() / -2.0, image_wh);
        let paint = Paint::new(Color::grayscale_alpha_f01(0.0, 0.5));
        ctx.add(namui::image(ImageParam {
            rect,
            image: tower_image,
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
