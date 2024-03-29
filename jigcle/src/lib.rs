mod piece;
mod solution_board;

use namui::*;
use piece::*;
use solution_board::*;
use std::sync::{atomic::AtomicBool, Arc};

pub fn main() {
    namui::start(|| Game)
}

const IMAGE: &str = "bundle:image.jpg";
const MUSIC: &str = "bundle:music.opus";

#[component]
pub struct Game;

/*

┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┐
│                                          │
│    solution board                        │
│   ┏━━━━━━━━━━━━━━┐                       │
│   │··············│                       │
│   │··············│        playground     │
│   │··············│                       │
│   │··············│                       │
│   │··············│                       │
│   │··············│                       │
│   └━━━━━━━━━━━━━━┘                       │
│                                          │
│                                          │
└━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┘

solution board
- 답을 맞추는 공간
- 보드에 테두리가 있고, 각 칸의 중앙마다 표식이 있음. 위 그림 속 [·]가 그것. 마치 자석처럼, 근처에 피스를 두면 촥 하고 달라붙음.
- 만약 보드 안에 피스간에 교섭이 발생하면 그곳에 경고 표시를 함.

playground
- 게임 화면에서 solution board를 제외한 나머지 공간
- 처음 피스가 마구자비로 흩어짐

피스
- 모양새를 알아차리기 쉽게 테두리가 있음.
- 피스마다 노래의 일부가 들어가 있음. 마우스 오버하면 그 노래가 나옴.

*/
impl Component for Game {
    fn render(self, ctx: &RenderCtx) {
        let image = ctx.image(IMAGE);
        let (music, set_music) = ctx.state::<Option<FullLoadOnceAudio>>(|| None);

        ctx.effect("load music", || {
            let set_music = set_music.cloned();
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path(MUSIC).unwrap();

                let music = namui::media::new_full_load_once_audio(&path).await.unwrap();

                set_music.set(Some(music));
            });
        });

        let Some(Ok(image)) = image.as_ref() else {
            return;
        };

        let Some(music) = music.as_ref() else {
            return;
        };

        const PUZZLE_WIDTH: usize = 8;
        const PUZZLE_HEIGHT: usize = 8;

        const PUZZLE_WH: Wh<usize> = Wh::new(PUZZLE_WIDTH, PUZZLE_HEIGHT);
        let image_height = 900.px();
        let image_width = image_height * (image.wh.height / image.wh.width).as_f32();
        let image_wh = Wh::new(image_width, image_height);
        let piece_wh = image_wh / PUZZLE_WH;

        let screen_wh = Wh::new(1920.px(), 1080.px());

        let ltrb_edges = ctx.memo(|| create_ltrb_edges(PUZZLE_WH));

        let (piece_xys, set_piece_xys) = ctx.state(|| {
            let mut piece_xys = [[Xy::<Px>::zero(); PUZZLE_WIDTH]; PUZZLE_HEIGHT];

            let start_piece_range_rect = {
                let screen_right_middle_center =
                    Xy::new(screen_wh.width * 3.0 / 4.0, screen_wh.height / 2.0);
                let start_piece_range_wh =
                    Wh::new(screen_wh.width / 2.0 * 0.75, screen_wh.height * 0.75);

                Rect::from_xy_wh(
                    screen_right_middle_center - start_piece_range_wh.as_xy() / 2.0,
                    start_piece_range_wh,
                )
            };

            piece_xys.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|cell| {
                    *cell = Xy::new(
                        (start_piece_range_rect.width() - piece_wh.width) * rand::random::<f32>()
                            + start_piece_range_rect.x(),
                        (start_piece_range_rect.height() - piece_wh.height) * rand::random::<f32>()
                            + start_piece_range_rect.y(),
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

        for y in 0..PUZZLE_WH.height {
            for x in 0..PUZZLE_WH.width {
                let piece_index = Xy::new(x, y);
                ctx.compose(|ctx| {
                    ctx.translate(piece_xys[y][x])
                        .add(Piece {
                            wh: piece_wh,
                            piece_index,
                            ltrb_edge: ltrb_edges[y][x],
                            image: image.src.clone(),
                            image_wh,
                        })
                        .attach_event(|event| match event {
                            Event::MouseDown { event } => {
                                if event.is_local_xy_in() {
                                    event.stop_propagation();
                                    set_dragging_piece_state.set(Some(DraggingPieceState {
                                        piece_index,
                                        anchor_xy: event.local_xy(),
                                        last_mouse_xy: event.global_xy,
                                    }));
                                }
                            }
                            Event::MouseMove { event } => {
                                if event.is_local_xy_in() {
                                    event.stop_propagation();

                                    if let Some(state) = playing_audio_state.as_ref() {
                                        if state.piece_index == piece_index {
                                            let audio_duration_for_piece = music.duration()
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

                                    let total_duration = music.duration();
                                    let seek_to = total_duration
                                        * (piece_index.y * PUZZLE_WIDTH + piece_index.x) as f32
                                        / (PUZZLE_WIDTH * PUZZLE_HEIGHT) as f32;

                                    let sliced = music
                                        .slice(
                                            seek_to
                                                ..(seek_to
                                                    + (total_duration
                                                        / (PUZZLE_WIDTH * PUZZLE_HEIGHT) as f32)),
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
                        });
                });
            }
        }

        let screen_left_middle_center = Xy::new(screen_wh.width / 4.0, screen_wh.height / 2.0);
        ctx.translate(screen_left_middle_center - image_wh.as_xy() / 2.0)
            .add(SolutionBoard {
                wh_counts: PUZZLE_WH,
                image_wh,
            });
    }
}

fn create_ltrb_edges(puzzle_wh: Wh<usize>) -> Vec<Vec<Ltrb<Edge>>> {
    let mut jigsaw_puzzles = vec![vec![false; puzzle_wh.width]; puzzle_wh.height]; // ignore last line

    jigsaw_puzzles.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|cell| {
            *cell = rand::random();
        });
    });

    let mut ltrb_edges = vec![
        vec![
            Ltrb {
                left: Edge::Straight,
                top: Edge::Straight,
                right: Edge::Straight,
                bottom: Edge::Straight,
            };
            puzzle_wh.width
        ];
        puzzle_wh.height
    ];
    for y in 0..puzzle_wh.height {
        for x in 0..puzzle_wh.width {
            let edge = &mut ltrb_edges[y][x];

            if y == 0 {
                edge.top = Edge::Straight;
            } else {
                edge.top = match jigsaw_puzzles[y - 1][x] {
                    true => Edge::In,
                    false => Edge::Out,
                };
            }

            if y == puzzle_wh.height - 1 {
                edge.bottom = Edge::Straight;
            } else {
                edge.bottom = match jigsaw_puzzles[y][x] {
                    true => Edge::Out,
                    false => Edge::In,
                };
            }

            if x == 0 {
                edge.left = Edge::Straight;
            } else {
                edge.left = match jigsaw_puzzles[y][x - 1] {
                    true => Edge::In,
                    false => Edge::Out,
                };
            }

            if x == puzzle_wh.width - 1 {
                edge.right = Edge::Straight;
            } else {
                edge.right = match jigsaw_puzzles[y][x] {
                    true => Edge::Out,
                    false => Edge::In,
                };
            }
        }
    }

    ltrb_edges
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
