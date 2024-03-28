use namui::prelude::*;
use std::sync::{atomic::AtomicBool, Arc};

const IMAGE: &str = "bundle:image.jpg";
const MUSIC: &str = "bundle:music.opus";

#[component]
pub struct Game {}

impl Component for Game {
    fn render(self, ctx: &RenderCtx) {
        let drum = ctx.image(IMAGE);
        const PUZZLE_WIDTH: usize = 8;
        const PUZZLE_HEIGHT: usize = 8;

        let jigsaw_puzzles = ctx.memo(|| {
            let mut jigsaw_puzzles = [[false; PUZZLE_WIDTH]; PUZZLE_HEIGHT]; // ignore last line

            jigsaw_puzzles.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    *cell = rand::random();
                });
            });

            jigsaw_puzzles
        });

        let (media, set_media) = ctx.state::<Option<FullLoadOnceAudio>>(|| None);

        ctx.effect("load media", || {
            let set_media = set_media.cloned();
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path(MUSIC).unwrap();

                let media = namui::media::new_full_load_once_audio(&path).await.unwrap();

                set_media.set(Some(media));
            });
        });

        let piece_wh = Wh::new(100.px(), 100.px());
        let playground_wh = Wh::new(1600.px(), 800.px());

        let (piece_xys, set_piece_xys) = ctx.state(|| {
            let mut piece_xys = [[Xy::<Px>::zero(); PUZZLE_WIDTH]; PUZZLE_HEIGHT];

            piece_xys.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    // random in playground_wh
                    *cell = Xy::new(
                        (playground_wh.width - piece_wh.width) * rand::random::<f32>(),
                        (playground_wh.height - piece_wh.height) * rand::random::<f32>(),
                    );
                });
            });

            piece_xys
        });

        #[derive(Debug)]
        struct PlayingAudioState {
            piece_index: Xy<usize>,
            start_time: Duration,
            stop: Arc<AtomicBool>,
        }

        let (playing_audio_state, set_playing_audio_state) =
            ctx.state::<Option<PlayingAudioState>>(|| None);

        #[derive(Debug)]
        struct DraggingPieceState {
            piece_index: Xy<usize>,
            anchor_xy: Xy<Px>,
            last_mouse_xy: Xy<Px>,
        }

        let (dragging_piece_state, set_dragging_piece_state) =
            ctx.state::<Option<DraggingPieceState>>(|| None);

        ctx.on_raw_event(|event| match event {
            RawEvent::MouseUp { event: _ } => {
                if dragging_piece_state.is_some() {
                    set_dragging_piece_state.set(None);
                }
            }
            RawEvent::MouseMove { event } => {
                if let Some(dragging_piece_state) = dragging_piece_state.as_ref() {
                    let last_mouse_xy = event.xy;
                    set_dragging_piece_state.mutate(move |state| {
                        state.as_mut().unwrap().last_mouse_xy = last_mouse_xy;
                    });

                    let piece_index = dragging_piece_state.piece_index;
                    let next_piece_xy = last_mouse_xy - dragging_piece_state.anchor_xy;
                    set_piece_xys.mutate(move |piece_xys| {
                        piece_xys[piece_index.y][piece_index.x] = next_piece_xy;
                    });
                }
            }
            _ => (),
        });

        let Some(media) = media.as_ref() else {
            return;
        };

        ctx.compose(|ctx| {
            let Some(Ok(image)) = drum.as_ref() else {
                return;
            };

            for y in 0..PUZZLE_HEIGHT {
                for x in 0..PUZZLE_WIDTH {
                    enum Edge {
                        In,
                        Out,
                        Straight,
                    }

                    let top = if y == 0 {
                        Edge::Straight
                    } else {
                        match jigsaw_puzzles[y - 1][x] {
                            true => Edge::In,
                            false => Edge::Out,
                        }
                    };
                    let bottom = if y == PUZZLE_HEIGHT - 1 {
                        Edge::Straight
                    } else {
                        match jigsaw_puzzles[y][x] {
                            true => Edge::Out,
                            false => Edge::In,
                        }
                    };

                    let left = if x == 0 {
                        Edge::Straight
                    } else {
                        match jigsaw_puzzles[y][x - 1] {
                            true => Edge::In,
                            false => Edge::Out,
                        }
                    };

                    let right = if x == PUZZLE_WIDTH - 1 {
                        Edge::Straight
                    } else {
                        match jigsaw_puzzles[y][x] {
                            true => Edge::Out,
                            false => Edge::In,
                        }
                    };

                    let mut clip_path =
                        Path::new().move_to(piece_wh.width * x, piece_wh.height * y);
                    clip_path = match top {
                        Edge::In => clip_path
                            .line_to(piece_wh.width * (x as f32 + 1.0 / 3.0), piece_wh.height * y)
                            .line_to(
                                piece_wh.width * (x as f32 + 1.0 / 3.0),
                                piece_wh.height * (y as f32 + 1.0 / 4.0),
                            )
                            .line_to(
                                piece_wh.width * (x as f32 + 2.0 / 3.0),
                                piece_wh.height * (y as f32 + 1.0 / 4.0),
                            )
                            .line_to(piece_wh.width * (x as f32 + 2.0 / 3.0), piece_wh.height * y)
                            .line_to(piece_wh.width * (x + 1), piece_wh.height * y),
                        Edge::Out => clip_path
                            .line_to(piece_wh.width * (x as f32 + 1.0 / 3.0), piece_wh.height * y)
                            .line_to(
                                piece_wh.width * (x as f32 + 1.0 / 3.0),
                                piece_wh.height * (y as f32 - 1.0 / 4.0),
                            )
                            .line_to(
                                piece_wh.width * (x as f32 + 2.0 / 3.0),
                                piece_wh.height * (y as f32 - 1.0 / 4.0),
                            )
                            .line_to(piece_wh.width * (x as f32 + 2.0 / 3.0), piece_wh.height * y)
                            .line_to(piece_wh.width * (x + 1), piece_wh.height * y),
                        Edge::Straight => {
                            clip_path.line_to(piece_wh.width * (x + 1), piece_wh.height * y)
                        }
                    };
                    clip_path = match right {
                        Edge::Straight => {
                            clip_path.line_to(piece_wh.width * (x + 1), piece_wh.height * (y + 1))
                        }
                        Edge::In => clip_path
                            .line_to(
                                piece_wh.width * (x + 1),
                                piece_wh.height * (y as f32 + 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 1.0 / 4.0),
                                piece_wh.height * (y as f32 + 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 1.0 / 4.0),
                                piece_wh.height * (y as f32 + 2.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * (x + 1),
                                piece_wh.height * (y as f32 + 2.0 / 3.0),
                            )
                            .line_to(piece_wh.width * (x + 1), piece_wh.height * (y + 1)),
                        Edge::Out => clip_path
                            .line_to(
                                piece_wh.width * (x + 1),
                                piece_wh.height * (y as f32 + 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 + 1.0 / 4.0),
                                piece_wh.height * (y as f32 + 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 + 1.0 / 4.0),
                                piece_wh.height * (y as f32 + 2.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * (x + 1),
                                piece_wh.height * (y as f32 + 2.0 / 3.0),
                            )
                            .line_to(piece_wh.width * (x + 1), piece_wh.height * (y + 1)),
                    };
                    clip_path = match bottom {
                        Edge::Straight => {
                            clip_path.line_to(piece_wh.width * x, piece_wh.height * (y + 1))
                        }
                        Edge::In => clip_path
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 1.0 / 3.0),
                                piece_wh.height * (y + 1),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 1.0 / 3.0),
                                piece_wh.height * ((y + 1) as f32 - 1.0 / 4.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 2.0 / 3.0),
                                piece_wh.height * ((y + 1) as f32 - 1.0 / 4.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 2.0 / 3.0),
                                piece_wh.height * (y + 1),
                            )
                            .line_to(piece_wh.width * ((x) as f32), piece_wh.height * (y + 1)),
                        Edge::Out => clip_path
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 1.0 / 3.0),
                                piece_wh.height * (y + 1),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 1.0 / 3.0),
                                piece_wh.height * ((y + 1) as f32 + 1.0 / 4.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 2.0 / 3.0),
                                piece_wh.height * ((y + 1) as f32 + 1.0 / 4.0),
                            )
                            .line_to(
                                piece_wh.width * ((x + 1) as f32 - 2.0 / 3.0),
                                piece_wh.height * (y + 1),
                            )
                            .line_to(piece_wh.width * (x as f32), piece_wh.height * (y + 1)),
                    };
                    clip_path = match left {
                        Edge::Straight => {
                            clip_path.line_to(piece_wh.width * x, piece_wh.height * y)
                        }
                        Edge::In => clip_path
                            .line_to(
                                piece_wh.width * x,
                                piece_wh.height * ((y + 1) as f32 - 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * (x as f32 + 1.0 / 4.0),
                                piece_wh.height * ((y + 1) as f32 - 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * (x as f32 + 1.0 / 4.0),
                                piece_wh.height * ((y + 1) as f32 - 2.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * x,
                                piece_wh.height * ((y + 1) as f32 - 2.0 / 3.0),
                            )
                            .line_to(piece_wh.width * x, piece_wh.height * (y + 1)),
                        Edge::Out => clip_path
                            .line_to(
                                piece_wh.width * x,
                                piece_wh.height * ((y + 1) as f32 - 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * (x as f32 - 1.0 / 4.0),
                                piece_wh.height * ((y + 1) as f32 - 1.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * (x as f32 - 1.0 / 4.0),
                                piece_wh.height * ((y + 1) as f32 - 2.0 / 3.0),
                            )
                            .line_to(
                                piece_wh.width * x,
                                piece_wh.height * ((y + 1) as f32 - 2.0 / 3.0),
                            )
                            .line_to(piece_wh.width * x, piece_wh.height * (y + 1)),
                    };
                    clip_path = clip_path.close();
                    let piece_index = Xy::new(x, y);

                    ctx.compose(|ctx| {
                        ctx.translate((
                            -piece_wh.width * x + piece_xys[y][x].x,
                            -piece_wh.height * y + piece_xys[y][x].y,
                        ))
                        .clip(clip_path, ClipOp::Intersect)
                        .add(
                            ImageDrawCommand {
                                rect: Rect::zero_wh(Wh::new(
                                    piece_wh.width * PUZZLE_WIDTH,
                                    piece_wh.height * PUZZLE_HEIGHT,
                                )),
                                source: image.src.clone(),
                                fit: ImageFit::Contain,
                                paint: None,
                            }
                            .attach_event(|event| match event {
                                Event::MouseDown { event } => {
                                    if event.is_local_xy_in() {
                                        event.stop_propagation();
                                        set_dragging_piece_state.set(Some(DraggingPieceState {
                                            piece_index,
                                            anchor_xy: event.local_xy()
                                                - Xy::new(piece_wh.width * x, piece_wh.height * y),
                                            last_mouse_xy: event.global_xy,
                                        }));
                                    }
                                }
                                Event::MouseMove { event } => {
                                    if event.is_local_xy_in() {
                                        event.stop_propagation();

                                        if let Some(state) = playing_audio_state.as_ref() {
                                            if state.piece_index == piece_index {
                                                let audio_duration_for_piece = media.duration()
                                                    / (PUZZLE_WIDTH * PUZZLE_HEIGHT) as f32;

                                                if namui::time::since_start() - state.start_time
                                                    <= audio_duration_for_piece
                                                {
                                                    return;
                                                }
                                            }

                                            state
                                                .stop
                                                .store(true, std::sync::atomic::Ordering::Relaxed);
                                        }

                                        let total_duration = media.duration();
                                        let seek_to = total_duration
                                            * (piece_index.y * PUZZLE_WIDTH + piece_index.x) as f32
                                            / (PUZZLE_WIDTH * PUZZLE_HEIGHT) as f32;

                                        let sliced = media
                                            .slice(
                                                seek_to
                                                    ..(seek_to
                                                        + (total_duration
                                                            / (PUZZLE_WIDTH * PUZZLE_HEIGHT)
                                                                as f32)),
                                            )
                                            .unwrap();

                                        let stop = Arc::new(AtomicBool::new(false));
                                        let audio = StoppableAudio {
                                            audio: sliced,
                                            stop: stop.clone(),
                                        };

                                        namui::media::play_audio_consume(audio).unwrap();

                                        set_playing_audio_state.set(Some(PlayingAudioState {
                                            start_time: namui::time::since_start(),
                                            piece_index,
                                            stop,
                                        }));
                                    }
                                }
                                _ => {}
                            }),
                        );
                    });
                }
            }
        });
    }
}

#[derive(Debug)]
struct StoppableAudio {
    audio: FullLoadOnceAudio,
    stop: Arc<AtomicBool>,
}

impl namui::media::AudioConsume for StoppableAudio {
    fn consume(&mut self, output: &mut [f32]) {
        self.audio.consume(output);
    }

    fn is_end(&self) -> bool {
        self.stop.load(std::sync::atomic::Ordering::Relaxed)
    }
}
