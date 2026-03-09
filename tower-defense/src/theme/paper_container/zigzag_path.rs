use super::{ArrowSide, PaperArrow};
use namui::*;
use rand::Rng;

const SIDE_EDGE_DYNAMIC_MIN_HEIGHT: Px = px(32.0);
const SIDE_EDGE_DYNAMIC_MAX_HEIGHT: Px = px(128.0);
const SIDE_EDGE_DISPLACEMENT_AT_MIN_HEIGHT: Px = px(2.0);
const SIDE_EDGE_DISPLACEMENT_AT_MAX_HEIGHT: Px = px(8.0);
const HORIZONTAL_EDGE_DISPLACEMENT: Px = px(2.0);
const SIDE_EDGE_STEP_AT_MIN_HEIGHT: Px = px(4.0);
const SIDE_EDGE_STEP_AT_MAX_HEIGHT: Px = px(8.0);
const SIDE_EDGE_SUBTLE_STEP: Px = px(96.0);
const HORIZONTAL_EDGE_STEP: Px = px(96.0);
const OFFSET_AMPLITUDE_MIN_SCALE: f32 = 0.25;
const OFFSET_AMPLITUDE_MAX_SCALE: f32 = 1.0;
const STEP_JITTER_MIN_SCALE: f32 = 0.7;
const STEP_JITTER_MAX_SCALE: f32 = 1.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub(super) enum TearSide {
    Torn,
    Subtle,
}

pub(super) fn torn_paper_path(
    width: Px,
    height: Px,
    tear_side: TearSide,
    arrow: Option<PaperArrow>,
) -> Path {
    let mut rng = rand::thread_rng();
    let torn_displacement = side_edge_displacement_for_height(height);
    let torn_step = side_edge_step_for_height(height);
    let (vertical_edge_displacement, vertical_edge_step) = match tear_side {
        TearSide::Torn => (torn_displacement, torn_step),
        TearSide::Subtle => (HORIZONTAL_EDGE_DISPLACEMENT, SIDE_EDGE_SUBTLE_STEP),
    };
    let top_points = edge_points(
        px(0.0),
        width,
        HORIZONTAL_EDGE_STEP,
        Xy::new,
        HORIZONTAL_EDGE_DISPLACEMENT,
        &mut rng,
    );
    let top_points = inject_top_arrow(top_points, width, height, arrow);

    let right_points = edge_points(
        px(0.0),
        height,
        vertical_edge_step,
        |y, offset| Xy::new(width + offset, y),
        vertical_edge_displacement,
        &mut rng,
    );
    let right_points = inject_right_arrow(right_points, width, height, arrow);

    let bottom_points = edge_points(
        width,
        px(0.0),
        HORIZONTAL_EDGE_STEP,
        |x, offset| Xy::new(x, height + offset),
        HORIZONTAL_EDGE_DISPLACEMENT,
        &mut rng,
    );
    // bottom edge is generated in decreasing-x order; reverse for injection then restore
    let bottom_points = {
        let mut pts = bottom_points;
        pts.reverse();
        pts = inject_bottom_arrow(pts, width, height, arrow);
        pts.reverse();
        pts
    };

    let left_points = edge_points(
        height,
        px(0.0),
        vertical_edge_step,
        |y, offset| Xy::new(offset, y),
        vertical_edge_displacement,
        &mut rng,
    );
    // left edge comes top-to-bottom decreasing; do same reversal trick
    let left_points = {
        let mut pts = left_points;
        pts.reverse();
        pts = inject_left_arrow(pts, width, height, arrow);
        pts.reverse();
        pts
    };

    let mut points = Vec::new();
    points.extend(top_points);
    points.extend(right_points.into_iter().skip(1));
    points.extend(bottom_points.into_iter().skip(1));
    points.extend(left_points.into_iter().skip(1));

    Path::new().add_poly(&points, true)
}

