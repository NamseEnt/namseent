use namui::*;

pub struct Piece {
    pub wh: Wh<Px>,
    pub piece_index: Xy<usize>,
    pub ltrb_edge: Ltrb<Edge>,
    pub image: Image,
    pub image_wh: Wh<Px>,
    pub piece_state: PieceState,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PieceState {
    None,
    DraggingShadow,
    Shaking { started_at: Instant },
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
            PieceState::None | PieceState::Shaking { .. } => None,
            PieceState::DraggingShadow => {
                Some(Paint::new(Color::WHITE).set_color_filter(ColorFilter {
                    color: Color::grayscale_f01(0.5),
                    blend_mode: BlendMode::Lighten,
                }))
            }
        };

        ctx.compose(|mut ctx| {
            if let PieceState::Shaking { started_at } = piece_state {
                struct Keyframe {
                    percent: Percent,
                    translate: Xy<Px>,
                    rotate_angle: Angle,
                }

                let keyframes: [Keyframe; 11] = [
                    Keyframe {
                        percent: percent(0.0),
                        translate: Xy::new(px(1.0), px(1.0)),
                        rotate_angle: 0.deg(),
                    },
                    Keyframe {
                        percent: percent(10.0),
                        translate: Xy::new(px(-1.0), px(-2.0)),
                        rotate_angle: (-1.0).deg(),
                    },
                    Keyframe {
                        percent: percent(20.0),
                        translate: Xy::new(px(-3.0), px(0.0)),
                        rotate_angle: (1.0).deg(),
                    },
                    Keyframe {
                        percent: percent(30.0),
                        translate: Xy::new(px(3.0), px(2.0)),
                        rotate_angle: 0.0.deg(),
                    },
                    Keyframe {
                        percent: percent(40.0),
                        translate: Xy::new(px(1.0), px(-1.0)),
                        rotate_angle: (1.0).deg(),
                    },
                    Keyframe {
                        percent: percent(50.0),
                        translate: Xy::new(px(-1.0), px(2.0)),
                        rotate_angle: (-1.0).deg(),
                    },
                    Keyframe {
                        percent: percent(60.0),
                        translate: Xy::new(px(-3.0), px(1.0)),
                        rotate_angle: 0.0.deg(),
                    },
                    Keyframe {
                        percent: percent(70.0),
                        translate: Xy::new(px(3.0), px(1.0)),
                        rotate_angle: (-1.0).deg(),
                    },
                    Keyframe {
                        percent: percent(80.0),
                        translate: Xy::new(px(-1.0), px(-1.0)),
                        rotate_angle: (1.0).deg(),
                    },
                    Keyframe {
                        percent: percent(90.0),
                        translate: Xy::new(px(1.0), px(2.0)),
                        rotate_angle: 0.0.deg(),
                    },
                    Keyframe {
                        percent: percent(100.0),
                        translate: Xy::new(px(1.0), px(-2.0)),
                        rotate_angle: (-1.0).deg(),
                    },
                ];

                let elapsed = namui::time::now() - started_at;

                let animation_duration = Duration::from_secs_f32(0.5);

                let progress_percent =
                    (((elapsed % animation_duration) / animation_duration) as f32 * 100.0)
                        .percent();

                let keyframe_index = 'outer: {
                    for (index, keyframe) in keyframes.iter().enumerate() {
                        let Some(next_keyframe) = keyframes.get(index + 1) else {
                            break;
                        };
                        if keyframe.percent <= progress_percent
                            && progress_percent <= next_keyframe.percent
                        {
                            break 'outer index;
                        }
                    }
                    keyframes.len() - 1
                };

                let (translate_xy, rotate_angle) = if keyframe_index == keyframes.len() - 1 {
                    let translate_xy = keyframes.last().unwrap().translate;
                    let rotate_angle = keyframes.last().unwrap().rotate_angle;

                    (translate_xy, rotate_angle)
                } else {
                    let next_keyframe = keyframes.get(keyframe_index + 1).unwrap();

                    let ratio = (progress_percent - keyframes[keyframe_index].percent)
                        / (next_keyframe.percent - keyframes[keyframe_index].percent);

                    let translate_xy = next_keyframe.translate * (100.percent() - ratio)
                        + next_keyframe.translate * ratio;

                    let rotate_angle = next_keyframe.rotate_angle * (100.percent() - ratio)
                        + next_keyframe.rotate_angle * ratio;

                    (translate_xy, rotate_angle)
                };

                ctx = ctx.translate(translate_xy).rotate(rotate_angle);
            }

            ctx.translate((-wh.as_xy()) * *piece_index)
                .add(namui::path(
                    clip_path.clone_inner(),
                    Paint::new(Color::BLACK)
                        .set_style(PaintStyle::Stroke)
                        .set_stroke_width(2.px())
                        .set_anti_alias(true),
                ))
                .clip(clip_path.clone_inner(), ClipOp::Intersect)
                .add(ImageDrawCommand {
                    rect: Rect::zero_wh(image_wh),
                    image: image.clone(),
                    fit: ImageFit::Contain,
                    paint,
                });
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
    const SHOULDER_WIDTH: f32 = 0.3;
    const NECK: Wh<f32> = Wh::new(0.1, 0.1);
    const HEAD_RADIUS: Wh<f32> = Wh::new(0.2, 0.075);

    const TOP_CONTROL_POINTS_HALF: [[Xy<f32>; 3]; 3] = [
        [
            Xy::new(0.0, 0.0),
            Xy::new(SHOULDER_WIDTH, 0.0),
            Xy::new(SHOULDER_WIDTH, 0.0),
        ],
        [
            Xy::new(SHOULDER_WIDTH, 0.0),
            Xy::new(SHOULDER_WIDTH + NECK.width, NECK.height / 2.0),
            Xy::new(SHOULDER_WIDTH + NECK.width / 2.0, NECK.height),
        ],
        [
            Xy::new(SHOULDER_WIDTH + NECK.width / 2.0, NECK.height),
            Xy::new(
                0.5 - HEAD_RADIUS.width,
                NECK.height + HEAD_RADIUS.height * 2.0,
            ),
            Xy::new(0.5, NECK.height + HEAD_RADIUS.height * 2.0),
        ],
    ];

    const TOP_CONTROL_POINTS: [[Xy<f32>; 3]; 6] = [
        TOP_CONTROL_POINTS_HALF[0],
        TOP_CONTROL_POINTS_HALF[1],
        TOP_CONTROL_POINTS_HALF[2],
        [
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[2][2].x,
                TOP_CONTROL_POINTS_HALF[2][2].y,
            ),
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[2][1].x,
                TOP_CONTROL_POINTS_HALF[2][1].y,
            ),
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[2][0].x,
                TOP_CONTROL_POINTS_HALF[2][0].y,
            ),
        ],
        [
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[1][2].x,
                TOP_CONTROL_POINTS_HALF[1][2].y,
            ),
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[1][1].x,
                TOP_CONTROL_POINTS_HALF[1][1].y,
            ),
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[1][0].x,
                TOP_CONTROL_POINTS_HALF[1][0].y,
            ),
        ],
        [
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[0][2].x,
                TOP_CONTROL_POINTS_HALF[0][2].y,
            ),
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[0][1].x,
                TOP_CONTROL_POINTS_HALF[0][1].y,
            ),
            Xy::new(
                1.0 - TOP_CONTROL_POINTS_HALF[0][0].x,
                TOP_CONTROL_POINTS_HALF[0][0].y,
            ),
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
                control_points[i][0] = Xy::new(-xys[0].x, xys[0].y);
                control_points[i][1] = Xy::new(-xys[1].x, xys[1].y);
                control_points[i][2] = Xy::new(-xys[2].x, xys[2].y);
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
