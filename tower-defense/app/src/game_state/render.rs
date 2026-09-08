use super::*;
use crate::game_state::tick::scheduler::RenderFrame;
use crate::headed_game::HeadedGame;

pub struct RenderGameState<'a> {
    pub(crate) game_state: &'a HeadedGame,
    pub(crate) presentation_instant: crate::PresentationInstant,
}

impl Component for RenderGameState<'_> {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(tick::Ticker {
            presentation_instant: self.presentation_instant,
        });

        let camera = self.game_state.camera();
        let visual_left_top = camera.visual_left_top();
        let final_offset = TILE_PX_SIZE.to_xy() * visual_left_top * -1.0;
        let render_frame = self.game_state.render_frame();

        ctx.scale(Xy::single(camera.zoom_level))
            .translate(final_offset)
            .compose(|ctx| {
                ctx.add((render_tower_info_popup, self.game_state));
                ctx.add((render_cursor_preview, self.game_state));
                ctx.add((render_field_particles, self.game_state.state()));
                ctx.add(RenderProjectiles {
                    game_state: self.game_state,
                    camera,
                    frame: render_frame,
                });
                ctx.add(RenderMonsters {
                    game_state: self.game_state,
                    camera,
                    frame: render_frame,
                });
                ctx.add((render_towers, self.game_state));
                ctx.add((render_bases, self.game_state));
                ctx.add(render_route_flag);
                ctx.add(render_route_guide);
                ctx.add((render_grid, self.game_state.state()));
                ctx.add((render_map_border_gradient, self.game_state.state()));
                ctx.add((render_decorations, self.game_state));
                ctx.add((render_backgrounds, self.game_state));
            });
    }
}

#[derive(Clone, Copy)]
struct RenderMonsters<'a> {
    game_state: &'a GameState,
    camera: &'a crate::game_state::Camera,
    frame: Option<RenderFrame<'a>>,
}

impl Component for RenderMonsters<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Some(frame) = self.frame else {
            render_monsters(ctx, self.game_state, self.camera);
            return;
        };
        let Some(current_snapshot) = frame.current_snapshot() else {
            render_monsters(ctx, self.game_state, self.camera);
            return;
        };

        for snapshot in current_snapshot.monsters() {
            let Some(sample) = frame.sample_monster(snapshot.id, true) else {
                continue;
            };
            ctx.translate(TILE_PX_SIZE.to_xy() * sample.position).add(
                crate::game_state::monster::RenderMonsterPose {
                    kind: sample.current.kind,
                    hp: sample.current.hp,
                    max_hp: sample.current.max_hp,
                    rotation: sample.rotation,
                    y_offset: sample.y_offset,
                },
            );
        }
    }
}

#[derive(Clone, Copy)]
struct RenderProjectiles<'a> {
    game_state: &'a GameState,
    camera: &'a crate::game_state::Camera,
    frame: Option<RenderFrame<'a>>,
}

impl Component for RenderProjectiles<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Some(frame) = self.frame else {
            render_projectiles(ctx, self.game_state, self.camera);
            return;
        };
        let Some(current_snapshot) = frame.current_snapshot() else {
            render_projectiles(ctx, self.game_state, self.camera);
            return;
        };

        for snapshot in current_snapshot.spatial_projectiles() {
            let Some(sample) = frame.sample_projectile(snapshot.id, true) else {
                continue;
            };
            ctx.translate(TILE_PX_SIZE.to_xy() * sample.position).add(
                crate::game_state::attack::RenderProjectileSnapshot {
                    projectile_kind: sample.current.projectile_kind,
                    direction: sample.direction,
                },
            );
        }
    }
}

