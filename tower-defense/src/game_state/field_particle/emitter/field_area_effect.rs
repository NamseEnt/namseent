use crate::MapCoordF32;
use crate::game_state::TILE_PX_SIZE;
use crate::game_state::{
    field_area_effect::FieldAreaEffectKind,
    field_particle::{
        FieldParticle,
        particle::{
            FieldAreaParticleShape, FieldDamageAreaParticle, IconParticle, IconParticleBehavior,
        },
    },
    schedule::CountBasedSchedule,
};
use crate::icon::{Icon, IconKind, IconSize};
use namui::*;
use rand::Rng;
use std::f32::consts::TAU;

const PARTICLE_DURATION: Duration = Duration::from_secs(1);
const ICON_SIZE: f32 = 48.0;
const LINE_LENGTH_TILES: f32 = 68.0;
const MIN_SPEED: f32 = 2.0;
const MAX_SPEED: f32 = 6.0;
const PARTICLES_PER_UNIT: f32 = 6.0;
const MIN_PARTICLES: f32 = 8.0;
const MAX_PARTICLES: f32 = 32.0;
const AIR_RESISTANCE: f32 = 2.0;
const ANGULAR_RESISTANCE: f32 = 0.5;
const GRAVITY_MULTIPLIER: f32 = 9.8;

pub struct FieldAreaEffectEmitter {
    kind: FieldAreaEffectKind,
    emit_schedule: CountBasedSchedule,
}
impl FieldAreaEffectEmitter {
    pub fn new(_now: Instant, kind: FieldAreaEffectKind, schedule: CountBasedSchedule) -> Self {
        Self {
            kind,
            emit_schedule: schedule,
        }
    }

    fn map_coord_to_pixel_f32(coord: MapCoordF32) -> Xy<f32> {
        let pixel = TILE_PX_SIZE.to_xy() * coord;
        Xy {
            x: pixel.x.as_f32(),
            y: pixel.y.as_f32(),
        }
    }

    fn calculate_particle_count(value: f32, is_line: bool) -> usize {
        if is_line {
            (value * PARTICLES_PER_UNIT).clamp(96.0, 128.0) as usize
        } else {
            (value * PARTICLES_PER_UNIT).clamp(MIN_PARTICLES, MAX_PARTICLES) as usize
        }
    }

    fn generate_random_rotation_and_angular_velocity(
        rng: &mut impl Rng,
    ) -> (Angle, Per<Angle, Duration>) {
        let rotation = rng.gen_range(0.0..360.0).deg();
        let angular_velocity = Per::new(
            rng.gen_range(-1800..1800).deg(),
            Duration::from_secs_f32(1.0),
        );
        (rotation, angular_velocity)
    }

    fn calculate_speed_from_distance(distance_factor: f32) -> f32 {
        (MAX_SPEED - (MAX_SPEED - MIN_SPEED) * distance_factor) * TILE_PX_SIZE.width.as_f32()
    }

    fn make_damage_area_particle(now: Instant, shape: FieldAreaParticleShape) -> FieldParticle {
        let damage_particle = FieldDamageAreaParticle::new(now, shape);
        FieldParticle::FieldDamageArea {
            particle: damage_particle,
        }
    }

    fn make_icon_particle(
        now: Instant,
        position: Xy<f32>,
        velocity: Xy<Px>,
        rotation: Angle,
        angular_velocity: Per<Angle, Duration>,
    ) -> FieldParticle {
        let behavior = IconParticleBehavior::Physics {
            duration: PARTICLE_DURATION,
            created_at: now,
            velocity: Per::new(velocity, PARTICLE_DURATION),
            angular_velocity,
            scale: 1.0,
            air_resistance: Per::new(AIR_RESISTANCE, PARTICLE_DURATION),
            angular_resistance: Per::new(ANGULAR_RESISTANCE, PARTICLE_DURATION),
            gravity_acceleration_per_second: Per::new(
                TILE_PX_SIZE.height * GRAVITY_MULTIPLIER,
                PARTICLE_DURATION,
            ),
        };
        let icon_particle = IconParticle {
            icon: Icon {
                kind: IconKind::AttackDamage,
                size: IconSize::Custom {
                    size: px(ICON_SIZE),
                },
                attributes: vec![],
                wh: Wh::single(px(ICON_SIZE)),
                opacity: 1.0,
            },
            xy: Xy {
                x: px(position.x),
                y: px(position.y),
            },
            rotation,
            behavior,
        };
        FieldParticle::Icon {
            particle: icon_particle,
        }
    }

