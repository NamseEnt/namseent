use super::{Tower, TowerKind, TowerTemplate};
use crate::card::{Rank, Suit};
use crate::game_state::{GameState, TILE_PX_SIZE, use_game_state};
use crate::hand::shared::get_suit_color;
use crate::icon::{Icon, IconKind, IconSize};
use crate::palette;
use crate::sound::{
    self, EmitSoundParams, SoundGroup, SpatialMode, VolumePreset, random_murchunga,
};
use crate::theme::typography::{FontSize, memoized_text};
use namui::*;

// ----- Tower suit/rank overlay tuning (adjust for your tower sprite)
pub const TOWER_OVERLAY_SUIT_X_RATIO: f32 = 0.21;
pub const TOWER_OVERLAY_RANK_X_RATIO: f32 = 0.6;
pub const TOWER_OVERLAY_SIDE_Y_RATIO: f32 = 0.62;
pub const TOWER_OVERLAY_ICON_SCALE: f32 = 0.85;
pub const TOWER_OVERLAY_ROTATION_DEG: f32 = -12.0;
pub const TOWER_OVERLAY_ICON_SIZE_PX: f32 = 64.0;

pub trait TowerImage {
    fn image(self) -> Image;
}

impl TowerImage for (TowerKind, AnimationKind) {
    fn image(self) -> Image {
        let (tower_kind, animation_kind) = self;
        match tower_kind {
            TowerKind::Barricade => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::barricade::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::barricade::IDLE1,
                AnimationKind::Attack => crate::asset::image::tower::barricade::IDLE1,
            },
            TowerKind::High => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::high::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::high::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::high::ATTACK,
            },
            TowerKind::OnePair => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::one_pair::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::one_pair::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::one_pair::ATTACK,
            },
            TowerKind::TwoPair => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::two_pair::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::two_pair::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::two_pair::ATTACK,
            },
            TowerKind::ThreeOfAKind => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::three_of_a_kind::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::three_of_a_kind::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::three_of_a_kind::ATTACK,
            },
            TowerKind::Straight => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::straight::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::straight::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::straight::ATTACK,
            },
            TowerKind::Flush => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::flush::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::flush::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::flush::ATTACK,
            },
            TowerKind::FullHouse => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::full_house::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::full_house::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::full_house::ATTACK,
            },
            TowerKind::FourOfAKind => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::four_of_a_kind::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::four_of_a_kind::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::four_of_a_kind::ATTACK,
            },
            TowerKind::StraightFlush => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::straight_flush::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::straight_flush::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::straight_flush::ATTACK,
            },
            TowerKind::RoyalFlush => match animation_kind {
                AnimationKind::Idle1 => crate::asset::image::tower::royal_flush::IDLE1,
                AnimationKind::Idle2 => crate::asset::image::tower::royal_flush::IDLE2,
                AnimationKind::Attack => crate::asset::image::tower::royal_flush::ATTACK,
            },
        }
    }
}

pub struct TowerSuitRankOverlay {
    pub suit: Suit,
    pub rank: Rank,
    pub image_wh: Wh<Px>,
    /// The coordinate of the image's top-left corner in the current coordinate system.
    ///
    /// - For tower placement rendering, the coordinate system is already moved so that the
    ///   origin is the bottom-center of the tower sprite, so this should be
    ///   `Xy::new(-image_wh.width * 0.5, -image_wh.height)`.
    /// - For previews (top-left origin), use `Xy::zero()`.
    pub origin: Xy<Px>,
    pub alpha: f32,
}