fn render_stuffs<'a, C, MapCoord, MapAxis>(
    camera: &crate::game_state::Camera,
    ctx: &ComposeCtx,
    stuffs: impl Iterator<Item = (MapCoord, C)>,
) where
    C: 'a + Component,
    MapCoord: AsRef<Xy<MapAxis>>,
    MapAxis: Ratio + std::fmt::Debug + Clone + Copy,
{
    let visual_left_top = camera.visual_left_top();
    let screen_rect = Rect::from_xy_wh(visual_left_top, {
        let screen_size = namui::screen::size();
        Wh::new(
            screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
            screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
        ) / camera.zoom_level
    });

    for (xy, stuff) in stuffs {
        let xy = *xy.as_ref();
        if screen_rect.right() < xy.x.as_f32() || screen_rect.bottom() < xy.y.as_f32() {
            continue;
        }

        let px_xy = TILE_PX_SIZE.to_xy() * xy.map(|t| t.as_f32());
        ctx.translate(px_xy).compose(move |ctx| {
            let rendering_tree = ctx.ghost_add("", stuff);
            let Some(bounding_box) = rendering_tree.bounding_box() else {
                return;
            };

            let local_right = bounding_box.right() / TILE_PX_SIZE.width;
            let local_bottom = bounding_box.bottom() / TILE_PX_SIZE.height;

            if xy.x.as_f32() + local_right < screen_rect.left()
                || xy.y.as_f32() + local_bottom < screen_rect.top()
            {
                return;
            }

            ctx.add(rendering_tree);
        });
    }
}

fn render_grid(ctx: &RenderCtx, _game_state: &GameState) {
    let mut path = Path::new();
    for x in 0..MAP_SIZE.width + 1 {
        let x = (x.as_f32() * TILE_PX_SIZE.width.as_f32()).px();
        path = path.move_to(x, 0.px());
        path = path.line_to(
            x,
            (MAP_SIZE.height.as_f32() * TILE_PX_SIZE.height.as_f32()).px(),
        );
    }
    for y in 0..MAP_SIZE.height + 1 {
        let y = (y.as_f32() * TILE_PX_SIZE.height.as_f32()).px();
        path = path.move_to(0.px(), y);
        path = path.line_to(
            (MAP_SIZE.width.as_f32() * TILE_PX_SIZE.width.as_f32()).px(),
            y,
        );
    }
    let paint = Paint::new(Color::grayscale_alpha_f01(1.0, 0.5))
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.px())
        .set_stroke_cap(StrokeCap::Round);

    ctx.add(namui::path(path, paint));
}

fn render_backgrounds(ctx: &RenderCtx, game_state: &HeadedGame) {
    let camera = game_state.camera();
    let visual_left_top = camera.visual_left_top();
    let screen_rect = Rect::from_xy_wh(visual_left_top, {
        let screen_size = namui::screen::size();
        Wh::new(
            screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
            screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
        ) / camera.zoom_level
    });

    for background in game_state.backgrounds().iter() {
        let xy = background.coord;

        if screen_rect.right() < xy.x || screen_rect.bottom() < xy.y {
            continue;
        }

        let px_xy = Xy::new(
            px(xy.x * TILE_PX_SIZE.width.as_f32()),
            px(xy.y * TILE_PX_SIZE.height.as_f32()),
        );

        ctx.translate(px_xy).compose({
            let background = *background;
            move |ctx| {
                let rendering_tree = ctx.ghost_add("", &background);
                let Some(bounding_box) = rendering_tree.bounding_box() else {
                    return;
                };

                let local_right = bounding_box.right() / TILE_PX_SIZE.width;
                let local_bottom = bounding_box.bottom() / TILE_PX_SIZE.height;

                if xy.x + local_right < screen_rect.left()
                    || xy.y + local_bottom < screen_rect.top()
                {
                    return;
                }

                ctx.add(rendering_tree);

                ctx.add(rect(RectParam {
                    rect: bounding_box,
                    style: RectStyle {
                        fill: Some(RectFill {
                            color: Color::TRANSPARENT,
                        }),
                        ..Default::default()
                    },
                }))
                .attach_event(|event| {
                    if let Event::MouseDown { event } = event {
                        if event.button != Some(MouseButton::Left) {
                            return;
                        }
                        if !event.is_local_xy_in() {
                            return;
                        }

                        crate::game_state::mutate_headed_game(|game_state| {
                            game_state
                                .set_selected_tower(None, crate::PresentationInstant::capture());
                        });
                    }
                });
            }
        });
    }
}

