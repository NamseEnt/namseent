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
                    .set_stroke_width(2.px()),
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

fn line_piece_part_cw(path: Path, piece_wh: Wh<Px>, side: Side, edge: Edge) -> Path {
    match side {
        Side::Top => {
            let sign = match edge {
                Edge::Straight => return path.line_to(piece_wh.width, 0.px()),
                Edge::In => 1.0,
                Edge::Out => -1.0,
            };

            path.line_to(piece_wh.width * (1.0 / 3.0), 0.px())
                .line_to(
                    piece_wh.width * (1.0 / 3.0),
                    piece_wh.height * (sign * 1.0 / 4.0),
                )
                .line_to(
                    piece_wh.width * (2.0 / 3.0),
                    piece_wh.height * (sign * 1.0 / 4.0),
                )
                .line_to(piece_wh.width * (2.0 / 3.0), 0.px())
                .line_to(piece_wh.width, 0.px())
        }
        Side::Right => {
            let sign = match edge {
                Edge::Straight => return path.line_to(piece_wh.width, piece_wh.height),
                Edge::In => -1.0,
                Edge::Out => 1.0,
            };

            path.line_to(piece_wh.width, piece_wh.height * (1.0 / 3.0))
                .line_to(
                    piece_wh.width * (1.0 + sign * 1.0 / 4.0),
                    piece_wh.height * (1.0 / 3.0),
                )
                .line_to(
                    piece_wh.width * (1.0 + sign * 1.0 / 4.0),
                    piece_wh.height * (2.0 / 3.0),
                )
                .line_to(piece_wh.width, piece_wh.height * (2.0 / 3.0))
                .line_to(piece_wh.width, piece_wh.height)
        }
        Side::Bottom => {
            let sign = match edge {
                Edge::Straight => return path.line_to(0.px(), piece_wh.height),
                Edge::In => -1.0,
                Edge::Out => 1.0,
            };

            path.line_to(piece_wh.width * (2.0 / 3.0), piece_wh.height)
                .line_to(
                    piece_wh.width * (2.0 / 3.0),
                    piece_wh.height * (1.0 + sign * 1.0 / 4.0),
                )
                .line_to(
                    piece_wh.width * (1.0 / 3.0),
                    piece_wh.height * (1.0 + sign * 1.0 / 4.0),
                )
                .line_to(piece_wh.width * (1.0 / 3.0), piece_wh.height)
                .line_to(0.px(), piece_wh.height)
        }
        Side::Left => {
            let sign = match edge {
                Edge::Straight => return path.line_to(0.px(), 0.px()),
                Edge::In => 1.0,
                Edge::Out => -1.0,
            };

            path.line_to(0.px(), piece_wh.height * (2.0 / 3.0))
                .line_to(
                    piece_wh.width * (sign * 1.0 / 4.0),
                    piece_wh.height * (2.0 / 3.0),
                )
                .line_to(
                    piece_wh.width * (sign * 1.0 / 4.0),
                    piece_wh.height * (1.0 / 3.0),
                )
                .line_to(0.px(), piece_wh.height * (1.0 / 3.0))
                .line_to(0.px(), 0.px())
        }
    }
}