impl Component for TowerSuitRankOverlay {
    fn render(self, ctx: &RenderCtx) {
        let TowerSuitRankOverlay {
            suit,
            rank,
            image_wh,
            origin,
            alpha,
        } = self;

        let alpha = alpha.clamp(0.0, 1.0);
        let mut text_color = get_suit_color(suit);
        text_color.a = (alpha * 255.0).round() as u8;

        // Position the suit/rank markers relative to the tower image bounds.
        let center_y = image_wh.height * TOWER_OVERLAY_SIDE_Y_RATIO;
        let left_x = image_wh.width * TOWER_OVERLAY_SUIT_X_RATIO;
        let right_x = image_wh.width * TOWER_OVERLAY_RANK_X_RATIO;
        let icon_wh = Wh::new(
            TOWER_OVERLAY_ICON_SIZE_PX.px(),
            TOWER_OVERLAY_ICON_SIZE_PX.px(),
        );
        let rotation = TOWER_OVERLAY_ROTATION_DEG.deg();
        let icon_scale = TOWER_OVERLAY_ICON_SCALE;

        // Suit (left side)
        ctx.compose(|ctx| {
            let mut icon = Icon::new(IconKind::Suit { suit })
                .wh(icon_wh)
                .size(IconSize::Custom {
                    size: icon_wh.height,
                });
            icon.opacity = alpha;

            ctx.translate(Xy::new(origin.x + left_x, origin.y + center_y))
                .rotate(-rotation)
                .scale(Xy::new(icon_scale, icon_scale))
                .add(icon);
        });

        // Rank (right side)
        ctx.compose(|ctx| {
            ctx.translate(Xy::new(origin.x + right_x, origin.y + center_y))
                .rotate(rotation)
                .scale(Xy::new(icon_scale, icon_scale))
                .add(memoized_text((&rank, &text_color), |mut builder| {
                    builder
                        .headline()
                        .size(FontSize::Custom {
                            size: icon_wh.height,
                        })
                        .color(text_color)
                        .text(rank.to_string())
                        .render_left_top()
                }));
        });
    }
}

pub struct RenderTower<'a> {
    pub tower: &'a Tower,
    pub now: Instant,
}

impl Component for RenderTower<'_> {
    fn render(self, ctx: &RenderCtx) {
        let RenderTower { tower, now } = self;

        if let Some(visual) = tower.royal_straight_flush_visual() {
            render_tower_sprite(ctx, tower, (0.0, 0.0), visual.original_alpha(now));

            let clone_alpha = visual.clone_alpha(now);
            let tower_left_top = tower.left_top.map(|t| t as f32);
            for clone_center_xy in visual.clone_positions(now) {
                let clone_left_top = Xy::new(clone_center_xy.0 - 1.0, clone_center_xy.1 - 1.0);
                let local_offset = (
                    clone_left_top.x - tower_left_top.x,
                    clone_left_top.y - tower_left_top.y,
                );
                render_tower_sprite(ctx, tower, local_offset, clone_alpha);
            }
            return;
        }

        render_tower_sprite(ctx, tower, (0.0, 0.0), 1.0);
    }
}

fn render_tower_sprite(ctx: &RenderCtx, tower: &Tower, local_left_top_xy: (f32, f32), alpha: f32) {
    if alpha <= 0.01 {
        return;
    }

    let image = (tower.kind, tower.animation.kind).image();
    let image_wh = image.info().wh();
    let scale = Xy::new(
        1.0 + tower.animation.y_ratio_offset * -0.5,
        1.0 + tower.animation.y_ratio_offset,
    );
    let paint = if alpha >= 0.999 {
        None
    } else {
        Some(Paint::new(Color::grayscale_alpha_f01(
            1.0,
            alpha.clamp(0.0, 1.0),
        )))
    };

    ctx.translate(TILE_PX_SIZE.to_xy() * Xy::new(local_left_top_xy.0, local_left_top_xy.1))
        .translate((image_wh.width * 0.5, image_wh.height))
        .scale(scale)
        .add(TowerSuitRankOverlay {
            suit: tower.suit,
            rank: tower.rank,
            image_wh,
            origin: Xy::new(-image_wh.width * 0.5, -image_wh.height),
            alpha,
        })
        .add(namui::image(ImageParam {
            rect: Rect::from_xy_wh(
                Xy::new(-image_wh.width * 0.5, -image_wh.height),
                image.info().wh(),
            ),
            image,
            style: ImageStyle {
                fit: ImageFit::None,
                paint,
            },
        }));
}

pub(crate) struct TowerAttackRange<'a> {
    pub(crate) tower_template: &'a TowerTemplate,
}

impl Component for TowerAttackRange<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { tower_template } = self;

        let game_state = use_game_state(ctx);

        let range_radius_px = TILE_PX_SIZE.width * tower_template.default_attack_range_radius;

        const ROTATION_SPEED_PX_PER_SEC: f32 = 120.0;
        const DASH_ON_PX: f32 = 40.0;
        const DASH_OFF_PX: f32 = 24.0;

        let elapsed_secs = (game_state.now() - Instant::new(Duration::ZERO)).as_secs_f32();
        let phase_px = (elapsed_secs * ROTATION_SPEED_PX_PER_SEC) % (DASH_ON_PX + DASH_OFF_PX);

        let ctx = ctx.translate(TILE_PX_SIZE.to_xy());

        let oval = Rect::Ltrb {
            left: -range_radius_px,
            top: -range_radius_px,
            right: range_radius_px,
            bottom: range_radius_px,
        };
        let path = Path::new().add_oval(oval);
        let paint = Paint::new(palette::PRIMARY)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(4.px())
            .set_stroke_cap(StrokeCap::Round)
            .set_path_effect(PathEffect::Dash {
                on: DASH_ON_PX,
                off: DASH_OFF_PX,
                phase: phase_px,
            });
        ctx.add(namui::path(path, paint));
    }
}

