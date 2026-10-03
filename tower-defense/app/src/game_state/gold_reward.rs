use namui::*;
use rand::Rng;

const POP_GRAVITY: f32 = 5.5;
const BOUNCE_RESTITUTION: f32 = 0.48;
const HORIZONTAL_FRICTION: f32 = 0.62;
const ROTATION_DAMPING_PER_SEC: f32 = 2.8;
const COIN_FADE_DURATION_MIN_SECS: f32 = 0.30;
const COIN_FADE_DURATION_MAX_SECS: f32 = 0.46;
const COIN_SIZE_PX: f32 = 34.0;
const COIN_SPAWN_STEP_SECS: f32 = 0.10;
const MAX_COIN_SPAWN_WINDOW_SECS: f32 = 0.90;

#[derive(Clone, State, Default)]
pub(crate) struct GoldRewardPresentation {
    coins: Vec<GoldRewardCoin>,
    sparkles: Vec<GoldSparkleParticle>,
}

impl GoldRewardPresentation {
    pub(crate) fn spawn(&mut self, position: Xy<f32>, amount: usize) {
        if amount == 0 {
            return;
        }

        let mut rng = rand::thread_rng();
        let spawn_gap_weights = (1..amount)
            .map(|_| rng.gen_range(0.5..=1.5))
            .collect::<Vec<_>>();
        let total_spawn_gap_weight = spawn_gap_weights.iter().sum::<f32>();
        let spawn_window_secs = ((amount.saturating_sub(1) as f32) * COIN_SPAWN_STEP_SECS)
            .min(MAX_COIN_SPAWN_WINDOW_SECS);
        let mut launch_delay_secs = rng.gen_range(0.0..0.025);

        for coin_index in 0..amount {
            if coin_index > 0 {
                launch_delay_secs +=
                    spawn_window_secs * spawn_gap_weights[coin_index - 1] / total_spawn_gap_weight;
            }
            let origin_xy =
                position + Xy::new(rng.gen_range(-0.18..=0.18), rng.gen_range(-0.06..=0.06));
            let landing_offset_y = rng.gen_range(0.04..=0.14);
            let velocity_xy: Xy<f32> =
                Xy::new(rng.gen_range(-1.8..=1.8), rng.gen_range(-3.0..=-2.2));
            let time_to_ground = (-velocity_xy.y
                + (velocity_xy.y.powi(2) + 2.0 * POP_GRAVITY * landing_offset_y).sqrt())
                / POP_GRAVITY;
            let ground_y = origin_xy.y
                + velocity_xy.y * time_to_ground
                + 0.5 * POP_GRAVITY * time_to_ground.powi(2);
            let total_bounces = rng.gen_range(2..=3);

            self.coins.push(GoldRewardCoin {
                position_xy: origin_xy,
                ground_y,
                velocity_xy,
                rotation_radians: rng.gen_range(0.0..std::f32::consts::TAU),
                angular_velocity: rng.gen_range(-180.0..=180.0),
                launch_delay_secs,
                elapsed_secs: 0.0,
                field_size_px: COIN_SIZE_PX * rng.gen_range(0.85..=1.15),
                bounces_remaining: total_bounces,
                total_bounces,
                fading: false,
                fade_elapsed_secs: 0.0,
                fade_duration_secs: rng
                    .gen_range(COIN_FADE_DURATION_MIN_SECS..=COIN_FADE_DURATION_MAX_SECS),
                sparkle_distance_remaining: rng.gen_range(0.06..=0.12),
            });
        }
    }