fn render_projectiles(ctx: &RenderCtx, game_state: &GameState, camera: &crate::game_state::Camera) {
    let snapshot =
        crate::game_state::render_snapshot::WorldRenderSnapshot::capture_with_base_scales(
            game_state,
            (Xy::single(1.0), Xy::single(1.0)),
        );
    render_stuffs(
        camera,
        ctx,
        snapshot.spatial_projectiles().iter().map(|projectile| {
            (
                projectile.position.as_map_coord_f32(),
                crate::game_state::attack::RenderProjectileSnapshot {
                    projectile_kind: projectile.projectile_kind,
                    direction: Xy::new(
                        projectile.direction.x as f32,
                        projectile.direction.y as f32,
                    ),
                },
            )
        }),
    );
}

fn render_towers(ctx: &RenderCtx, game_state: &HeadedGame) {
    let state = game_state.state();
    let camera = game_state.camera();
    let ui_state = game_state.ui_state();
    let selected_tower_id = ui_state.selected_tower_id;
    let visual_left_top = camera.visual_left_top();
    let screen_rect = {
        let screen_size = namui::screen::size();
        Rect::from_xy_wh(visual_left_top, {
            Wh::new(
                screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
                screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
            ) / camera.zoom_level
        })
    };

    let render_frame = game_state.render_frame();
    let sim_render_time = render_frame.map(|frame| frame.time).unwrap_or_else(|| {
        crate::SimRenderTime::new(game_state.sim_tick(), crate::InterpolationAlpha::ZERO)
    });

    let tower_snapshots = render_frame
        .as_ref()
        .and_then(|frame| frame.current_snapshot())
        .map(crate::game_state::render_snapshot::WorldRenderSnapshot::towers)
        .unwrap_or(&[]);

    for snapshot in tower_snapshots {
        let tower_id = snapshot.id;
        let Some(tower) = state.presentation_tower(tower_id) else {
            continue;
        };
        let Some(tower_metadata) = state.presentation_metadata.tower(tower_id) else {
            continue;
        };
        let tower_xy = Xy::new(snapshot.left_top[0] as f32, snapshot.left_top[1] as f32);

        // Culling check
        if (screen_rect.right() < tower_xy.x || screen_rect.bottom() < tower_xy.y)
            && tower_metadata.royal_straight_flush_visual.is_none()
        {
            continue;
        }

        let px_xy = TILE_PX_SIZE.to_xy() * tower_xy;
        let y_ratio_offset = render_frame
            .and_then(|frame| frame.sample_tower(snapshot.id, true))
            .unwrap_or(snapshot.y_ratio_offset);
        let attack_range_radius = snapshot.attack_range_radius;
        let on_attack_splash_radii = snapshot.on_attack_splash_radii.clone();
        ctx.translate(px_xy).compose(move |ctx| {
            if selected_tower_id == Some(tower_id) {
                ctx.add(crate::game_state::tower::render::TowerAttackRange {
                    range_radius: attack_range_radius,
                    splash_radii: on_attack_splash_radii.clone(),
                });
            }

            ctx.mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
                .add(
                    crate::game_state::tower::render::RenderTower {
                        tower: &tower,
                        sim_render_time,
                        animation_kind: tower_metadata.animation_kind,
                        royal_straight_flush_visual: tower_metadata
                            .royal_straight_flush_visual
                            .as_ref(),
                        y_ratio_offset,
                    }
                    .attach_event({
                        move |event| {
                            let Event::MouseDown { event } = event else {
                                return;
                            };
                            if event.button != Some(MouseButton::Left) {
                                return;
                            }
                            if !event.is_local_xy_in() {
                                return;
                            }
                            event.stop_propagation();
                            crate::game_state::mutate_headed_game(move |game_state| {
                                let next_selected = if selected_tower_id == Some(tower_id) {
                                    None
                                } else {
                                    Some(tower_id)
                                };
                                game_state.set_selected_tower(
                                    next_selected,
                                    crate::PresentationInstant::capture(),
                                );
                            });
                        }
                    }),
                );
        });
    }
}

