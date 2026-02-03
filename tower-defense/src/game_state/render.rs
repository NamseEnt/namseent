use super::*;

pub struct RenderGameState<'a> {
    pub game_state: &'a GameState,
}

impl Component for RenderGameState<'_> {
    fn render(self, ctx: &RenderCtx) {
        ctx.add(tick::Ticker);

        let visual_left_top = self.game_state.camera.visual_left_top();
        let final_offset = TILE_PX_SIZE.to_xy() * visual_left_top * -1.0;

        ctx.scale(Xy::single(self.game_state.camera.zoom_level))
            .translate(final_offset)
            .compose(|ctx| {
                ctx.add((render_tower_info_popup, self.game_state));
                ctx.add((render_cursor_preview, self.game_state));
                ctx.add((render_field_particles, self.game_state));
                ctx.add((render_laser_beams, self.game_state));
                ctx.add((render_tower_emit_effects, self.game_state));
                ctx.add((render_target_hit_effects, self.game_state));
                ctx.add((render_projectiles, self.game_state));
                ctx.add((render_monsters, self.game_state));
                ctx.add((render_route_guide, self.game_state));
                ctx.add((render_towers, self.game_state));
                ctx.add((render_grid, self.game_state));
                ctx.add((render_backgrounds, self.game_state));
            });
    }
}

impl GameState {
    fn render_stuffs<'a, C, MapCoord, MapAxis>(
        &self,
        ctx: &ComposeCtx,
        stuffs: impl Iterator<Item = (MapCoord, C)>,
    ) where
        C: 'a + Component,
        MapCoord: AsRef<Xy<MapAxis>>,
        MapAxis: Ratio + std::fmt::Debug + Clone + Copy,
    {
        let visual_left_top = self.camera.visual_left_top();
        let screen_rect = Rect::from_xy_wh(visual_left_top, {
            let screen_size = namui::screen::size();
            Wh::new(
                screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
                screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
            ) / self.camera.zoom_level
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
}

fn render_grid(ctx: &RenderCtx, game_state: &GameState) {
    // Capture game_state reference to avoid unused variable warning
    let _towers = &game_state.towers;

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

fn render_backgrounds(ctx: &RenderCtx, game_state: &GameState) {
    let visual_left_top = game_state.camera.visual_left_top();
    let screen_rect = Rect::from_xy_wh(visual_left_top, {
        let screen_size = namui::screen::size();
        Wh::new(
            screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
            screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
        ) / game_state.camera.zoom_level
    });

    for background in game_state.backgrounds.iter() {
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

                        mutate_game_state(|game_state| {
                            game_state.set_selected_tower(None);
                        });
                    }
                });
            }
        });
    }
}

fn render_projectiles(ctx: &RenderCtx, game_state: &GameState) {
    game_state.render_stuffs(
        ctx,
        game_state
            .projectiles
            .iter()
            .map(|projectile| (projectile.xy, projectile)),
    );
}

fn render_towers(ctx: &RenderCtx, game_state: &GameState) {
    let visual_left_top = game_state.camera.visual_left_top();
    let screen_rect = {
        let screen_size = namui::screen::size();
        Rect::from_xy_wh(visual_left_top, {
            Wh::new(
                screen_size.width.as_i32().as_f32() / TILE_PX_SIZE.width.as_f32(),
                screen_size.height.as_i32().as_f32() / TILE_PX_SIZE.height.as_f32(),
            ) / game_state.camera.zoom_level
        })
    };

    for tower in game_state.towers.iter() {
        let tower_xy = tower.left_top.map(|t| t.as_f32());

        // Culling check
        if screen_rect.right() < tower_xy.x || screen_rect.bottom() < tower_xy.y {
            continue;
        }

        let px_xy = TILE_PX_SIZE.to_xy() * tower_xy;
        ctx.translate(px_xy).compose(move |ctx| {
            // For now, just render the tower without hover functionality
            // We'll need to modify this once we can access mutable game state
            ctx.add(tower);

            // Render hover area
            let tower_size = 128.0; // TILE_PX_SIZE
            ctx.add(namui::rect(RectParam {
                rect: Rect::from_xy_wh(Xy::zero(), Wh::new(tower_size.px(), tower_size.px())),
                style: RectStyle {
                    fill: Some(RectFill {
                        color: Color::TRANSPARENT,
                    }),
                    ..Default::default()
                },
            }))
            .attach_event({
                let tower_id = tower.id();
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
                    mutate_game_state(move |game_state| {
                        game_state.set_selected_tower(Some(tower_id));
                    });
                }
            });
        });
    }
}