fn inject_right_arrow(
    right_points: Vec<Xy<Px>>,
    width: Px,
    height: Px,
    arrow: Option<PaperArrow>,
) -> Vec<Xy<Px>> {
    let Some(arrow) = arrow else {
        return right_points;
    };
    if !matches!(arrow.side, ArrowSide::Right) {
        return right_points;
    }

    let half = arrow.height / 2.0;
    let start_y = (arrow.offset - half).max(0.px()).min(height);
    let tip_y = arrow.offset.max(0.px()).min(height);
    let end_y = (arrow.offset + half).max(0.px()).min(height);

    let mut out = Vec::with_capacity(right_points.len() + 3);
    let mut inserted = false;

    for point in right_points {
        if point.y < start_y {
            out.push(point);
            continue;
        }

        if point.y > end_y {
            if !inserted {
                out.push(Xy::new(width, start_y));
                out.push(Xy::new(width + arrow.width, tip_y));
                out.push(Xy::new(width, end_y));
                inserted = true;
            }
            out.push(point);
        }
    }

    if !inserted {
        out.push(Xy::new(width, start_y));
        out.push(Xy::new(width + arrow.width, tip_y));
        out.push(Xy::new(width, end_y));
    }

    out
}

fn inject_left_arrow(
    left_points: Vec<Xy<Px>>,
    _width: Px,
    height: Px,
    arrow: Option<PaperArrow>,
) -> Vec<Xy<Px>> {
    let Some(arrow) = arrow else {
        return left_points;
    };
    if !matches!(arrow.side, ArrowSide::Left) {
        return left_points;
    }

    let half = arrow.height / 2.0;
    let start_y = (arrow.offset - half).max(0.px()).min(height);
    let tip_y = arrow.offset.max(0.px()).min(height);
    let end_y = (arrow.offset + half).max(0.px()).min(height);

    let mut out = Vec::with_capacity(left_points.len() + 3);
    let mut inserted = false;

    for point in left_points {
        if point.y < start_y {
            out.push(point);
            continue;
        }

        if point.y > end_y {
            if !inserted {
                out.push(Xy::new(0.px(), start_y));
                out.push(Xy::new(-arrow.width, tip_y));
                out.push(Xy::new(0.px(), end_y));
                inserted = true;
            }
            out.push(point);
        }
    }

    if !inserted {
        out.push(Xy::new(0.px(), start_y));
        out.push(Xy::new(-arrow.width, tip_y));
        out.push(Xy::new(0.px(), end_y));
    }

    out
}

fn inject_top_arrow(
    top_points: Vec<Xy<Px>>,
    width: Px,
    _height: Px,
    arrow: Option<PaperArrow>,
) -> Vec<Xy<Px>> {
    let Some(arrow) = arrow else {
        return top_points;
    };
    if !matches!(arrow.side, ArrowSide::Top) {
        return top_points;
    }

    let half = arrow.width / 2.0;
    let start_x = (arrow.offset - half).max(0.px()).min(width);
    let tip_x = arrow.offset.max(0.px()).min(width);
    let end_x = (arrow.offset + half).max(0.px()).min(width);

    let mut out = Vec::with_capacity(top_points.len() + 3);
    let mut inserted = false;

    for point in top_points {
        if point.x < start_x {
            out.push(point);
            continue;
        }

        if point.x > end_x {
            if !inserted {
                out.push(Xy::new(start_x, 0.px()));
                out.push(Xy::new(tip_x, -arrow.height));
                out.push(Xy::new(end_x, 0.px()));
                inserted = true;
            }
            out.push(point);
        }
    }

    if !inserted {
        out.push(Xy::new(start_x, 0.px()));
        out.push(Xy::new(tip_x, -arrow.height));
        out.push(Xy::new(end_x, 0.px()));
    }

    out
}

