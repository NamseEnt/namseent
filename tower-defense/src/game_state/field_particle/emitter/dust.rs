use crate::game_state::field_particle::particle::DustParticleParams;
use crate::game_state::field_particle::{DustParticle, DustParticleConfig, spawn_dust_particle};
use namui::*;
use rand::Rng;

pub const DUST_CLUSTER_COUNT: usize = 5;
pub const DUST_CLUSTER_SPAWN_RADIUS_TILE: f32 = 0.125;
pub const DUST_CLUSTER_SPEED_MIN_TILE_PER_SEC: f32 = 0.3;
pub const DUST_CLUSTER_SPEED_MAX_TILE_PER_SEC: f32 = 1.0;
pub const DUST_CLUSTER_ROTATION_SPEED_MIN_TURNS_PER_SEC: f32 = -0.25;
pub const DUST_CLUSTER_ROTATION_SPEED_MAX_TURNS_PER_SEC: f32 = 0.25;
pub const DUST_PARTICLES_PER_CLUSTER_MIN: usize = 5;
pub const DUST_PARTICLES_PER_CLUSTER_MAX: usize = 10;
pub const DUST_PARTICLE_LOCAL_SPAWN_RADIUS_TILE: f32 = 0.25;
pub const DUST_PARTICLE_SPEED_MIN_TILE_PER_SEC: f32 = 0.05;
pub const DUST_PARTICLE_SPEED_MAX_TILE_PER_SEC: f32 = 0.1;
pub const DUST_PARTICLE_SCALE_IN_MS: i64 = 50;
pub const DUST_PARTICLE_FINAL_SCALE_MULTIPLIER: f32 = 1.5;
pub const DUST_PARTICLE_LIFETIME_MS: i64 = 1500;
pub const DUST_PARTICLE_BASE_SIZE_MIN_TILE: f32 = 0.6;
pub const DUST_PARTICLE_BASE_SIZE_MAX_TILE: f32 = 0.9;

fn random_unit_direction<R: Rng + ?Sized>(rng: &mut R) -> (f32, f32) {
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    (angle.cos(), angle.sin())
}

fn random_disk_offset<R: Rng + ?Sized>(rng: &mut R, max_radius: f32) -> (f32, f32) {
    let angle = rng.gen_range(0.0..std::f32::consts::TAU);
    let radius = rng.gen_range(0.0..=1.0f32).sqrt() * max_radius;
    (angle.cos() * radius, angle.sin() * radius)
}

pub fn spawn_tower_remove_dust_burst(tower_center_xy: (f32, f32), now: Instant) {
    let mut rng = rand::thread_rng();

    for _ in 0..DUST_CLUSTER_COUNT {
        let offset = random_disk_offset(&mut rng, DUST_CLUSTER_SPAWN_RADIUS_TILE);
        let cluster_xy = (tower_center_xy.0 + offset.0, tower_center_xy.1 + offset.1);

        let outward_dir = if offset.0.abs() > 1e-6 || offset.1.abs() > 1e-6 {
            let radius = (offset.0 * offset.0 + offset.1 * offset.1).sqrt();
            (offset.0 / radius, offset.1 / radius)
        } else {
            random_unit_direction(&mut rng)
        };

        let cluster_speed = rng
            .gen_range(DUST_CLUSTER_SPEED_MIN_TILE_PER_SEC..=DUST_CLUSTER_SPEED_MAX_TILE_PER_SEC);
        let cluster_velocity_xy = (outward_dir.0 * cluster_speed, outward_dir.1 * cluster_speed);

        let cluster_rotation_speed_turns_per_sec = rng.gen_range(
            DUST_CLUSTER_ROTATION_SPEED_MIN_TURNS_PER_SEC
                ..=DUST_CLUSTER_ROTATION_SPEED_MAX_TURNS_PER_SEC,
        );
        let particles_to_spawn =
            rng.gen_range(DUST_PARTICLES_PER_CLUSTER_MIN..=DUST_PARTICLES_PER_CLUSTER_MAX);

        for _ in 0..particles_to_spawn {
            let local_spawn_offset =
                random_disk_offset(&mut rng, DUST_PARTICLE_LOCAL_SPAWN_RADIUS_TILE);

            let move_angle = rng.gen_range(0.0..std::f32::consts::TAU);
            let move_speed = rng.gen_range(
                DUST_PARTICLE_SPEED_MIN_TILE_PER_SEC..=DUST_PARTICLE_SPEED_MAX_TILE_PER_SEC,
            );
            let particle_velocity_xy =
                (move_angle.cos() * move_speed, move_angle.sin() * move_speed);
            let base_size_tile =
                rng.gen_range(DUST_PARTICLE_BASE_SIZE_MIN_TILE..=DUST_PARTICLE_BASE_SIZE_MAX_TILE);

            spawn_dust_particle(DustParticle::new(
                DustParticleParams {
                    cluster_spawn_xy: cluster_xy,
                    cluster_velocity_xy,
                    local_spawn_offset_xy: local_spawn_offset,
                    local_velocity_xy: particle_velocity_xy,
                    cluster_rotation_speed_turns_per_sec,
                },
                now,
                DustParticleConfig {
                    lifetime: Duration::from_millis(DUST_PARTICLE_LIFETIME_MS),
                    scale_in_duration: Duration::from_millis(DUST_PARTICLE_SCALE_IN_MS),
                    base_size_tile,
                    final_scale_multiplier: DUST_PARTICLE_FINAL_SCALE_MULTIPLIER,
                },
                &mut rng,
            ));
        }
    }
}
