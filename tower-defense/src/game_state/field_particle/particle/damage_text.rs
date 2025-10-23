use namui::*;

#[derive(Clone, State)]
pub struct DamageTextParticle {
    pub position: Xy<Px>,
    pub display_value: String,
    pub display_color: Color,
    pub created_at: Instant,
    pub duration: Duration,
    pub opacity: u8,
}

impl DamageTextParticle {
    pub fn new(position: Xy<Px>, damage_value: f32, now: Instant) -> Self {
        let display_value = Self::format_display_value(damage_value);
        let display_color = Self::calculate_display_color(damage_value);
        Self {
            position,
            display_value,
            display_color,
            created_at: now,
            duration: Duration::from_millis(800),
            opacity: 255,
        }
    }
    pub fn is_done(&self, now: Instant) -> bool {
        now - self.created_at >= self.duration
    }
    fn format_display_value(damage_value: f32) -> String {
        let abs_value = damage_value.abs();
        if abs_value >= 1_000_000_000.0 {
            format!("{:.1}b", damage_value / 1_000_000_000.0)
        } else if abs_value >= 1_000_000.0 {
            format!("{:.1}m", damage_value / 1_000_000.0)
        } else if abs_value >= 1_000.0 {
            format!("{:.1}k", damage_value / 1_000.0)
        } else {
            format!("{damage_value:.0}")
        }
    }
    pub fn tick(&mut self, now: Instant, delta_time: Duration) {
        let elapsed = now - self.created_at;
        let progress = (elapsed.as_secs_f32() / self.duration.as_secs_f32()).clamp(0.0, 1.0);
        self.position.y -= px(24.0 * delta_time.as_secs_f32() / self.duration.as_secs_f32());
        self.opacity = (255.0 * (1.0 - progress)).round().clamp(0.0, 255.0) as u8;
    }
    pub fn render(&self) -> RenderingTree {
        let opacity = self.opacity;
        let style = TextStyle {
            border: Some(TextStyleBorder {
                color: Color::BLACK.with_alpha(opacity),
                width: 4.0.into(),
            }),
            color: self.display_color.with_alpha(opacity),
            ..Default::default()
        };
        namui::translate(
            self.position.x,
            self.position.y,
            namui::text(namui::TextParam {
                text: self.display_value.clone(),
                x: 0.px(),
                y: 0.px(),
                align: namui::TextAlign::Center,
                baseline: namui::TextBaseline::Middle,
                font: namui::Font {
                    name: crate::theme::typography::HEADLINE_FONT_NAME.to_string(),
                    size: int_px(64),
                },
                style,
                max_width: None,
            }),
        )
    }
    fn calculate_display_color(damage_value: f32) -> Color {
        const YELLOW_THRESHOLD: f32 = 2000.0;
        const RED_THRESHOLD: f32 = 10000.0;
        let (r, g, b) = if damage_value < YELLOW_THRESHOLD {
            let t = (damage_value / YELLOW_THRESHOLD).clamp(0.0, 1.0);
            let r = (255.0 * t + 255.0 * (1.0 - t)).round() as u8;
            let g = (220.0 * t + 255.0 * (1.0 - t)).round() as u8;
            let b = (40.0 * t + 255.0 * (1.0 - t)).round() as u8;
            (r, g, b)
        } else if damage_value < RED_THRESHOLD {
            let t = ((damage_value - YELLOW_THRESHOLD) / (RED_THRESHOLD - YELLOW_THRESHOLD))
                .clamp(0.0, 1.0);
            let r = 255u8;
            let g = (40.0 * t + 220.0 * (1.0 - t)).round() as u8;
            let b = (40.0 * t + 40.0 * (1.0 - t)).round() as u8;
            (r, g, b)
        } else {
            (255u8, 40u8, 40u8)
        };
        Color::from_u8(r, g, b, 255)
    }
}
