use namui::*;

use crate::{MapCoordF32, game_state::TILE_PX_SIZE};

#[derive(Clone)]
pub struct FieldDamageAreaParticle {
    pub shape: FieldAreaParticleShape,
    pub started_at: Instant,
    pub end_at: Instant,
}

impl FieldDamageAreaParticle {
    pub fn new(now: Instant, shape: FieldAreaParticleShape) -> Self {
        Self {
            shape,
            started_at: now,
            end_at: now + Duration::from_secs(2),
        }
    }

    pub fn tick(&mut self, _now: Instant, _dt: Duration) {
        // Nothing to do
    }

    pub fn render(&self) -> RenderingTree {
        let progress = self.progress();
        // Quad easing in-out function: starts slow, fast in middle, ends slow
        let eased_progress = if progress < 0.5 {
            2.0 * progress * progress
        } else {
            1.0 - 2.0 * (1.0 - progress).powi(2)
        };
        let alpha = (1.0 - eased_progress) * 0.75; // Fade out as time progresses, max opacity 0.75
        let color = Color::RED.with_alpha((alpha * 255.0) as u8);
        match &self.shape {
            FieldAreaParticleShape::Circle { center, radius } => {
                let center_px = Xy::new(
                    (center.x * TILE_PX_SIZE.width.as_f32()).px(),
                    (center.y * TILE_PX_SIZE.height.as_f32()).px(),
                );
                let radius_px = radius * TILE_PX_SIZE.width.as_f32();

                let circle_path = Path::new().add_oval(Rect::Xywh {
                    x: center_px.x - radius_px.px(),
                    y: center_px.y - radius_px.px(),
                    width: (radius_px * 2.0).px(),
                    height: (radius_px * 2.0).px(),
                });

                let fill_paint = Paint::new(color.with_alpha((alpha * 100.0) as u8)) // 20% of max opacity for fill
                    .set_style(PaintStyle::Fill)
                    .set_anti_alias(true);

                let stroke_paint = Paint::new(color)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(2.px())
                    .set_anti_alias(true);

                render([
                    path(circle_path.clone(), fill_paint),
                    path(circle_path, stroke_paint),
                ])
            }
            FieldAreaParticleShape::Line {
                start,
                end,
                thickness,
            } => {
                let start_px = Xy::new(
                    (start.x * TILE_PX_SIZE.width.as_f32()).px(),
                    (start.y * TILE_PX_SIZE.height.as_f32()).px(),
                );
                let end_px = Xy::new(
                    (end.x * TILE_PX_SIZE.width.as_f32()).px(),
                    (end.y * TILE_PX_SIZE.height.as_f32()).px(),
                );
                let direction = {
                    let delta = end_px - start_px;
                    let len = (delta.x.as_f32() * delta.x.as_f32()
                        + delta.y.as_f32() * delta.y.as_f32())
                    .sqrt();
                    if len > 0.0 {
                        Xy::new(
                            (delta.x.as_f32() / len) * TILE_PX_SIZE.width.as_f32() * 68.0,
                            (delta.y.as_f32() / len) * TILE_PX_SIZE.width.as_f32() * 68.0,
                        )
                    } else {
                        Xy::new(0.0, 0.0)
                    }
                };
                let end_px = start_px + direction.map(px);
                let thickness_px = TILE_PX_SIZE.width * *thickness;

                let line_path = Path::new()
                    .move_to(start_px.x, start_px.y)
                    .line_to(end_px.x, end_px.y);

                let paint = Paint::new(color)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(thickness_px)
                    .set_stroke_cap(StrokeCap::Round)
                    .set_anti_alias(true);

                render([path(line_path, paint)])
            }
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now > self.end_at
    }

    pub fn progress(&self) -> f32 {
        let now = Instant::now();
        let total_duration = self.end_at - self.started_at;
        let elapsed_duration = now - self.started_at;

        (elapsed_duration.as_secs_f32() / total_duration.as_secs_f32()).clamp(0.0, 1.0)
    }
}

#[derive(Clone)]
pub enum FieldAreaParticleShape {
    Circle {
        center: MapCoordF32,
        radius: f32,
    },
    Line {
        start: MapCoordF32,
        end: MapCoordF32,
        thickness: f32,
    },
}
