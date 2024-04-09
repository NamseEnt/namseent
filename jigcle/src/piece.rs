use namui::*;

#[component]
pub struct Piece {
    pub wh: Wh<Px>,
    pub piece_index: Xy<usize>,
    pub ltrb_edge: Ltrb<Edge>,
    pub image: ImageSource,
    pub image_wh: Wh<Px>,
    pub piece_state: PieceState,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceState {
    None,
    Dragging,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Edge {
    In,
    Out,
    Straight,
}

impl Component for Piece {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            piece_index,
            ltrb_edge,
            image,
            image_wh,
            piece_state,
        } = self;
        let wh = ctx.track_eq(&wh);
        let ltrb_edge = ctx.track_eq(&ltrb_edge);
        let piece_index = ctx.track_eq(&piece_index);
        let clip_path = ctx.memo(|| {
            create_piece_clip_path(*wh, *ltrb_edge)
                .translate(wh.width * piece_index.x, wh.height * piece_index.y)
        });

        let paint = match piece_state {
            PieceState::None => None,
            PieceState::Dragging => Some(Paint::new(Color::WHITE).set_color_filter(ColorFilter {
                color: Color::grayscale_f01(0.5),
                blend_mode: BlendMode::Lighten,
            })),
        };

        ctx.translate((-wh.as_xy()) * *piece_index)
            .add(namui::path(
                clip_path.clone(),
                Paint::new(Color::BLACK)
                    .set_style(PaintStyle::Stroke)
                    .set_stroke_width(2.px())
                    .set_anti_alias(true),
            ))
            .clip(clip_path.clone(), ClipOp::Intersect)
            .add(ImageDrawCommand {
                rect: Rect::zero_wh(image_wh),
                source: image.clone(),
                fit: ImageFit::Contain,
                paint,
            });
    }
}

pub fn create_piece_clip_path(piece_wh: Wh<Px>, ltrb_edge: Ltrb<Edge>) -> Path {
    let mut clip_path = Path::new();

    clip_path = line_piece_part_cw(clip_path, piece_wh, Side::Top, ltrb_edge.top);
    clip_path = line_piece_part_cw(clip_path, piece_wh, Side::Right, ltrb_edge.right);
    clip_path = line_piece_part_cw(clip_path, piece_wh, Side::Bottom, ltrb_edge.bottom);
    clip_path = line_piece_part_cw(clip_path, piece_wh, Side::Left, ltrb_edge.left);

    clip_path.close()
}

enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

fn line_piece_part_cw(mut path: Path, piece_wh: Wh<Px>, side: Side, edge: Edge) -> Path {
    const SHOULDER_WIDTH: f32 = 0.37;
    const NECK: Wh<f32> = Wh::new(0.03, 0.05);
    const HEAD_RADIUS: Wh<f32> = Wh::new(0.12, 0.15);

    const TOP_CONTROL_POINTS: [[Xy<f32>; 3]; 6] = [
        [
            Xy::new(0.0, 0.0),
            Xy::new(0.0, 0.0),
            Xy::new(SHOULDER_WIDTH, 0.0),
        ],
        [
            Xy::new(SHOULDER_WIDTH, 0.0),
            Xy::new(SHOULDER_WIDTH + NECK.width, 0.0),
            Xy::new(SHOULDER_WIDTH + NECK.width / 2.0, -NECK.height),
        ],
        [
            Xy::new(SHOULDER_WIDTH + NECK.width / 2.0, -NECK.height),
            Xy::new(
                0.5 - HEAD_RADIUS.width,
                -NECK.height - HEAD_RADIUS.height * 2.0,
            ),
            Xy::new(0.5, -NECK.height - HEAD_RADIUS.height * 2.0),
        ],
        [
            Xy::new(0.5, -NECK.height - HEAD_RADIUS.height * 2.0),
            Xy::new(
                0.5 + HEAD_RADIUS.width,
                -NECK.height - HEAD_RADIUS.height * 2.0,
            ),
            Xy::new(1.0 - (SHOULDER_WIDTH + NECK.width / 2.0), -NECK.height),
        ],
        [
            Xy::new(1.0 - (SHOULDER_WIDTH + NECK.width / 2.0), -NECK.height),
            Xy::new(1.0 - (SHOULDER_WIDTH + NECK.width), 0.0),
            Xy::new(1.0 - SHOULDER_WIDTH, 0.0),
        ],
        [
            Xy::new(1.0 - SHOULDER_WIDTH, 0.0),
            Xy::new(1.0, 0.0),
            Xy::new(1.0, 0.0),
        ],
    ];

    let sign = match side {
        Side::Top => match edge {
            Edge::Straight => return path.line_to(piece_wh.width, 0.px()),
            Edge::In => Xy::new(1.0, 1.0),
            Edge::Out => Xy::new(1.0, -1.0),
        },
        Side::Right => match edge {
            Edge::Straight => return path.line_to(piece_wh.width, piece_wh.height),
            Edge::In => Xy::new(-1.0, 1.0),
            Edge::Out => Xy::new(1.0, 1.0),
        },
        Side::Bottom => match edge {
            Edge::Straight => return path.line_to(0.px(), piece_wh.height),
            Edge::In => Xy::new(1.0, -1.0),
            Edge::Out => Xy::new(1.0, 1.0),
        },
        Side::Left => match edge {
            Edge::Straight => return path.line_to(0.px(), 0.px()),
            Edge::In => Xy::new(1.0, 1.0),
            Edge::Out => Xy::new(-1.0, 1.0),
        },
    };

    let control_points = match side {
        Side::Top => TOP_CONTROL_POINTS,
        Side::Right => {
            let mut control_points = [[Xy::new(0.0, 0.0); 3]; 6];
            for (i, xys) in TOP_CONTROL_POINTS.iter().enumerate() {
                control_points[i][0] = Xy::new(xys[0].y, xys[0].x);
                control_points[i][1] = Xy::new(xys[1].y, xys[1].x);
                control_points[i][2] = Xy::new(xys[2].y, xys[2].x);
            }
            control_points
        }
        Side::Bottom => {
            let mut control_points = [[Xy::new(0.0, 0.0); 3]; 6];
            for (i, xys) in TOP_CONTROL_POINTS.iter().enumerate() {
                control_points[i][0] = Xy::new(-xys[0].x, -xys[0].y);
                control_points[i][1] = Xy::new(-xys[1].x, -xys[1].y);
                control_points[i][2] = Xy::new(-xys[2].x, -xys[2].y);
            }
            control_points
        }
        Side::Left => {
            let mut control_points = [[Xy::new(0.0, 0.0); 3]; 6];
            for (i, xys) in TOP_CONTROL_POINTS.iter().enumerate() {
                control_points[i][0] = Xy::new(xys[0].y, -xys[0].x);
                control_points[i][1] = Xy::new(xys[1].y, -xys[1].x);
                control_points[i][2] = Xy::new(xys[2].y, -xys[2].x);
            }
            control_points
        }
    };

    let start_xy = match side {
        Side::Top => Xy::new(0.px(), 0.px()),
        Side::Right => Xy::new(piece_wh.width, 0.px()),
        Side::Bottom => Xy::new(piece_wh.width, piece_wh.height),
        Side::Left => Xy::new(0.px(), piece_wh.height),
    };

    for xys in control_points {
        path = path.cubic_to(
            start_xy + piece_wh.as_xy() * xys[0] * sign,
            start_xy + piece_wh.as_xy() * xys[1] * sign,
            start_xy + piece_wh.as_xy() * xys[2] * sign,
        );
    }

    path
}