    pub(crate) fn tick(&mut self, presentation_delta: Duration) {
        let delta_secs = presentation_delta.as_secs_f32().clamp(0.0, 0.05);
        for sparkle in &mut self.sparkles {
            sparkle.tick(delta_secs);
        }
        self.sparkles
            .retain(|sparkle| sparkle.age_secs < sparkle.lifetime_secs);

        let mut rng = rand::thread_rng();
        for coin in &mut self.coins {
            let previous_active_secs = (coin.elapsed_secs - coin.launch_delay_secs).max(0.0);
            coin.elapsed_secs += delta_secs;
            let current_active_secs = (coin.elapsed_secs - coin.launch_delay_secs).max(0.0);
            let active_delta_secs = current_active_secs - previous_active_secs;
            if active_delta_secs <= 0.0 {
                continue;
            }

            let (impact_count, distance_travelled) = coin.advance(active_delta_secs);
            coin.sparkle_distance_remaining -= distance_travelled;
            while coin.sparkle_distance_remaining <= 0.0 && !coin.fading {
                self.sparkles.push(GoldSparkleParticle::new(
                    coin.position_xy,
                    coin.sparkle_progress(),
                    &mut rng,
                ));
                coin.sparkle_distance_remaining += rng.gen_range(0.06..=0.12);
            }
            for _ in 0..impact_count * 2 {
                self.sparkles.push(GoldSparkleParticle::new(
                    coin.position_xy,
                    coin.sparkle_progress(),
                    &mut rng,
                ));
            }
        }

        self.coins.retain(|coin| !coin.is_done());
    }

    pub(crate) fn clear(&mut self) {
        self.coins.clear();
        self.sparkles.clear();
    }

    pub(crate) fn field_particles(&self) -> impl Iterator<Item = (Xy<f32>, f32, f32, f32)> + '_ {
        self.coins.iter().filter_map(|coin| {
            let elapsed = coin.elapsed_secs - coin.launch_delay_secs;
            if elapsed < 0.0 || coin.is_done() {
                return None;
            }
            Some((
                coin.position_xy,
                coin.field_size_px,
                coin.rotation_radians,
                coin.opacity(),
            ))
        })
    }

    pub(crate) fn sparkle_sprites(&self) -> Vec<ImageSprite> {
        self.sparkles
            .iter()
            .map(GoldSparkleParticle::render)
            .collect()
    }
}

#[derive(Clone, State)]
struct GoldRewardCoin {
    position_xy: Xy<f32>,
    ground_y: f32,
    velocity_xy: Xy<f32>,
    rotation_radians: f32,
    angular_velocity: f32,
    launch_delay_secs: f32,
    elapsed_secs: f32,
    field_size_px: f32,
    bounces_remaining: u8,
    total_bounces: u8,
    fading: bool,
    fade_elapsed_secs: f32,
    fade_duration_secs: f32,
    sparkle_distance_remaining: f32,
}

impl GoldRewardCoin {
    fn advance(&mut self, mut delta_secs: f32) -> (usize, f32) {
        let mut impact_count = 0;
        let mut distance_travelled = 0.0;

        while delta_secs > 0.00001 {
            if self.fading {
                self.rotation_radians += self.angular_velocity * delta_secs;
                self.angular_velocity *= (-ROTATION_DAMPING_PER_SEC * delta_secs).exp();
                self.fade_elapsed_secs += delta_secs;
                break;
            }

            let distance_to_ground = (self.ground_y - self.position_xy.y).max(0.0);
            let time_to_ground = (-self.velocity_xy.y
                + (self.velocity_xy.y.powi(2) + 2.0 * POP_GRAVITY * distance_to_ground).sqrt())
                / POP_GRAVITY;

            if time_to_ground <= delta_secs {
                let previous_y = self.position_xy.y;
                self.position_xy.x += self.velocity_xy.x * time_to_ground;
                self.position_xy.y = self.ground_y;
                self.rotation_radians += self.angular_velocity * time_to_ground;
                self.angular_velocity *= (-ROTATION_DAMPING_PER_SEC * time_to_ground).exp();
                self.velocity_xy.y += POP_GRAVITY * time_to_ground;
                distance_travelled += ((self.velocity_xy.x * time_to_ground).powi(2)
                    + (self.ground_y - previous_y).powi(2))
                .sqrt();
                delta_secs -= time_to_ground;
                impact_count += 1;

                if self.bounces_remaining > 0 {
                    self.bounces_remaining -= 1;
                    self.velocity_xy.y *= -BOUNCE_RESTITUTION;
                    self.velocity_xy.x *= HORIZONTAL_FRICTION;
                    self.angular_velocity *= 0.72;
                } else {
                    self.fading = true;
                    self.velocity_xy = Xy::zero();
                }
            } else {
                let previous_xy = self.position_xy;
                self.position_xy.x += self.velocity_xy.x * delta_secs;
                self.position_xy.y +=
                    self.velocity_xy.y * delta_secs + 0.5 * POP_GRAVITY * delta_secs.powi(2);
                self.velocity_xy.y += POP_GRAVITY * delta_secs;
                self.rotation_radians += self.angular_velocity * delta_secs;
                self.angular_velocity *= (-ROTATION_DAMPING_PER_SEC * delta_secs).exp();
                distance_travelled += (self.position_xy - previous_xy).length();
                delta_secs = 0.0;
            }
        }

        (impact_count, distance_travelled)
    }

