use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ShakeIntensity {
    Light,
    Medium,
    Heavy,
}

impl ShakeIntensity {
    pub fn value(&self) -> f32 {
        match self {
            ShakeIntensity::Light => 10.0,
            ShakeIntensity::Medium => 15.0,
            ShakeIntensity::Heavy => 30.0,
        }
    }
}

impl Camera {
    pub fn shake(&mut self, intensity: ShakeIntensity) {
        self.shake_intensity += intensity.value();
    }

    pub fn update_shake(&mut self, dt: Duration, elapsed: Duration) {
        let decay_rate = 10.0; // 높을수록 빠르게 감소
        let decay = (-decay_rate * dt.as_secs_f32()).exp();
        self.shake_intensity *= decay;

        if self.shake_intensity < 0.001 {
            self.shake_intensity = 0.0;
            self.shake_offset = Xy::new(0.0, 0.0);
            return;
        }

        let shake_offset_px = get_camera_shake_offset(elapsed, self.shake_intensity);
        self.shake_offset = Xy::new(
            shake_offset_px.x / TILE_PX_SIZE.width,
            shake_offset_px.y / TILE_PX_SIZE.height,
        );
    }
}

// Perlin-like noise function for smooth random values
fn noise_2d(x: f32, y: f32) -> f32 {
    let n = (x * 12.9898 + y * 78.233).sin() * 43_758.547;
    n - n.floor()
}

// Get shake offset based on duration since start and intensity
fn get_camera_shake_offset(elapsed: Duration, intensity: f32) -> Xy<Px> {
    if intensity <= 0.001 {
        return Xy::new(0.px(), 0.px());
    }

    // Use elapsed time for deterministic noise
    let time_value = elapsed.as_secs_f32();

    // Use fast noise functions for x and y offsets
    // High frequency for jittery effect
    let x_offset = (noise_2d(time_value * 50.0, 0.0) - 0.5) * 2.0 * intensity;
    let y_offset = (noise_2d(0.0, time_value * 50.0) - 0.5) * 2.0 * intensity;

    Xy::new(x_offset.px(), y_offset.px())
}