fn inject_bottom_arrow(
    bottom_points: Vec<Xy<Px>>,
    width: Px,
    height: Px,
    arrow: Option<PaperArrow>,
) -> Vec<Xy<Px>> {
    let Some(arrow) = arrow else {
        return bottom_points;
    };
    if !matches!(arrow.side, ArrowSide::Bottom) {
        return bottom_points;
    }

    let half = arrow.width / 2.0;
    let start_x = (arrow.offset - half).max(0.px()).min(width);
    let tip_x = arrow.offset.max(0.px()).min(width);
    let end_x = (arrow.offset + half).max(0.px()).min(width);

    let mut out = Vec::with_capacity(bottom_points.len() + 3);
    let mut inserted = false;

    for point in bottom_points {
        if point.x < start_x {
            out.push(point);
            continue;
        }

        if point.x > end_x {
            if !inserted {
                out.push(Xy::new(start_x, height));
                out.push(Xy::new(tip_x, height + arrow.height));
                out.push(Xy::new(end_x, height));
                inserted = true;
            }
            out.push(point);
        }
    }

    if !inserted {
        out.push(Xy::new(start_x, height));
        out.push(Xy::new(tip_x, height + arrow.height));
        out.push(Xy::new(end_x, height));
    }

    out
}

fn edge_points(
    start_value: Px,
    end_value: Px,
    step: Px,
    mut point_from_position: impl FnMut(Px, Px) -> Xy<Px>,
    displacement: Px,
    rng: &mut impl Rng,
) -> Vec<Xy<Px>> {
    let mut points = Vec::new();
    let mut index = 0;

    if start_value <= end_value {
        let mut edge_value = start_value;
        while edge_value < end_value {
            points.push(point_from_position(
                edge_value,
                zigzag_offset(index, displacement, rng),
            ));
            edge_value += randomized_step(step, rng);
            index += 1;
        }
        points.push(point_from_position(
            end_value,
            zigzag_offset(index, displacement, rng),
        ));
    } else {
        let mut edge_value = start_value;
        while edge_value > end_value {
            points.push(point_from_position(
                edge_value,
                zigzag_offset(index, displacement, rng),
            ));
            edge_value -= randomized_step(step, rng);
            index += 1;
        }
        points.push(point_from_position(
            end_value,
            zigzag_offset(index, displacement, rng),
        ));
    }

    points
}

fn zigzag_offset(index: usize, displacement: Px, rng: &mut impl Rng) -> Px {
    let sign = if index.is_multiple_of(2) { 1.0 } else { -1.0 };
    let amplitude_scale = rng.gen_range(OFFSET_AMPLITUDE_MIN_SCALE..=OFFSET_AMPLITUDE_MAX_SCALE);
    displacement * sign * amplitude_scale
}

fn randomized_step(step: Px, rng: &mut impl Rng) -> Px {
    let step_scale = rng.gen_range(STEP_JITTER_MIN_SCALE..=STEP_JITTER_MAX_SCALE);
    step * step_scale
}

fn side_edge_displacement_for_height(height: Px) -> Px {
    let height = height.as_f32();
    let min_height = SIDE_EDGE_DYNAMIC_MIN_HEIGHT.as_f32();
    let max_height = SIDE_EDGE_DYNAMIC_MAX_HEIGHT.as_f32();
    let min_displacement = SIDE_EDGE_DISPLACEMENT_AT_MIN_HEIGHT.as_f32();
    let max_displacement = SIDE_EDGE_DISPLACEMENT_AT_MAX_HEIGHT.as_f32();

    if height <= min_height {
        return SIDE_EDGE_DISPLACEMENT_AT_MIN_HEIGHT;
    }
    if height >= max_height {
        return SIDE_EDGE_DISPLACEMENT_AT_MAX_HEIGHT;
    }

    let t = (height - min_height) / (max_height - min_height);
    px(min_displacement + (max_displacement - min_displacement) * t)
}

fn side_edge_step_for_height(height: Px) -> Px {
    let height = height.as_f32();
    let min_height = SIDE_EDGE_DYNAMIC_MIN_HEIGHT.as_f32();
    let max_height = SIDE_EDGE_DYNAMIC_MAX_HEIGHT.as_f32();
    let min_step = SIDE_EDGE_STEP_AT_MIN_HEIGHT.as_f32();
    let max_step = SIDE_EDGE_STEP_AT_MAX_HEIGHT.as_f32();

    if height <= min_height {
        return SIDE_EDGE_STEP_AT_MIN_HEIGHT;
    }
    if height >= max_height {
        return SIDE_EDGE_STEP_AT_MAX_HEIGHT;
    }

    let t = (height - min_height) / (max_height - min_height);
    px(min_step + (max_step - min_step) * t)
}