    fn make_icon_particles_circle(
        now: Instant,
        map_center_coordinate: MapCoordF32,
        radius_tile: f32,
    ) -> Vec<FieldParticle> {
        let center_pixel_f32 = Self::map_coord_to_pixel_f32(map_center_coordinate);
        let radius_pixel = TILE_PX_SIZE.width.as_f32() * radius_tile;
        let particle_count = Self::calculate_particle_count(radius_pixel, false);
        let mut rng = rand::thread_rng();
        let mut field_particles = Vec::with_capacity(particle_count);

        for _ in 0..particle_count {
            let angle = rng.gen_range(0.0..TAU);
            let direction = Xy {
                x: angle.cos(),
                y: angle.sin(),
            };
            let distance_factor = rng.gen_range(0.0..=1.0);
            let speed = Self::calculate_speed_from_distance(distance_factor);
            let velocity = Xy {
                x: px(direction.x * speed),
                y: px(direction.y * speed),
            };
            let random_radius = rng.gen_range(0.0..=radius_pixel);
            let random_position = Xy {
                x: center_pixel_f32.x + direction.x * random_radius,
                y: center_pixel_f32.y + direction.y * random_radius,
            };
            let (rotation, angular_velocity) =
                Self::generate_random_rotation_and_angular_velocity(&mut rng);
            field_particles.push(Self::make_icon_particle(
                now,
                random_position,
                velocity,
                rotation,
                angular_velocity,
            ));
        }
        field_particles
    }

    fn make_icon_particles_line(
        now: Instant,
        center_xy: MapCoordF32,
        target_xy: MapCoordF32,
        thickness: f32,
    ) -> Vec<FieldParticle> {
        let center_pixel_f32 = Self::map_coord_to_pixel_f32(center_xy);
        let target_pixel_f32 = Self::map_coord_to_pixel_f32(target_xy);
        let line_length = LINE_LENGTH_TILES * TILE_PX_SIZE.width.as_f32();

        let (normalized_direction, normal) = {
            let dx = target_pixel_f32.x - center_pixel_f32.x;
            let dy = target_pixel_f32.y - center_pixel_f32.y;
            let length = (dx * dx + dy * dy).sqrt();
            let direction = if length > 0.0 {
                Xy {
                    x: dx / length,
                    y: dy / length,
                }
            } else {
                Xy { x: 1.0, y: 0.0 }
            };
            let normal = Xy {
                x: -direction.y,
                y: direction.x,
            };
            (direction, normal)
        };

        let particle_count = Self::calculate_particle_count(thickness, true);
        let mut rng = rand::thread_rng();
        let mut particles = Vec::with_capacity(particle_count);

        for _ in 0..particle_count {
            let thickness_factor = rng.gen_range(0.0..=1.0);
            let offset = (thickness_factor - 0.5) * thickness * TILE_PX_SIZE.width.as_f32();
            let along = rng.gen_range(0.0..=1.0) * line_length;

            let position = Xy {
                x: center_pixel_f32.x + normalized_direction.x * along + normal.x * offset,
                y: center_pixel_f32.y + normalized_direction.y * along + normal.y * offset,
            };

            let distance_from_center = ((along - line_length / 2.0).abs()) / (line_length / 2.0);
            let speed = Self::calculate_speed_from_distance(distance_from_center);

            let outward_direction = if offset.abs() > 0.0 {
                Xy {
                    x: normal.x * offset.signum(),
                    y: normal.y * offset.signum(),
                }
            } else {
                normal
            };

            let velocity = Xy {
                x: px(outward_direction.x * speed),
                y: px(outward_direction.y * speed),
            };

            let (rotation, angular_velocity) =
                Self::generate_random_rotation_and_angular_velocity(&mut rng);

            particles.push(Self::make_icon_particle(
                now,
                position,
                velocity,
                rotation,
                angular_velocity,
            ));
        }
        particles
    }

    fn create_particles_for_shape(
        now: Instant,
        shape: FieldAreaParticleShape,
    ) -> Vec<FieldParticle> {
        let mut particles = vec![Self::make_damage_area_particle(now, shape.clone())];

        match shape {
            FieldAreaParticleShape::Circle { center, radius } => {
                particles.extend(Self::make_icon_particles_circle(now, center, radius));
            }
            FieldAreaParticleShape::Line {
                start,
                end,
                thickness,
            } => {
                particles.extend(Self::make_icon_particles_line(now, start, end, thickness));
            }
        }

        particles
    }

    pub fn emit(&mut self, now: Instant, _dt: Duration) -> Vec<FieldParticle> {
        if !self.emit_schedule.try_emit(now) {
            return vec![];
        }

        match self.kind {
            FieldAreaEffectKind::RoundDamage { xy, radius, .. }
            | FieldAreaEffectKind::RoundDamageOverTime { xy, radius, .. } => {
                Self::create_particles_for_shape(
                    now,
                    FieldAreaParticleShape::Circle { center: xy, radius },
                )
            }
            FieldAreaEffectKind::LinearDamage {
                center_xy,
                target_xy,
                thickness,
                ..
            }
            | FieldAreaEffectKind::LinearDamageOverTime {
                center_xy,
                target_xy,
                thickness,
                ..
            } => Self::create_particles_for_shape(
                now,
                FieldAreaParticleShape::Line {
                    start: center_xy,
                    end: target_xy,
                    thickness,
                },
            ),
        }
    }

    pub fn is_done(&self, now: Instant) -> bool {
        self.emit_schedule.is_done(now)
    }
}