pub fn tower_animation_tick(game_state: &mut GameState, now: Instant) {
    // STIFFNESS represents the spring constant in the physics simulation.
    // A negative value is used to simulate a restoring force that pulls the tower back to its equilibrium position.
    const STIFFNESS: f32 = -1500.0;

    // DAMPING represents the damping coefficient, which reduces oscillations over time.
    // A negative value is used to simulate a force opposing the velocity of the tower's animation.
    const DAMPING: f32 = -10.0;

    game_state.towers.iter_mut().for_each(|tower| {
        let Tower {
            animation,
            template,
            ..
        } = tower;
        let kind = template.kind;
        if let TowerKind::Barricade = kind {
            return;
        }

        let delta_time = (now - animation.tick_at).as_secs_f32();
        animation.tick_at = now;

        if now - animation.transited_at > animation.duration() {
            animation.transition(
                match animation.kind {
                    AnimationKind::Idle1 => AnimationKind::Idle2,
                    AnimationKind::Idle2 => AnimationKind::Idle1,
                    AnimationKind::Attack => AnimationKind::Idle1,
                },
                now,
            );

            sound::emit_sound(EmitSoundParams::one_shot(
                random_murchunga(),
                SoundGroup::Sfx,
                VolumePreset::Minimum,
                SpatialMode::Spatial {
                    position: tower.left_top.map(|coord| coord as f32) + Xy::new(1.0, 1.0),
                },
            ));
        }

        let transit_force_expired = animation
            .transit_force
            .is_some_and(|transit_force| transit_force.end_at < now);
        let transit_force = animation
            .transit_force
            .map(|transit_force| transit_force.force)
            .unwrap_or(0.0);
        let spring_force = STIFFNESS * animation.y_ratio_offset;
        let damping_force = DAMPING * animation.y_ratio_velocity;
        let acceleration = spring_force + damping_force + transit_force;
        animation.y_ratio_velocity += acceleration * delta_time;
        animation.y_ratio_offset += animation.y_ratio_velocity * delta_time;

        if transit_force_expired {
            animation.transit_force = None;
        }
    });
}

#[derive(Clone, PartialEq, State)]
pub(super) struct Animation {
    kind: AnimationKind,
    transited_at: Instant,
    transit_force: Option<TransitForce>,
    tick_at: Instant,
    y_ratio_offset: f32,
    y_ratio_velocity: f32,
}

impl Animation {
    pub(super) fn new(now: Instant) -> Self {
        Self {
            kind: AnimationKind::Idle1,
            transited_at: now,
            transit_force: None,
            tick_at: now,
            y_ratio_offset: 0.0,
            y_ratio_velocity: 0.0,
        }
    }

    pub(super) fn transition(&mut self, kind: AnimationKind, now: Instant) {
        const IDLE_TRANSIT_FORCE: f32 = -100.0;
        const ATTACK_TRANSIT_FORCE: f32 = -500.0;
        const FORCE_DURATION: Duration = Duration::from_millis(33);

        if let AnimationKind::Attack = kind {
            self.transit_force = Some(TransitForce {
                force: ATTACK_TRANSIT_FORCE,
                end_at: now + FORCE_DURATION,
            });
        } else if let AnimationKind::Attack = self.kind {
            // Ignore transit force for transition from attack to idle
        } else {
            self.transit_force = Some(TransitForce {
                force: IDLE_TRANSIT_FORCE,
                end_at: now + FORCE_DURATION,
            });
        }

        self.kind = kind;
        self.transited_at = now;
    }

    fn duration(&self) -> Duration {
        self.kind.duration()
    }
}

#[derive(Clone, Copy, PartialEq, State)]
struct TransitForce {
    force: f32,
    end_at: Instant,
}

#[derive(Clone, Copy, PartialEq, State)]
pub enum AnimationKind {
    Idle1,
    Idle2,
    Attack,
}
impl AnimationKind {
    fn duration(&self) -> Duration {
        match self {
            Self::Idle1 => Duration::from_millis(1500),
            Self::Idle2 => Duration::from_millis(1500),
            Self::Attack => Duration::from_millis(333),
        }
    }
}
