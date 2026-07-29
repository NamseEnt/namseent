use super::palette;
use namui::*;

#[derive(Debug, Clone, Copy, State)]
struct CardHaloFxState {
    last_time: Instant,
    elapsed_seconds: f32,
}

pub struct CardHaloFx {
    pub wh: Wh<Px>,
    pub radius: Px,
    pub color: Color,
    pub strength: f32,
    pub seed: f32,
}

impl Component for CardHaloFx {
    fn render(self, ctx: &RenderCtx) {
        let now = Instant::now();
        let (state_sig, set_state) = ctx.state(|| CardHaloFxState {
            last_time: now,
            elapsed_seconds: 0.0,
        });

        let mut state = state_sig.clone_inner();
        state.elapsed_seconds += (now - state.last_time).as_secs_f32();
        state.last_time = now;
        set_state.set(state);

        let strength = self.strength.clamp(0.0, 1.0);
        let padded_wh = Wh::new(
            self.wh.width + self.radius + self.radius,
            self.wh.height + self.radius + self.radius,
        );
        let padded_w: f32 = padded_wh.width.into();
        let padded_h: f32 = padded_wh.height.into();
        let radius: f32 = self.radius.into();
        let corner_radius: f32 = palette::ROUND.into();
        let color = self.color;

        let uniforms: [f32; 13] = [
            padded_w,
            padded_h,
            state.elapsed_seconds,
            self.seed,
            strength,
            (color.r as f32) / 255.0,
            (color.g as f32) / 255.0,
            (color.b as f32) / 255.0,
            (color.a as f32) / 255.0,
            radius,
            corner_radius,
            self.seed.fract(),
            0.72 + self.seed.fract() * 0.36,
        ];

        let shader = Shader::runtime_effect(&uniforms, CARD_HALO_SHADER, Box::new([]));
        let paint1 = Paint::new(Color::WHITE.with_alpha(80))
            .set_shader(shader.clone())
            .set_blend_mode(BlendMode::Plus);
        let paint2 = Paint::new(Color::WHITE.with_alpha(120))
            .set_shader(shader)
            .set_blend_mode(BlendMode::Screen);

        let rect = Rect::from_xy_wh(Xy::zero(), padded_wh);
        let path = Path::new().add_rect(rect);

        let translated = ctx.translate(Xy::single(-self.radius));
        translated.add(namui::path(path.clone(), paint1));
        translated.add(namui::path(path.clone(), paint2));
    }
}

const CARD_HALO_SHADER: &str = r#"
uniform float2 iResolution;
uniform float uTime;
uniform float uSeed;
uniform float uStrength;
uniform float3 uColor;
uniform float uAlpha;
uniform float uPadding;
uniform float uCornerRadius;
uniform float uPhase;
uniform float uSpeed;

float hash(float value) {
    value = fract(value * 0.1031);
    return fract(value * (value + 33.33));
}

float valueNoise(float value) {
    float index = floor(value);
    float fraction = fract(value);
    float smoothFraction = fraction * fraction * (3.0 - 2.0 * fraction);
    return mix(hash(index), hash(index + 1.0), smoothFraction);
}

float sdRoundedBox(float2 point, float2 halfSize, float radius) {
    float2 offset = abs(point) - halfSize + radius;
    return min(max(offset.x, offset.y), 0.0) + length(max(offset, 0.0)) - radius;
}

float fastAtan2(float y, float x) {
    float ax = abs(x);
    float ay = abs(y);
    float ratio = min(ax, ay) / max(ax, ay);
    float square = ratio * ratio;
    float angle = ((-0.0464964749 * square + 0.15931422) * square - 0.327622764) * square * ratio + ratio;

    if (ay > ax) {
        angle = 1.57079632679 - angle;
    }
    if (x < 0.0) {
        angle = 3.14159265359 - angle;
    }
    return y < 0.0 ? -angle : angle;
}

