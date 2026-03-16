use namui::*;

const RAY_SHARPNESS: f32 = 10.0;
const BASE_SHARPNESS: f32 = 0.5;
const THICK_BASE_SHARPNESS: f32 = 0.1;

#[derive(Debug, Clone, Copy, State)]
struct HaloState {
    last_time: Instant,
    rotation: f32,
}

/// A soft radial "rays" halo effect.
///
/// - `strength` controls both opacity and the length of the rays.
/// - `rotation_deg_per_sec` rotates the pattern over time.
/// - `radius` expands the glow beyond the core bounds.
pub struct Halo {
    /// The core bounds of the halo effect.
    pub wh: Wh<Px>,

    /// Expanded padding radius (in physical px) for glow.
    pub radius: Px,

    pub color: Color,

    /// 0..1
    pub strength: f32,

    /// Degrees per second.
    pub rotation_deg_per_sec: f32,
}

impl Component for Halo {
    fn render(self, ctx: &RenderCtx) {
        let now = Instant::now();
        let (state_sig, set_state) = ctx.state(|| HaloState {
            last_time: now,
            rotation: 0.0,
        });

        let mut state = state_sig.clone_inner();
        let dt = (now - state.last_time).as_secs_f32();
        state.last_time = now;
        state.rotation += dt * self.rotation_deg_per_sec.to_radians();
        set_state.set(state);

        let strength = self.strength.clamp(0.0, 1.0);

        let radius_px = self.radius;

        // Pad the drawn halo so the effect can extend beyond the core bounds.
        let padded_wh = Wh::new(
            self.wh.width + radius_px + radius_px,
            self.wh.height + radius_px + radius_px,
        );
        let padded_w: f32 = padded_wh.width.into();
        let padded_h: f32 = padded_wh.height.into();

        // auto density: keep a roughly consistent ray spacing across sizes
        // (bigger area -> more rays)
        let diag = (padded_w * padded_w + padded_h * padded_h).sqrt();
        let ray_count = ((diag / 40.0).round() as usize).max(8) as f32;

        let color = self.color;
        let color_r = (color.r as f32) / 255.0;
        let color_g = (color.g as f32) / 255.0;
        let color_b = (color.b as f32) / 255.0;
        let color_a = (color.a as f32) / 255.0;

        let seed1 = 0.0;
        let seed2 = 37.0;
        let seed3 = 73.0;
        let rotation1 = state.rotation;
        // Counter-rotating second layer keeps motion complex even with fewer passes.
        let rotation2 = -state.rotation * 1.2 + 0.8;
        // Slow/low-contrast base layer.
        let rotation3 = state.rotation * 0.5 + 0.3;

        let pad: f32 = radius_px.into();

        let uniforms1: [f32; 12] = [
            padded_w,
            padded_h,
            rotation1,
            ray_count,
            RAY_SHARPNESS,
            strength,
            color_r,
            color_g,
            color_b,
            color_a,
            seed1,
            pad,
        ];

        let uniforms2: [f32; 12] = [
            padded_w,
            padded_h,
            rotation2,
            ray_count,
            BASE_SHARPNESS,
            strength,
            color_r,
            color_g,
            color_b,
            color_a,
            seed2,
            pad,
        ];

        let uniforms3: [f32; 12] = [
            padded_w,
            padded_h,
            rotation3,
            ray_count,
            THICK_BASE_SHARPNESS,
            strength,
            color_r,
            color_g,
            color_b,
            color_a,
            seed3,
            pad,
        ];

        let shader1 = Shader::runtime_effect(&uniforms1, RAY_SHADER, Box::new([]));
        let paint1 = Paint::new(Color::WHITE.with_alpha(96))
            .set_shader(shader1)
            .set_blend_mode(BlendMode::Plus);

        let shader2 = Shader::runtime_effect(&uniforms2, RAY_SHADER, Box::new([]));
        let paint2 = Paint::new(Color::WHITE)
            .set_shader(shader2)
            .set_blend_mode(BlendMode::Screen);

        let shader3 = Shader::runtime_effect(&uniforms3, RAY_SHADER, Box::new([]));
        let paint3 = Paint::new(Color::WHITE)
            .set_shader(shader3)
            .set_blend_mode(BlendMode::Screen);

        // Expand rendering bounds by the radius so the glow can draw outside the core rect.
        let rect = Rect::from_xy_wh(Xy::zero(), padded_wh);
        let path = Path::new().add_rect(rect);
        let ctx = ctx.translate(Xy::single(-radius_px));

        // Layered passes (two plus + three screen). Third layer is a thick low-sharpness base.
        ctx.add(namui::path(path.clone(), paint1));
        ctx.add(namui::path(path.clone(), paint2));
        ctx.add(namui::path(path.clone(), paint3));
    }
}

