use crate::shop_panel::constants::{
    BG_HEIGHT, PAPER_HEIGHT, VOYAGER_ANIM_PERIOD, VOYAGER_HEIGHT, VOYAGER_WIDTH,
};
use namui::{self, Instant, *};

const STIFFNESS: f32 = -1000.0;
const DAMPING: f32 = -10.0;
const TRANSIT_FORCE: f32 = -50.0;
const FORCE_DURATION: Duration = Duration::from_millis(33);

const MAX_DELTA_TIME: f32 = 1.0 / 30.0;

pub(super) struct Voyager;

#[derive(Clone, PartialEq, State)]
struct VoyagerAnimation {
    last_tick_at: Instant,
    last_frame_index: u32,
    transit_force: Option<TransitForce>,
    y_ratio_offset: f32,
    y_ratio_velocity: f32,
}

#[derive(Clone, Copy, PartialEq, State)]
struct TransitForce {
    force: f32,
    end_at: Instant,
}

impl Component for Voyager {
    fn render(self, ctx: &RenderCtx) {
        let now = Instant::now();
        let (start_at, set_start_at) = ctx.state(Instant::now);

        let mut elapsed = (now - *start_at).as_secs_f32();
        if elapsed < 0.0 {
            set_start_at.set(now);
            elapsed = 0.0;
        }
        let frame_index = ((elapsed / VOYAGER_ANIM_PERIOD.as_secs_f32()) as u32) % 2;

        let (animation, set_animation) = ctx.state(|| VoyagerAnimation {
            last_tick_at: now,
            last_frame_index: frame_index,
            transit_force: None,
            y_ratio_offset: 0.0,
            y_ratio_velocity: 0.0,
        });

        let mut new_anim = (*animation).clone();
        let mut delta_time = (now - animation.last_tick_at).as_secs_f32();
        delta_time = delta_time.clamp(0.0, MAX_DELTA_TIME);
        new_anim.last_tick_at = now;

        if frame_index != animation.last_frame_index {
            new_anim.last_frame_index = frame_index;
            new_anim.transit_force = Some(TransitForce {
                force: TRANSIT_FORCE,
                end_at: now + FORCE_DURATION,
            });
        }

        let transit_force_expired = new_anim.transit_force.is_some_and(|tf| tf.end_at < now);
        let transit_force = new_anim.transit_force.map_or(0.0, |tf| tf.force);

        let spring_force = STIFFNESS * new_anim.y_ratio_offset;
        let damping_force = DAMPING * new_anim.y_ratio_velocity;
        let acceleration: f32 = spring_force + damping_force + transit_force;
        new_anim.y_ratio_velocity += acceleration * delta_time;
        new_anim.y_ratio_offset += new_anim.y_ratio_velocity * delta_time;

        if transit_force_expired {
            new_anim.transit_force = None;
        }

        let scale = Xy::new(
            1.0 + new_anim.y_ratio_offset * -0.5,
            1.0 + new_anim.y_ratio_offset,
        );
        set_animation.set(new_anim);

        let voyager_image = if frame_index == 0 {
            crate::asset::image::ui::voyager::VOYAGER_00
        } else {
            crate::asset::image::ui::voyager::VOYAGER_01
        };

        let voyager_off = Xy::new(
            -VOYAGER_WIDTH * 0.25,
            PAPER_HEIGHT - BG_HEIGHT - VOYAGER_HEIGHT * 0.75,
        );

        ctx.translate(voyager_off)
            .translate((VOYAGER_WIDTH * 0.5, VOYAGER_HEIGHT))
            .scale(scale)
            .add(namui::image(ImageParam {
                rect: Rect::from_xy_wh(
                    Xy::new(-VOYAGER_WIDTH * 0.5, -VOYAGER_HEIGHT),
                    Wh::new(VOYAGER_WIDTH, VOYAGER_HEIGHT),
                ),
                image: voyager_image,
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }));
    }
}