fn render_tower_info_popup(ctx: &RenderCtx, game_state: &GameState) {
    use crate::game_state::tower_info_popup::TowerInfoPopup;

    for tower in game_state.towers.iter() {
        if let Some(popup_state) = game_state.ui_state.get_popup_state(tower.id())
            && popup_state.is_visible()
        {
            let popup_scale = popup_state.scale;
            let popup_opacity = popup_state.opacity;

            if popup_scale > 0.01 && popup_opacity > 0.01 {
                let px_xy = TILE_PX_SIZE.to_xy() * tower.left_top.map(|t| t as f32)
                    + Xy::new(TILE_PX_SIZE.width, 0.px());

                ctx.translate(px_xy)
                    .scale(Xy::single(popup_scale / game_state.camera.zoom_level))
                    .add(TowerInfoPopup { tower });
            }
        }
    }
}

fn render_monsters(ctx: &RenderCtx, game_state: &GameState) {
    game_state.render_stuffs(
        ctx,
        game_state
            .monsters
            .iter()
            .map(|monster| (monster.move_on_route.xy(), monster)),
    );
}

fn render_cursor_preview(ctx: &RenderCtx, game_state: &GameState) {
    ctx.add(game_state.cursor_preview.render());

    // Render tower preview if in PlacingTower flow and hand has selected tower
    if let crate::game_state::GameFlow::PlacingTower { hand } = &game_state.flow {
        let selected_slot_ids = hand.selected_slot_ids();
        if let Some(&selected_slot_id) = selected_slot_ids.first()
            && let Some(tower_template) = hand.get_item(selected_slot_id)
        {
            ctx.add(
                crate::game_state::cursor_preview::tower::TowerCursorPreview {
                    tower_template,
                    map_coord: game_state.cursor_preview.map_coord,
                    placing_tower_slot_id: selected_slot_id,
                },
            );
        }
    }
}

fn render_field_particles(ctx: &RenderCtx, game_state: &GameState) {
    game_state
        .field_particle_system_manager
        .render(ctx, game_state.now());
}