const RAY_SHADER: &str = r#"
uniform float2 iResolution;
uniform float uRotation;
uniform float uRayCount;
uniform float uSharpness;
uniform float uStrength;
uniform float3 uColor;
uniform float uAlpha;
uniform float uSeed;
uniform float uPadding;

float hash(float x) {
    // cheaper hash: avoids sin() and still produces pseudo-random variation.
    x = fract(x * 0.1031);
    return fract(x * (x + 33.33));
}

float timeNoise(float segment) {
    float phase = uRotation * 0.08;
    float i = floor(phase);
    float f = fract(phase);
    float n0 = hash(segment + uSeed + i);
    float n1 = hash(segment + uSeed + i + 1.0);
    return n0 + (n1 - n0) * f;
}

float rayIntensity(float angle, float dist) {
    float norm = fract(angle / (2.0 * 3.141592653589793));
    float segF = norm * uRayCount;
    float segment = floor(segF);
    float frac = fract(segF);

    float nextSegment = mod(segment + 1.0, uRayCount);
    float noiseA = timeNoise(segment);
    float noiseB = timeNoise(nextSegment);
    float noise = mix(noiseA, noiseB, smoothstep(0.0, 1.0, frac));

    float base = sin(angle * uRayCount + uRotation * 0.18) * 0.4 + 0.6;
    float sharpness = mix(1.0, uSharpness, smoothstep(0.0, 0.35, dist));
    // Approximate pow() to avoid expensive exp operations.
    float pattern = base + (base - 0.5) * (sharpness - 1.0) * 0.25;

    float coreBlend = smoothstep(0.0, 0.25, dist);
    pattern = mix(1.0, pattern, coreBlend);

    float visibility = smoothstep(0.3, 0.7, noise);
    return pattern * visibility;
}

float fastAtan2(float y, float x) {
    // Higher‑precision atan2 approximation.
    // Based on a minimax polynomial approximation with quadrant correction.
    float ax = abs(x);
    float ay = abs(y);
    float a = min(ax, ay) / max(ax, ay);
    float s = a * a;

    // polynomial approximation for atan(a)
    float r = ((-0.0464964749 * s + 0.15931422) * s - 0.327622764) * s * a + a;

    if (ay > ax) {
        r = 1.57079632679 - r;
    }
    if (x < 0.0) {
        r = 3.14159265359 - r;
    }
    return y < 0.0 ? -r : r;
}

half4 main(float2 pos) {
    float2 coreSize = iResolution - 2.0 * uPadding;
    float2 center = float2(uPadding, uPadding) + coreSize * 0.5;
    float2 uv = (pos - center) / min(coreSize.y, coreSize.x);

    float2 uv01 = pos / iResolution;
    float edge = 0.14;
    float edgeMask = smoothstep(0.0, edge, min(min(uv01.x, uv01.y), min(1.0 - uv01.x, 1.0 - uv01.y)));

    float dist = length(uv);
    // Soften central falloff to make the core glow spread out more.
    // Use the padding/radius to influence how wide the glow spreads.
    // Larger radius => glow is allowed to bleed further from the center.
    float radiusScale = 1.0 + uPadding / min(coreSize.x, coreSize.y);
    float glowDist = pow(dist / radiusScale, 2.0);
    float angle = fastAtan2(uv.y, uv.x) + uRotation;

    const int SAMPLE_COUNT = 6;
    float maxVal = 0.0;
    for (int i = 1; i <= SAMPLE_COUNT; ++i) {
        float t = float(i) / float(SAMPLE_COUNT);
        float2 sampleUV = uv * t;
        float sampleDist = length(sampleUV);
        float sampleAngle = fastAtan2(sampleUV.y, sampleUV.x) + uRotation;
        float val = rayIntensity(sampleAngle, sampleDist);
        val *= smoothstep(1.0, 0.0, sampleDist);
        maxVal = max(maxVal, val);
    }

    float falloff = uStrength * (0.15 / max(glowDist, 0.001));
    float finalShine = maxVal * falloff;

    float alpha = clamp(finalShine * uAlpha * edgeMask, 0.0, 1.0);
    half3 color = half3(uColor) * alpha;
    return half4(color, alpha);
}
"#;
