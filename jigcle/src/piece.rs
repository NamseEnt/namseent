use namui::*;

#[component]
pub struct Piece {
    pub wh: Wh<Px>,
    pub piece_index: Xy<usize>,
    pub ltrb_edge: Ltrb<Edge>,
    pub image: ImageSource,
    pub image_wh: Wh<Px>,
    pub color_filter: Option<ColorFilter>,
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
            color_filter,
        } = self;
        let wh = ctx.track_eq(&wh);
        let ltrb_edge = ctx.track_eq(&ltrb_edge);
        let piece_index = ctx.track_eq(&piece_index);
        let clip_path = ctx.memo(|| {
            create_piece_clip_path(*wh, *ltrb_edge)
                .translate(wh.width * piece_index.x, wh.height * piece_index.y)
        });

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
                paint: color_filter
                    .map(|color_filter| Paint::new(Color::WHITE).set_color_filter(color_filter)),
            });
    }
}

fn create_piece_clip_path(piece_wh: Wh<Px>, ltrb_edge: Ltrb<Edge>) -> Path {
    let mut clip_path = Path::new();

    clip_path = 'outer: {
        let sign = match ltrb_edge.top {
            Edge::Straight => break 'outer clip_path.line_to(piece_wh.width, 0.px()),
            Edge::In => 1.0,
            Edge::Out => -1.0,
        };

        clip_path
            .line_to(piece_wh.width * (1.0 / 3.0), 0.px())
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
    };
    clip_path = 'outer: {
        let sign = match ltrb_edge.right {
            Edge::Straight => break 'outer clip_path.line_to(piece_wh.width, piece_wh.height),
            Edge::In => -1.0,
            Edge::Out => 1.0,
        };

        clip_path
            .line_to(piece_wh.width, piece_wh.height * (1.0 / 3.0))
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
    };
    clip_path = 'outer: {
        let sign = match ltrb_edge.bottom {
            Edge::Straight => break 'outer clip_path.line_to(0.px(), piece_wh.height),
            Edge::In => -1.0,
            Edge::Out => 1.0,
        };
        clip_path
            .line_to(piece_wh.width * (2.0 / 3.0), piece_wh.height)
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
    };
    clip_path = 'outer: {
        let sign = match ltrb_edge.left {
            Edge::Straight => break 'outer clip_path.line_to(0.px(), 0.px()),
            Edge::In => 1.0,
            Edge::Out => -1.0,
        };

        clip_path
            .line_to(0.px(), piece_wh.height * (2.0 / 3.0))
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
    };
    clip_path.close()
}