fn render_tower_info_popup(ctx: &RenderCtx, game_state: &HeadedGame) {
    use crate::game_state::tower_info_popup::TowerInfoPopup;
    let state = game_state.state();
    let camera = game_state.camera();
    let ui_state = game_state.ui_state();
    let raw_snapshot = state.raw_render_snapshot();

    for snapshot in raw_snapshot.towers.iter() {
        if let Some(popup_state) = ui_state.get_popup_state(crate::TowerId::from_raw(snapshot.id))
            && popup_state.is_visible()
        {
            let popup_scale = popup_state.scale;
            let popup_opacity = popup_state.opacity;

            if popup_scale > 0.01 && popup_opacity > 0.01 {
                let Some(tower) = state
                    .raw_core
                    .towers()
                    .iter()
                    .find(|tower| tower.id == Some(snapshot.id))
                else {
                    continue;
                };
                let px_xy = TILE_PX_SIZE.to_xy()
                    * Xy::new(snapshot.left_top[0] as f32, snapshot.left_top[1] as f32)
                    + Xy::new(TILE_PX_SIZE.width, 0.px());

                ctx.translate(px_xy)
                    .scale(Xy::single(popup_scale / camera.zoom_level))
                    .add(TowerInfoPopup { tower });
            }
        }
    }
}

fn render_monsters(ctx: &RenderCtx, game_state: &GameState, camera: &crate::game_state::Camera) {
    let snapshot =
        crate::game_state::render_snapshot::WorldRenderSnapshot::capture_with_base_scales(
            game_state,
            (Xy::single(1.0), Xy::single(1.0)),
        );
    render_stuffs(
        camera,
        ctx,
        snapshot.monsters().iter().map(|monster| {
            (
                monster.position.as_map_coord_f32(),
                crate::game_state::monster::RenderMonsterPose {
                    kind: monster.kind,
                    hp: monster.hp,
                    max_hp: monster.max_hp,
                    rotation: monster.rotation,
                    y_offset: monster.y_offset,
                },
            )
        }),
    );
}

fn render_cursor_preview(ctx: &RenderCtx, game_state: &HeadedGame) {
    let cursor_preview = game_state.cursor_preview();
    ctx.add(cursor_preview.render());

    // Render tower preview if in PlacingTower flow and hand has selected tower
    if matches!(
        game_state.raw_core_state().flow(),
        td_core::GameFlowState::PlacingTower
    ) {
        let hand = game_state.state().presentation_hand_snapshot();
        let selected_slot_ids = hand.selected_slot_ids();
        if let Some(&selected_slot_id) = selected_slot_ids.first()
            && let Some(tower_template) = hand
                .get_item(selected_slot_id)
                .and_then(|item| item.as_tower())
        {
            let Some(placing_tower_slot_index) = hand
                .active_slot_ids()
                .iter()
                .position(|slot_id| *slot_id == selected_slot_id)
            else {
                return;
            };
            ctx.add(
                crate::game_state::cursor_preview::tower::TowerCursorPreview {
                    tower_template,
                    map_coord: cursor_preview.map_coord,
                    placing_tower_slot_index,
                },
            );
        }
    }
}

fn render_field_particles(ctx: &RenderCtx, _game_state: &GameState) {
    let attack = crate::asset::image::PARTICLE_ATTACK;
    let projectiles = crate::asset::image::PARTICLE_PROJECTILES;
    let monsters = crate::asset::image::PARTICLE_MONSTERS;
    let icons = crate::asset::image::PARTICLE_ICONS;
    let digits = crate::asset::image::PARTICLE_DIGITS;
    let dust = crate::asset::image::ui::particle::DUST;
    let screen_paint = Some(Paint::new(Color::WHITE).set_blend_mode(BlendMode::Screen));

    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::ATTACK_PARTICLES,
        image: attack,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: screen_paint.clone(),
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::MONSTER_CORPSES,
        image: monsters,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::PROJECTILES,
        image: projectiles,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::TRASHES,
        image: projectiles,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::MONSTER_SOULS,
        image: monsters,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::ICONS,
        image: icons,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::DAMAGE_TEXTS,
        image: digits,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::CARDS,
        image: projectiles,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::HEARTS,
        image: projectiles,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::BLACK_SMOKES,
        image: attack,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
    ctx.add(namui::particle::RenderEmitter {
        emitter: &field_particle::DUSTS,
        image: dust,
        sprite_colors_blend_mode: BlendMode::Modulate,
        paint: None,
    });
}

