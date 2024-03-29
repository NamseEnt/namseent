use namui::*;

#[component]
pub struct Piece {
    pub wh: Wh<Px>,
    pub piece_index: Xy<usize>,
    pub ltrb_edge: Ltrb<Edge>,
    pub image: ImageSource,
    pub image_wh: Wh<Px>,
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
        } = self;
        let wh = ctx.track_eq(&wh);
        let ltrb_edge = ctx.track_eq(&ltrb_edge);
        let piece_index = ctx.track_eq(&piece_index);
        let clip_path = ctx.memo(|| {
            create_piece_clip_path(*wh, *ltrb_edge)
                .translate(wh.width * piece_index.x, wh.height * piece_index.y)
        });

        //                 Path::new().move_to(piece_wh.width * x, piece_wh.height * y);
        //             ctx.compose(|ctx| {
        //                 ctx.translate((
        //                     -piece_wh.width * x + piece_xys[y][x].x,
        //                     -piece_wh.height * y + piece_xys[y][x].y,
        //                 ))
        //                 .clip(clip_path, ClipOp::Intersect)

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
                paint: None,
            });
        //                     .attach_event(|event| match event {
        //                         Event::MouseDown { event } => {
        //                             if event.is_local_xy_in() {
        //                                 event.stop_propagation();
        //                                 set_dragging_piece_state.set(Some(DraggingPieceState {
        //                                     piece_index,
        //                                     anchor_xy: event.local_xy()
        //                                         - Xy::new(piece_wh.width, piece_wh.height),
        //                                     last_mouse_xy: event.global_xy,
        //                                 }));
        //                             }
        //                         }
        //                         Event::MouseMove { event } => {
        //                             if event.is_local_xy_in() {
        //                                 event.stop_propagation();

        //                                 if let Some(state) = playing_audio_state.as_ref() {
        //                                     if state.piece_index == piece_index {
        //                                         let audio_duration_for_piece = media.duration()
        //                                             / (PUZZLE_WIDTH * PUZZLE_HEIGHT) as f32;

        //                                         if namui::time::since_start() - state.start_time
        //                                             <= audio_duration_for_piece
        //                                         {
        //                                             return;
        //                                         }
        //                                     }

        //                                     state
        //                                         .stop
        //                                         .store(true, std::sync::atomic::Ordering::Relaxed);
        //                                 }

        //                                 let total_duration = media.duration();
        //                                 let seek_to = total_duration
        //                                     * (piece_index.y * PUZZLE_WIDTH + piece_index.x) as f32
        //                                     / (PUZZLE_WIDTH * PUZZLE_HEIGHT) as f32;

        //                                 let sliced = media
        //                                     .slice(
        //                                         seek_to
        //                                             ..(seek_to
        //                                                 + (total_duration
        //                                                     / (PUZZLE_WIDTH * PUZZLE_HEIGHT)
        //                                                         as f32)),
        //                                     )
        //                                     .unwrap();

        //                                 let stop = Arc::new(AtomicBool::new(false));
        //                                 let audio = StoppableAudio {
        //                                     audio: sliced,
        //                                     stop: stop.clone(),
        //                                 };

        //                                 namui::media::play_audio_consume(audio).unwrap();

        //                                 set_playing_audio_state.set(Some(PlayingAudioState {
        //                                     start_time: namui::time::since_start(),
        //                                     piece_index,
        //                                     stop,
        //                                 }));
        //                             }
        //                         }
        //                         _ => {}
        //                     }),
        //                 );
        //             });
        //         }
        //     }
        // });
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
