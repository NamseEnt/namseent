use namui::*;

const SOUL_OPACITY_START: f32 = 0.75;
const SOUL_OFFSET_MAX_PX: f32 = 128.0;
const SOUL_SCALE_MIN: f32 = 0.0;
const SOUL_SCALE_MAX: f32 = 1.0;

#[derive(Clone, State)]
pub struct MonsterDeathParticle {
    pub position: Xy<Px>,
    pub created_at: Instant,
    pub duration: Duration,
    pub rotation: Angle,
    pub opacity: f32,
    pub scale: Xy<f32>,
    pub offset: Px,
}

impl MonsterDeathParticle {
    pub fn new(position: Xy<Px>, now: Instant, rotation: Angle) -> Self {
        Self {
            position,
            created_at: now,
            duration: 0.6.sec(),
            rotation,
            opacity: SOUL_OPACITY_START,
            scale: Xy::single(SOUL_SCALE_MIN),
            offset: 0.px(),
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }

    pub fn tick(&mut self, now: Instant, _delta_time: Duration) {
        let elapsed = now - self.created_at;
        let progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);

        let remain = 1.0 - progress;
        let remain_pow4 = remain * remain * remain * remain; // (1 - t)^4
        let eased = 1.0 - remain_pow4; // easeOutQuart(t)

        // Opacity: 0.75 -> 0.0 following (1 - t)^4
        self.opacity = SOUL_OPACITY_START * remain_pow4;

        // Offset: 0 -> MAX following easeOutQuart
        self.offset = px(SOUL_OFFSET_MAX_PX * eased);

        // Scale: MIN -> MAX following easeOutQuart
        let scale_v = SOUL_SCALE_MIN + (SOUL_SCALE_MAX - SOUL_SCALE_MIN) * eased;
        self.scale = Xy::single(scale_v);
    }

    pub fn render(&self) -> RenderingTree {
        let Self {
            rotation,
            opacity,
            scale,
            offset,
            ..
        } = self;

        let wh = Wh::new(px(128.0), px(192.0));

        let paint = Paint::new(Color::WHITE.with_alpha((opacity * 255.0) as u8));

        namui::translate(
            self.position.x,
            self.position.y,
            namui::rotate(
                *rotation,
                namui::translate(
                    0.px(),
                    -*offset,
                    namui::scale(
                        scale.x,
                        scale.y,
                        namui::image(ImageParam {
                            rect: Rect::from_xy_wh(Xy::new(-wh.width * 0.5, -wh.height), wh),
                            image: crate::asset::image::MONSTER_SOUL,
                            style: ImageStyle {
                                fit: ImageFit::None,
                                paint: Some(paint),
                            },
                        }),
                    ),
                ),
            ),
        )
    }
}