float coneBeam(
    float angle,
    float outsideAmount,
    float time,
    float seed,
    float rayCount,
    float minWidth,
    float maxWidth,
    float minReach,
    float maxReach
) {
    float spin = time * (0.018 + hash(seed + rayCount) * 0.022);
    float coordinate = (angle / 6.28318530718 + spin + seed) * rayCount;
    float rayIndex = floor(coordinate);
    float distanceFromRayCenter = abs(fract(coordinate) - 0.5) * 2.0;
    float baseWidth = mix(minWidth, maxWidth, hash(rayIndex + seed * 37.0));
    float width = baseWidth * (0.8 + outsideAmount * 2.8);
    float shoulder = 1.0 - smoothstep(width, width * 4.5, distanceFromRayCenter);
    float core = 1.0 - smoothstep(width * 0.05, width * 0.45, distanceFromRayCenter);
    float body = shoulder * (0.45 + core * 0.55);

    float lengthTime = time * 0.19 + hash(rayIndex + seed * 11.0) * 5.0;
    float lengthStep = floor(lengthTime);
    float lengthBlend = smoothstep(0.0, 1.0, fract(lengthTime));
    float lengthNoise = mix(
        hash(rayIndex + seed * 71.0 + lengthStep),
        hash(rayIndex + seed * 71.0 + lengthStep + 1.0),
        lengthBlend
    );
    float reach = mix(minReach, maxReach, lengthNoise * 0.8);
    float tipNoise = (valueNoise(outsideAmount * 9.0 + rayIndex * 3.8 + time * 0.27) - 0.5) * 0.09;
    float tip = 1.0 - smoothstep(reach - 0.35 + tipNoise, reach + 0.22 + tipNoise, outsideAmount);
    float distanceFade = 1.0 - smoothstep(reach * 0.03, reach, outsideAmount);
    float visibility = 0.6 + 0.4 * sin(time * 1.7 + rayIndex * 1.3);
    float brightness = 0.7 + hash(rayIndex + seed * 97.0) * 0.3;

    return body * tip * distanceFade * visibility * brightness;
}

half4 main(float2 pos) {
    float2 coreSize = iResolution - 2.0 * uPadding;
    float2 center = iResolution * 0.5;
    float2 halfSize = coreSize * 0.5;
    float cornerRadius = min(uCornerRadius, min(halfSize.x, halfSize.y));
    float2 normalized = (pos - center) / min(coreSize.x, coreSize.y);
    float angle = fastAtan2(normalized.y, normalized.x);
    float time = uTime * uSpeed + uPhase * 6.28318530718;

    float boundaryDistance = sdRoundedBox(pos - center, halfSize, cornerRadius);
    float outsideAmount = clamp(max(boundaryDistance, 0.0) / max(uPadding, 1.0), 0.0, 1.0);

    // Add stronger angular noise to completely break up any rectangular boundary
    float boundaryNoise = (valueNoise(angle * 12.0 + time * 1.1) - 0.5) * 0.25 * (1.0 + outsideAmount * 3.0);
    boundaryDistance -= boundaryNoise;
    outsideAmount = clamp(max(boundaryDistance, 0.0) / max(uPadding, 1.0), 0.0, 1.0);

    float primaryBeams = coneBeam(
        angle,
        outsideAmount,
        time * 2.8,
        uSeed,
        7.0 + floor(hash(uSeed * 13.0) * 4.0),
        0.26,
        0.52,
        0.35,
        0.94
    );
    float secondaryBeams = coneBeam(
        angle,
        outsideAmount,
        time * 1.9,
        uSeed + 0.53,
        14.0 + floor(hash(uSeed * 29.0) * 6.0),
        0.06,
        0.17,
        0.48,
        0.79
    );
    float bloom = (1.0 - smoothstep(0.0, 0.85, outsideAmount)) * (0.65 + valueNoise(angle * 4.1 + time * 0.08 + uSeed * 17.0) * 0.35);
    float rim = 1.0 - smoothstep(0.0, 6.0, abs(boundaryDistance));
    float globalPulse = 0.88 + 0.12 * sin(time * 0.53 + uSeed * 13.0);
    float outerSoft = 1.0 - smoothstep(0.15, 2.8, outsideAmount);

    // Local brightness from rays modulates alpha (higher on ray side, lower elsewhere) to hide boundary
    float localRayBrightness = primaryBeams * 1.1 + secondaryBeams * 0.6 + bloom * 0.4;
    float rayModulatedAlpha = localRayBrightness * 0.65 + 0.35;

    // Strong radial + corner vignette to make any geometric shape completely imperceptible
    float radialFalloff = 1.0 - length(normalized) * 0.75;
    float hardOuterCut = 1.0 - smoothstep(0.88, 1.12, outsideAmount);

    float alpha = (primaryBeams * 0.65 + secondaryBeams * 0.25 + bloom * 0.45 + rim * 0.06 + outerSoft * 0.38)
        * rayModulatedAlpha
        * radialFalloff
        * hardOuterCut
        * globalPulse
        * uStrength
        * uAlpha;

    return half4(half3(uColor) * alpha, alpha);
}
"#;