    fn opacity(&self) -> f32 {
        if self.fading {
            (1.0 - self.fade_elapsed_secs / self.fade_duration_secs).clamp(0.0, 1.0)
        } else {
            1.0
        }
    }

    fn is_done(&self) -> bool {
        self.fading && self.fade_elapsed_secs >= self.fade_duration_secs
    }

    fn sparkle_progress(&self) -> f32 {
        1.0 - self.bounces_remaining as f32 / self.total_bounces as f32
    }
}

#[derive(Clone, State)]
struct GoldSparkleParticle {
    position_xy: Xy<f32>,
    velocity_xy: Xy<f32>,
    age_secs: f32,
    lifetime_secs: f32,
    size_px: f32,
    glow_strength: f32,
    rotation_radians: f32,
    angular_velocity: f32,
}

impl GoldSparkleParticle {
    fn new<R: Rng + ?Sized>(position_xy: Xy<f32>, trail_progress: f32, rng: &mut R) -> Self {
        let trail_progress = trail_progress.clamp(0.0, 1.0);
        let lifetime_secs = rng.gen_range(0.35..=0.50) + trail_progress * rng.gen_range(0.25..=1.5);
        Self {
            position_xy: position_xy
                + Xy::new(rng.gen_range(-0.05..=0.05), rng.gen_range(-0.05..=0.05)),
            velocity_xy: Xy::new(rng.gen_range(-0.4..=0.4), rng.gen_range(-0.5..=0.25)),
            age_secs: 0.0,
            lifetime_secs,
            size_px: rng.gen_range(40.0..=64.0),
            glow_strength: 0.78 + trail_progress * 0.22,
            rotation_radians: rng.gen_range(0.0..std::f32::consts::TAU),
            angular_velocity: rng.gen_range(-8.0..=8.0),
        }
    }

    fn tick(&mut self, delta_secs: f32) {
        self.age_secs += delta_secs;
        self.position_xy.x += self.velocity_xy.x * delta_secs;
        self.position_xy.y += self.velocity_xy.y * delta_secs;
        self.rotation_radians += self.angular_velocity * delta_secs;
    }

