use namui::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

const TORN_BORDER_RING_WIDTH: f32 = 2.0;
const TORN_INNER_NOISE_SEED: u64 = 137;
const TORN_OUTER_NOISE_SEED: u64 = 337;
const TORN_OUTER_ROUGHNESS_SEED: u64 = 7337;
const TORN_OUTER_EXTRA_ROUGHNESS: f32 = 2.0;
const SMOOTH_EDGE_STEP: f32 = 8.0;
const SMOOTH_LARGE_WAVE_PERIOD: f32 = 96.0;
const SMOOTH_SMALL_WAVE_PERIOD: f32 = 32.0;
const SMOOTH_LARGE_WAVE_AMPLITUDE: f32 = 2.5;
const SMOOTH_SMALL_WAVE_AMPLITUDE: f32 = 1.0;

pub(super) fn dual_layer_torn_paper_paths(width: Px, height: Px) -> (Path, Path) {
    let mut inner_rng = StdRng::seed_from_u64(TORN_INNER_NOISE_SEED);
    let mut outer_rng = StdRng::seed_from_u64(TORN_OUTER_NOISE_SEED);
    let mut rough_rng = StdRng::seed_from_u64(TORN_OUTER_ROUGHNESS_SEED);

    let w = width.as_f32();
    let h = height.as_f32();
    let ring = TORN_BORDER_RING_WIDTH;
    let base_amplitude = SMOOTH_LARGE_WAVE_AMPLITUDE * 1.5;

    let top_positions = axis_positions(w, false);
    let right_positions = axis_positions(h, false);
    let bottom_positions = axis_positions(w, true);
    let left_positions = axis_positions(h, true);

    let top_inner_offsets =
        smooth_noise_offsets(w, base_amplitude, TORN_INNER_NOISE_SEED, &mut inner_rng);
    let right_inner_offsets = smooth_noise_offsets(
        h,
        base_amplitude,
        TORN_INNER_NOISE_SEED.wrapping_add(1),
        &mut inner_rng,
    );
    let bottom_inner_offsets = smooth_noise_offsets(
        w,
        base_amplitude,
        TORN_INNER_NOISE_SEED.wrapping_add(2),
        &mut inner_rng,
    );
    let left_inner_offsets = smooth_noise_offsets(
        h,
        base_amplitude,
        TORN_INNER_NOISE_SEED.wrapping_add(3),
        &mut inner_rng,
    );

    let top_outer_offsets =
        smooth_noise_offsets(w, base_amplitude, TORN_OUTER_NOISE_SEED, &mut outer_rng);
    let right_outer_offsets = smooth_noise_offsets(
        h,
        base_amplitude,
        TORN_OUTER_NOISE_SEED.wrapping_add(1),
        &mut outer_rng,
    );
    let bottom_outer_offsets = smooth_noise_offsets(
        w,
        base_amplitude,
        TORN_OUTER_NOISE_SEED.wrapping_add(2),
        &mut outer_rng,
    );
    let left_outer_offsets = smooth_noise_offsets(
        h,
        base_amplitude,
        TORN_OUTER_NOISE_SEED.wrapping_add(3),
        &mut outer_rng,
    );

    let top_inner = edge_points_from_offsets(&top_positions, &top_inner_offsets, Xy::new);
    let right_inner = edge_points_from_offsets(&right_positions, &right_inner_offsets, |y, off| {
        Xy::new(w + off, y)
    });
    let bottom_inner =
        edge_points_from_offsets(&bottom_positions, &bottom_inner_offsets, |x, off| {
            Xy::new(x, h + off)
        });
    let left_inner = edge_points_from_offsets(&left_positions, &left_inner_offsets, |y, off| {
        Xy::new(off, y)
    });

    let top_outer = edge_points_from_clamped_offsets(
        &top_positions,
        &top_inner_offsets,
        &top_outer_offsets,
        &mut rough_rng,
        Xy::new,
        |inner, outer| outer.min(inner - ring),
    );
    let right_outer = edge_points_from_clamped_offsets(
        &right_positions,
        &right_inner_offsets,
        &right_outer_offsets,
        &mut rough_rng,
        |y, off| Xy::new(w + off, y),
        |inner, outer| outer.max(inner + ring),
    );
    let bottom_outer = edge_points_from_clamped_offsets(
        &bottom_positions,
        &bottom_inner_offsets,
        &bottom_outer_offsets,
        &mut rough_rng,
        |x, off| Xy::new(x, h + off),
        |inner, outer| outer.max(inner + ring),
    );
    let left_outer = edge_points_from_clamped_offsets(
        &left_positions,
        &left_inner_offsets,
        &left_outer_offsets,
        &mut rough_rng,
        |y, off| Xy::new(off, y),
        |inner, outer| outer.min(inner - ring),
    );

    let inner_path =
        build_smooth_path_from_edges([top_inner, right_inner, bottom_inner, left_inner]);
    let outer_path =
        build_smooth_path_from_edges([top_outer, right_outer, bottom_outer, left_outer]);

    (inner_path, outer_path)
}

