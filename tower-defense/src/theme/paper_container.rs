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
const SHADOW_OFFSET_Y: Px = px(2.0);
const SHADOW_ALPHA: u8 = 192;
const OFFSET_AMPLITUDE_MIN_SCALE: f32 = 0.25;
const OFFSET_AMPLITUDE_MAX_SCALE: f32 = 1.0;
const STEP_JITTER_MIN_SCALE: f32 = 0.7;
const STEP_JITTER_MAX_SCALE: f32 = 1.3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub enum PaperTexture {
    Rough,
    Crumpled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, State)]
pub enum TearSide {
    Torn,
    Subtle,
}

impl PaperTexture {
    fn image(self) -> Image {
        match self {
            PaperTexture::Rough => crate::asset::image::ui::paper::PAPER_00,
            PaperTexture::Crumpled => crate::asset::image::ui::paper::PAPER_01,
        }
    }
}

pub struct PaperContainerBackground {
    pub width: Px,
    pub height: Px,
    pub texture: PaperTexture,
    pub tear_side: TearSide,
    pub color: Color,
    pub shadow: bool,
}

impl Component for PaperContainerBackground {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            width,
            height,
            texture,
            tear_side,
            color,
            shadow,
        } = self;
        let tracked_path_inputs = ctx.track_eq(&(width, height, tear_side));
        let path = ctx.memo(|| {
            torn_paper_path(
                tracked_path_inputs.0,
                tracked_path_inputs.1,
                tracked_path_inputs.2,
            )
        });

        let paint = Paint::new(Color::WHITE)
            .set_style(PaintStyle::Fill)
            .set_shader(Shader::Image {
                src: texture.image(),
                tile_mode: Xy::single(TileMode::Repeat),
            })
            .set_color_filter(ColorFilter::Blend {
                color,
                blend_mode: BlendMode::Modulate,
            });

        ctx.add(namui::path(path.as_ref().clone(), paint));

        if shadow {
            let shadow_path = path.as_ref().clone().translate(px(0.0), SHADOW_OFFSET_Y);
            let shadow_paint = Paint::new(Color::BLACK.with_alpha(SHADOW_ALPHA))
                .set_style(PaintStyle::Fill)
                .set_mask_filter(MaskFilter::Blur {
                    blur_style: BlurStyle::Normal,
                    sigma: 2.5,
                });
            ctx.add(namui::path(shadow_path, shadow_paint));
        }
    }
}

fn torn_paper_path(width: Px, height: Px, tear_side: TearSide) -> Path {
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
    let right_points = edge_points(
        px(0.0),
        height,
        vertical_edge_step,
        |y, offset| Xy::new(width + offset, y),
        vertical_edge_displacement,
        &mut rng,
    );
    let bottom_points = edge_points(
        width,
        px(0.0),
        HORIZONTAL_EDGE_STEP,
        |x, offset| Xy::new(x, height + offset),
        HORIZONTAL_EDGE_DISPLACEMENT,
        &mut rng,
    );
    let left_points = edge_points(
        height,
        px(0.0),
        vertical_edge_step,
        |y, offset| Xy::new(offset, y),
        vertical_edge_displacement,
        &mut rng,
    );

    let mut points = Vec::new();
    points.extend(top_points);
    points.extend(right_points.into_iter().skip(1));
    points.extend(bottom_points.into_iter().skip(1));
    points.extend(left_points.into_iter().skip(1));

    Path::new().add_poly(&points, true)
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