    fn render(&self) -> ImageSprite {
        let progress = (self.age_secs / self.lifetime_secs).clamp(0.0, 1.0);
        let opacity = ((1.0 - progress) * self.glow_strength * 0.60 * 255.0).round() as u8;
        let center_xy = crate::game_state::TILE_PX_SIZE.to_xy() * self.position_xy;
        crate::game_state::field_particle::atlas::centered_rotated_sprite(
            crate::game_state::field_particle::atlas::sparkle(),
            center_xy.x,
            center_xy.y,
            self.size_px * (1.0 - progress) / 128.0,
            self.rotation_radians,
            Some(Color::WHITE.with_alpha(opacity)),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gold_coins_are_staggered_bounce_and_fade() {
        let mut rewards = GoldRewardPresentation::default();
        rewards.spawn(Xy::new(4.0, 5.0), 5);

        let launch_delays = rewards
            .coins
            .iter()
            .map(|coin| coin.launch_delay_secs)
            .collect::<Vec<_>>();
        assert!(launch_delays.windows(2).all(|pair| pair[0] < pair[1]));
        assert!(launch_delays.last().copied().unwrap() < MAX_COIN_SPAWN_WINDOW_SECS + 0.025);

        let mut elapsed_secs = 0.0;
        let mut saw_fade = false;
        while elapsed_secs < 6.0 && !rewards.coins.is_empty() {
            rewards.tick(Duration::from_millis(50));
            saw_fade |= rewards
                .coins
                .iter()
                .any(|coin| coin.fading && coin.opacity() < 1.0);
            elapsed_secs += 0.05;
        }

        assert!(saw_fade);
        assert!(rewards.coins.is_empty());
    }

    #[test]
    fn bounces_spawn_field_sparkles_that_shrink_and_fade() {
        let mut rewards = GoldRewardPresentation::default();
        rewards.spawn(Xy::new(4.0, 5.0), 1);
        let mut elapsed_secs = 0.0;
        let mut spawned_sparkles = false;

        while elapsed_secs < 1.5 {
            rewards.tick(Duration::from_millis(50));
            spawned_sparkles |= !rewards.sparkles.is_empty();
            elapsed_secs += 0.05;
        }

        assert!(spawned_sparkles);
        assert!(rewards.sparkles.iter().all(|sparkle| sparkle.size_px > 0.0));
    }

    #[test]
    fn gold_pop_follows_gravity_and_lands_at_its_target() {
        let mut rewards = GoldRewardPresentation::default();
        rewards.spawn(Xy::new(4.0, 5.0), 1);
        let coin = &rewards.coins[0];
        let origin_y = coin.position_xy.y;
        let initial_velocity_y = coin.velocity_xy.y;
        let landing_offset = coin.ground_y - origin_y;
        let pop_duration = (-initial_velocity_y
            + (initial_velocity_y.powi(2) + 2.0 * POP_GRAVITY * landing_offset).sqrt())
            / POP_GRAVITY;
        let midpoint = pop_duration / 2.0;
        let midpoint_y =
            origin_y + initial_velocity_y * midpoint + 0.5 * POP_GRAVITY * midpoint.powi(2);
        let landed_y =
            origin_y + initial_velocity_y * pop_duration + 0.5 * POP_GRAVITY * pop_duration.powi(2);

        assert!(origin_y - midpoint_y > 0.3);
        assert!((landed_y - coin.ground_y).abs() < 0.0001);
        assert!((2..=3).contains(&coin.bounces_remaining));
    }

    #[test]
    fn fifty_gold_coins_spawn_within_one_second() {
        let mut rewards = GoldRewardPresentation::default();
        rewards.spawn(Xy::new(4.0, 5.0), 50);
        let last_launch_delay = rewards
            .coins
            .iter()
            .map(|coin| coin.launch_delay_secs)
            .reduce(f32::max)
            .expect("fifty coins should be spawned");

        assert!(last_launch_delay < 1.0);
    }

    #[test]
    fn coin_rotation_slows_during_the_pop() {
        let mut rewards = GoldRewardPresentation::default();
        rewards.spawn(Xy::new(4.0, 5.0), 1);
        let coin = &mut rewards.coins[0];
        coin.angular_velocity = 12.0;

        coin.advance(0.15);
        let first_rotation_speed = coin.angular_velocity.abs();
        coin.advance(0.15);

        assert!(first_rotation_speed < 12.0);
        assert!(coin.angular_velocity.abs() < first_rotation_speed);
    }

    #[test]
    fn later_sparkles_live_longer_near_the_coin() {
        let mut rng = rand::thread_rng();
        let early = GoldSparkleParticle::new(Xy::zero(), 0.0, &mut rng);
        let late = GoldSparkleParticle::new(Xy::zero(), 1.0, &mut rng);

        assert!(early.lifetime_secs <= 0.50);
        assert!(late.lifetime_secs >= 0.60);
        assert!((40.0..=64.0).contains(&early.size_px));
    }
}