fn axis_positions(length: f32, reverse: bool) -> Vec<f32> {
    let count = (length / SMOOTH_EDGE_STEP).ceil() as usize + 1;
    (0..count)
        .map(|i| {
            let p = (i as f32 * SMOOTH_EDGE_STEP).min(length);
            if reverse { length - p } else { p }
        })
        .collect()
}

fn edge_points_from_offsets(
    positions: &[f32],
    offsets: &[f32],
    mut map: impl FnMut(f32, f32) -> Xy<f32>,
) -> Vec<Xy<f32>> {
    positions
        .iter()
        .zip(offsets.iter())
        .map(|(&position, &offset)| map(position, offset))
        .collect()
}

fn edge_points_from_clamped_offsets(
    positions: &[f32],
    inner_offsets: &[f32],
    outer_offsets: &[f32],
    rough_rng: &mut StdRng,
    mut map: impl FnMut(f32, f32) -> Xy<f32>,
    mut clamp: impl FnMut(f32, f32) -> f32,
) -> Vec<Xy<f32>> {
    positions
        .iter()
        .zip(inner_offsets.iter())
        .zip(outer_offsets.iter())
        .map(|((&position, &inner), &outer)| {
            let rough =
                rough_rng.gen_range(-TORN_OUTER_EXTRA_ROUGHNESS..=TORN_OUTER_EXTRA_ROUGHNESS);
            map(position, clamp(inner, outer + rough))
        })
        .collect()
}

fn smooth_noise_offsets(length: f32, amplitude: f32, seed: u64, rng: &mut impl Rng) -> Vec<f32> {
    let step = SMOOTH_EDGE_STEP;
    let count = (length / step).ceil() as usize + 1;

    let large_phase: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let small_phase: f32 = rng.gen_range(0.0..std::f32::consts::TAU);
    let jitter_scale = amplitude * 0.15;

    let large_amp = SMOOTH_LARGE_WAVE_AMPLITUDE * amplitude / SMOOTH_LARGE_WAVE_AMPLITUDE.max(1.0);
    let small_amp = SMOOTH_SMALL_WAVE_AMPLITUDE * amplitude / SMOOTH_LARGE_WAVE_AMPLITUDE.max(1.0);
    let freq_var = 1.0 + (seed as f32 * 0.0137).sin() * 0.2;

    (0..count)
        .map(|i| {
            let t = (i as f32 * step).min(length);
            let large = large_amp
                * (t * std::f32::consts::TAU / SMOOTH_LARGE_WAVE_PERIOD * freq_var + large_phase)
                    .sin();
            let small = small_amp
                * (t * std::f32::consts::TAU / SMOOTH_SMALL_WAVE_PERIOD * freq_var + small_phase)
                    .sin();
            let jitter = rng.gen_range(-jitter_scale..=jitter_scale);
            large + small + jitter
        })
        .collect()
}

fn build_smooth_path_from_edges(edges: [Vec<Xy<f32>>; 4]) -> Path {
    let mut all_points: Vec<Xy<f32>> = Vec::new();
    for (i, edge) in edges.iter().enumerate() {
        if i == 0 {
            all_points.extend_from_slice(edge);
        } else {
            all_points.extend_from_slice(&edge[1..]);
        }
    }

    if all_points.len() < 2 {
        return Path::new();
    }

    let points: Vec<Xy<Px>> = all_points
        .into_iter()
        .map(|xy| Xy::new(px(xy.x), px(xy.y)))
        .collect();
    Path::new().add_poly(&points, true)
}