fn render_decorations(ctx: &RenderCtx, game_state: &HeadedGame) {
    ctx.add(background::decoration_rendering_tree(
        game_state.decorations(),
    ));
}

fn render_map_border_gradient(ctx: &RenderCtx, _game_state: &GameState) {
    let map_px_w = MAP_SIZE.width as f32 * TILE_PX_SIZE.width.as_f32();
    let map_px_h = MAP_SIZE.height as f32 * TILE_PX_SIZE.height.as_f32();
    let grad_px = MAP_OUTSIDE_MARGIN_TILES * TILE_PX_SIZE.width.as_f32(); // 4 tiles of gradient
    let dark = Color::from_u8(0, 0, 0, 96);
    let transparent = Color::from_u8(0, 0, 0, 0);

    // Left strip: trapezoid
    ctx.add(namui::path(
        Path::new()
            .move_to(px(-grad_px), px(-grad_px))
            .line_to(px(0.0), px(0.0))
            .line_to(px(0.0), px(map_px_h))
            .line_to(px(-grad_px), px(map_px_h + grad_px))
            .close(),
        Paint::new(Color::WHITE).set_shader(Shader::LinearGradient {
            start_xy: Xy::new(px(0.0), px(0.0)),
            end_xy: Xy::new(px(-grad_px), px(0.0)),
            colors: vec![transparent, dark],
            tile_mode: TileMode::Clamp,
        }),
    ));

    // Right strip: trapezoid
    ctx.add(namui::path(
        Path::new()
            .move_to(px(map_px_w), px(0.0))
            .line_to(px(map_px_w + grad_px), px(-grad_px))
            .line_to(px(map_px_w + grad_px), px(map_px_h + grad_px))
            .line_to(px(map_px_w), px(map_px_h))
            .close(),
        Paint::new(Color::WHITE).set_shader(Shader::LinearGradient {
            start_xy: Xy::new(px(map_px_w), px(0.0)),
            end_xy: Xy::new(px(map_px_w + grad_px), px(0.0)),
            colors: vec![transparent, dark],
            tile_mode: TileMode::Clamp,
        }),
    ));

    // Top strip: trapezoid
    ctx.add(namui::path(
        Path::new()
            .move_to(px(-grad_px), px(-grad_px))
            .line_to(px(0.0), px(0.0))
            .line_to(px(map_px_w), px(0.0))
            .line_to(px(map_px_w + grad_px), px(-grad_px))
            .close(),
        Paint::new(Color::WHITE).set_shader(Shader::LinearGradient {
            start_xy: Xy::new(px(0.0), px(0.0)),
            end_xy: Xy::new(px(0.0), px(-grad_px)),
            colors: vec![transparent, dark],
            tile_mode: TileMode::Clamp,
        }),
    ));

    // Bottom strip: trapezoid
    ctx.add(namui::path(
        Path::new()
            .move_to(px(0.0), px(map_px_h))
            .line_to(px(map_px_w), px(map_px_h))
            .line_to(px(map_px_w + grad_px), px(map_px_h + grad_px))
            .line_to(px(-grad_px), px(map_px_h + grad_px))
            .close(),
        Paint::new(Color::WHITE).set_shader(Shader::LinearGradient {
            start_xy: Xy::new(px(0.0), px(map_px_h)),
            end_xy: Xy::new(px(0.0), px(map_px_h + grad_px)),
            colors: vec![transparent, dark],
            tile_mode: TileMode::Clamp,
        }),
    ));
}