fn render_laser_beams(ctx: &RenderCtx, game_state: &GameState) {
    let now = game_state.now();

    for laser in &game_state.laser_beams {
        let alpha = laser.current_alpha(now);
        if alpha <= 0.0 {
            continue;
        }

        // 맵 좌표를 픽셀 좌표로 변환
        let start_px = TILE_PX_SIZE.to_xy() * Xy::new(laser.start_xy.0, laser.start_xy.1);
        let end_px = TILE_PX_SIZE.to_xy() * Xy::new(laser.end_xy.0, laser.end_xy.1);

        // 레이저 색상에 투명도 적용
        let color = Color::from_f01(1.0, 0.2, 0.2, alpha);

        // 레이저 광선 그리기 (굵은 선)
        let mut path = Path::new();
        path = path.move_to(start_px.x, start_px.y);
        path = path.line_to(end_px.x, end_px.y);

        let paint = Paint::new(color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(px(8.0 * alpha))
            .set_stroke_cap(StrokeCap::Round);

        ctx.add(namui::path(path, paint));

        // 광선 중심에 더 밝은 선 추가 (광선 효과)
        let mut inner_path = Path::new();
        inner_path = inner_path.move_to(start_px.x, start_px.y);
        inner_path = inner_path.line_to(end_px.x, end_px.y);

        let inner_alpha = alpha * 0.8;
        let inner_paint = Paint::new(Color::WHITE.with_alpha((inner_alpha * 255.0) as u8))
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(px(3.0 * alpha))
            .set_stroke_cap(StrokeCap::Round);

        ctx.add(namui::path(inner_path, inner_paint));
    }
}

fn render_tower_emit_effects(ctx: &RenderCtx, game_state: &GameState) {
    let now = game_state.now();

    for effect in &game_state.tower_emit_effects {
        let progress = effect.progress(now);
        if progress >= 1.0 {
            continue;
        }

        // 타워에서 적까지 빛줄기 효과
        let tower_px = TILE_PX_SIZE.to_xy() * Xy::new(effect.tower_xy.0, effect.tower_xy.1);
        let target_px = TILE_PX_SIZE.to_xy() * Xy::new(effect.target_xy.0, effect.target_xy.1);

        // 진행도에 따라 빛줄기가 타워에서 적으로 이동
        let current_end = tower_px + (target_px - tower_px) * (progress * 2.0).min(1.0);

        let alpha = if progress < 0.5 {
            1.0
        } else {
            1.0 - (progress - 0.5) * 2.0
        };

        let color = match effect.kind {
            attack::instant_effect::InstantEffectKind::Explosion => {
                Color::from_f01(1.0, 0.5, 0.0, alpha)
            }
            attack::instant_effect::InstantEffectKind::Lightning => {
                Color::from_f01(1.0, 1.0, 0.2, alpha)
            }
            attack::instant_effect::InstantEffectKind::MagicCircle => {
                Color::from_f01(0.5, 0.2, 1.0, alpha)
            }
        };

        let mut path = Path::new();
        path = path.move_to(tower_px.x, tower_px.y);
        path = path.line_to(current_end.x, current_end.y);

        let paint = Paint::new(color)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(px(4.0))
            .set_stroke_cap(StrokeCap::Round);

        ctx.add(namui::path(path, paint));
    }
}

fn render_target_hit_effects(ctx: &RenderCtx, game_state: &GameState) {
    let now = game_state.now();

    for effect in &game_state.target_hit_effects {
        let progress = effect.progress(now);
        if progress >= 1.0 {
            continue;
        }

        let xy_px = TILE_PX_SIZE.to_xy() * Xy::new(effect.xy.0, effect.xy.1);
        let scale = effect.current_scale(now);
        let alpha = effect.current_alpha(now);

        match effect.kind {
            attack::instant_effect::InstantEffectKind::Explosion => {
                // 원형 폭발 이펙트 - 호로 근사
                let radius = 32.0 * scale;
                let num_points = 16;
                let mut path = Path::new();

                for i in 0..=num_points {
                    let angle = (i as f32 / num_points as f32) * std::f32::consts::PI * 2.0;
                    let x = xy_px.x + px(radius * angle.cos());
                    let y = xy_px.y + px(radius * angle.sin());
                    if i == 0 {
                        path = path.move_to(x, y);
                    } else {
                        path = path.line_to(x, y);
                    }
                }

                let color = Color::from_f01(1.0, 0.5, 0.0, alpha);
                let paint = Paint::new(color).set_style(PaintStyle::Fill);

                ctx.add(namui::path(path, paint));
            }
            attack::instant_effect::InstantEffectKind::Lightning => {
                // 번개 이펙트 (십자가 형태)
                let size = 24.0 * scale;
                let color = Color::from_f01(1.0, 1.0, 0.2, alpha);

                let mut path = Path::new();
                path = path.move_to(xy_px.x - px(size), xy_px.y);
                path = path.line_to(xy_px.x + px(size), xy_px.y);
                path = path.move_to(xy_px.x, xy_px.y - px(size));
                path = path.line_to(xy_px.x, xy_px.y + px(size));

                let paint = Paint::new(color)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(px(4.0 * scale))
                    .set_stroke_cap(StrokeCap::Round);

                ctx.add(namui::path(path, paint));
            }
            attack::instant_effect::InstantEffectKind::MagicCircle => {
                // 마법진 이펙트 (원형)
                let radius = 28.0 * scale;
                let num_points = 16;
                let mut path = Path::new();

                for i in 0..=num_points {
                    let angle = (i as f32 / num_points as f32) * std::f32::consts::PI * 2.0;
                    let x = xy_px.x + px(radius * angle.cos());
                    let y = xy_px.y + px(radius * angle.sin());
                    if i == 0 {
                        path = path.move_to(x, y);
                    } else {
                        path = path.line_to(x, y);
                    }
                }

                let color = Color::from_f01(0.5, 0.2, 1.0, alpha);
                let paint = Paint::new(color)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(px(3.0));

                ctx.add(namui::path(path, paint));
            }
        }
    }
}
