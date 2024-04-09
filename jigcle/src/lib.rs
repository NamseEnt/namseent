mod piece;
mod solution_board;

use namui::*;
use piece::*;
use solution_board::*;

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
  - 그 표식이 있는 칸 하나를 '슬롯'이라고 부르자.
- 만약 보드 안에 피스간에 교섭이 발생하면 그곳에 경고 표시를 함.

playground
- 게임 화면에서 solution board를 제외한 나머지 공간
- 처음 피스가 마구자비로 흩어짐

피스
- 모양새를 알아차리기 쉽게 테두리가 있음.
- 피스마다 노래의 일부가 들어가 있음. 마우스 오버하면 그 노래가 나옴.

플레이 소감: 2024-03-31
playground의 크기가 좀 더 커져야할 것 같음. 즉, 피스의 크기가 더 작아야할 것 같음.
노래 소리가 너무 계속 들려서 정신사나움. 오른쪽 클릭하면 나오게 하든지, 노래 나오는 모드를 지정할 수 있게 해야할듯.

플레이 소감: 2024-04-01
playground를 줄이니까 꽤 괜찮음.
노래는 그냥 브금으로 깔았음.
*/
impl Component for Game {
    fn render(self, ctx: &RenderCtx) {
        let image = ctx.image(IMAGE);
        let (bgm, set_bgm) = ctx.state::<Option<FullLoadRepeatAudio>>(|| None);

        ctx.effect("load bgm", || {
            let set_bgm = set_bgm.cloned();
            namui::spawn(async move {
                let path = namui::system::file::bundle::to_real_path(MUSIC).unwrap();

                let bgm = namui::media::new_full_load_repeat_audio(&path)
                    .await
                    .unwrap();

                set_bgm.set(Some(bgm));
            });
        });

        let Some(Ok(image)) = image.as_ref() else {
            return;
        };

        ctx.effect("repeat bgm", || {
            let Some(bgm) = bgm.as_ref() else {
                return;
            };

            bgm.play().unwrap();
        });

        const PUZZLE_WIDTH: usize = 8;
        const PUZZLE_HEIGHT: usize = 8;

        const PUZZLE_WH: Wh<usize> = Wh::new(PUZZLE_WIDTH, PUZZLE_HEIGHT);
        let image_height = 600.px();
        let image_width = image_height * (image.wh.height / image.wh.width).as_f32();
        let image_wh = Wh::new(image_width, image_height);
        let piece_wh = image_wh / PUZZLE_WH;

        let screen_wh = Wh::new(1920.px(), 1080.px());

        let ltrb_edges = ctx.memo(|| create_ltrb_edges(PUZZLE_WH));

        #[derive(Debug, Clone, Copy)]
        enum PiecePosition {
            Playground { xy: Xy<Px> },
            SolutionBoard { slot_index: Xy<usize> },
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        enum PositionType {
            Playground,
            SolutionBoard,
        }

        let (piece_positions, set_piece_positions) = ctx.state(|| {
            let mut piece_xys = [[PiecePosition::Playground {
                xy: Xy::<Px>::zero(),
            }; PUZZLE_WIDTH]; PUZZLE_HEIGHT];

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
                    *cell = PiecePosition::Playground {
                        xy: Xy::new(
                            (start_piece_range_rect.width() - piece_wh.width)
                                * rand::random::<f32>()
                                + start_piece_range_rect.x(),
                            (start_piece_range_rect.height() - piece_wh.height)
                                * rand::random::<f32>()
                                + start_piece_range_rect.y(),
                        ),
                    };
                });
            });

            piece_xys
        });

        let solution_board_slot_piece_index_map = ctx.memo(|| {
            let mut solution_board_slot_piece_index_map = [[None; PUZZLE_WIDTH]; PUZZLE_HEIGHT];

            for y in 0..PUZZLE_HEIGHT {
                for x in 0..PUZZLE_WIDTH {
                    let PiecePosition::SolutionBoard { slot_index } = piece_positions[y][x] else {
                        continue;
                    };
                    solution_board_slot_piece_index_map[slot_index.y][slot_index.x] =
                        Some(Xy::new(x, y));
                }
            }

            solution_board_slot_piece_index_map
        });

        #[derive(Debug)]
        struct DraggingPieceState {
            piece_index: Xy<usize>,
            anchor_xy: Xy<Px>,
            last_mouse_xy: Xy<Px>,
        }

        let (dragging_piece_state, set_dragging_piece_state) =
            ctx.state::<Option<DraggingPieceState>>(|| None);

        let screen_left_middle_center = Xy::new(screen_wh.width / 4.0, screen_wh.height / 2.0);
        let solution_board_xy = screen_left_middle_center - image_wh.as_xy() / 2.0;

        let to_piece_xy = |piece_position: PiecePosition| -> Xy<Px> {
            match piece_position {
                PiecePosition::Playground { xy } => xy,
                PiecePosition::SolutionBoard { slot_index } => {
                    solution_board_xy + piece_wh.as_xy() * slot_index
                }
            }
        };

        let get_piece_position = |piece_xy: Xy<Px>| {
            let piece_center_xy = piece_xy + piece_wh.as_xy() / 2.0;

            let solution_board_rect = Rect::from_xy_wh(solution_board_xy, image_wh);

            if solution_board_rect.is_xy_inside(piece_center_xy) {
                let next_slot_index = Xy::new(
                    ((piece_center_xy.x - solution_board_xy.x) / piece_wh.width).floor() as usize,
                    ((piece_center_xy.y - solution_board_xy.y) / piece_wh.height).floor() as usize,
                );

                if piece_positions.iter().flatten().any(|position| {
                    let PiecePosition::SolutionBoard { slot_index } = position else {
                        return false;
                    };
                    next_slot_index == *slot_index
                }) {
                    PiecePosition::Playground { xy: piece_xy }
                } else {
                    PiecePosition::SolutionBoard {
                        slot_index: next_slot_index,
                    }
                }
            } else {
                PiecePosition::Playground { xy: piece_xy }
            }
        };

        ctx.on_raw_event(|event| match event {
            RawEvent::MouseUp { event } => {
                if let Some(dragging_piece_state) = dragging_piece_state.as_ref() {
                    set_dragging_piece_state.set(None);

                    let last_mouse_xy = event.xy;

                    let piece_index = dragging_piece_state.piece_index;
                    let next_piece_xy = last_mouse_xy - dragging_piece_state.anchor_xy;
                    let next_piece_position = get_piece_position(next_piece_xy);

                    set_piece_positions.mutate(move |piece_xys| {
                        piece_xys[piece_index.y][piece_index.x] = next_piece_position;
                    });
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

                    set_piece_positions.mutate(move |piece_xys| {
                        piece_xys[piece_index.y][piece_index.x] =
                            PiecePosition::Playground { xy: next_piece_xy };
                    });
                }
            }
            _ => (),
        });

        ctx.compose_2("warning for overlapping on solution board", |ctx| {
            let ctx = ctx.translate(solution_board_xy);

            let xy_iter =
                (0..PUZZLE_WH.height).flat_map(|y| (0..PUZZLE_WH.width).map(move |x| (x, y)));

            let dt = namui::time::since_start().as_secs_f32();
            let pattern_hz = 0.5;
            let pattern_length = 10.px();
            let from = pattern_length * (dt * pattern_hz);
            let to = pattern_length * (dt * pattern_hz) + pattern_length;
            let paint = Paint::new(Color::WHITE).set_shader(Shader::LinearGradient {
                start_xy: Xy::new(from, from),
                end_xy: Xy::new(from, to),
                colors: vec![
                    Color::from_f01(1.0, 0.0, 0.0, 0.50),
                    Color::from_f01(0.0, 0.0, 0.0, 0.50),
                    Color::from_f01(1.0, 0.0, 0.0, 0.50),
                    Color::from_f01(0.5, 0.5, 0.5, 0.50),
                ],
                tile_mode: TileMode::Repeat,
            });
            let paint2 = Paint::new(Color::WHITE).set_shader(Shader::LinearGradient {
                start_xy: Xy::new(from, from),
                end_xy: Xy::new(to, from),
                colors: vec![
                    Color::from_f01(1.0, 0.0, 0.0, 0.50),
                    Color::from_f01(0.0, 0.0, 0.0, 0.50),
                    Color::from_f01(1.0, 0.0, 0.0, 0.50),
                    Color::from_f01(0.5, 0.5, 0.5, 0.50),
                ],
                tile_mode: TileMode::Repeat,
            });

            for (slot_x, slot_y) in xy_iter {
                let Some(piece_index) = solution_board_slot_piece_index_map[slot_y][slot_x] else {
                    continue;
                };

                let add_intersection_warning =
                    |piece_part_1: Path, piece_part_2: Path, two_squares: Path| {
                        ctx.clip(piece_part_1, ClipOp::Intersect)
                            .clip(piece_part_2, ClipOp::Intersect)
                            .add(namui::path(two_squares.clone(), paint.clone()))
                            .add(namui::path(two_squares, paint2.clone()));
                    };

                if slot_x > 0 {
                    if let Some(left_piece_index) =
                        solution_board_slot_piece_index_map[slot_y][slot_x - 1]
                    {
                        let left_piece_xy = piece_wh.as_xy() * Xy::new(slot_x - 1, slot_y);
                        let me_piece_xy = piece_wh.as_xy() * Xy::new(slot_x, slot_y);

                        let square_plus_right_on_left_piece = piece::create_piece_clip_path(
                            piece_wh,
                            Ltrb {
                                left: Edge::Straight,
                                top: Edge::Straight,
                                right: ltrb_edges[left_piece_index.y][left_piece_index.x].right,
                                bottom: Edge::Straight,
                            },
                        )
                        .translate(left_piece_xy.x, left_piece_xy.y);

                        let square_plus_left_on_me = piece::create_piece_clip_path(
                            piece_wh,
                            Ltrb {
                                left: ltrb_edges[piece_index.y][piece_index.x].left,
                                top: Edge::Straight,
                                right: Edge::Straight,
                                bottom: Edge::Straight,
                            },
                        )
                        .translate(me_piece_xy.x, me_piece_xy.y);

                        let two_squares = Path::new()
                            .add_rect(Rect::from_xy_wh(left_piece_xy, piece_wh * Wh::new(2, 1)));

                        add_intersection_warning(
                            square_plus_right_on_left_piece,
                            square_plus_left_on_me,
                            two_squares,
                        );
                    }
                }

                if slot_y > 0 {
                    if let Some(top_piece_index) =
                        solution_board_slot_piece_index_map[slot_y - 1][slot_x]
                    {
                        let top_piece_xy = piece_wh.as_xy() * Xy::new(slot_x, slot_y - 1);
                        let me_piece_xy = piece_wh.as_xy() * Xy::new(slot_x, slot_y);

                        let square_plus_bottom_on_top_piece = piece::create_piece_clip_path(
                            piece_wh,
                            Ltrb {
                                left: Edge::Straight,
                                top: Edge::Straight,
                                right: Edge::Straight,
                                bottom: ltrb_edges[top_piece_index.y][top_piece_index.x].bottom,
                            },
                        )
                        .translate(top_piece_xy.x, top_piece_xy.y);

                        let square_plus_top_on_me = piece::create_piece_clip_path(
                            piece_wh,
                            Ltrb {
                                left: Edge::Straight,
                                top: ltrb_edges[piece_index.y][piece_index.x].top,
                                right: Edge::Straight,
                                bottom: Edge::Straight,
                            },
                        )
                        .translate(me_piece_xy.x, me_piece_xy.y);

                        let two_squares = Path::new()
                            .add_rect(Rect::from_xy_wh(top_piece_xy, piece_wh * Wh::new(1, 2)));

                        add_intersection_warning(
                            square_plus_bottom_on_top_piece,
                            square_plus_top_on_me,
                            two_squares,
                        );
                    }
                }
            }
        });

        ctx.compose_2(
            "non-dragging pieces, order in playground and solution board",
            |ctx| {
                let iter = [PositionType::Playground, PositionType::SolutionBoard]
                    .into_iter()
                    .flat_map(|piece_type| {
                        (0..PUZZLE_WH.height).flat_map(move |y| {
                            (0..PUZZLE_WH.width).map(move |x| (piece_type, x, y))
                        })
                    });

                for (position_type, x, y) in iter {
                    let piece_position_type = match piece_positions[y][x] {
                        PiecePosition::Playground { .. } => PositionType::Playground,
                        PiecePosition::SolutionBoard { .. } => PositionType::SolutionBoard,
                    };

                    if piece_position_type != position_type {
                        continue;
                    }

                    let piece_index = Xy::new(x, y);
                    ctx.compose(|ctx| {
                        let piece_xy = to_piece_xy(piece_positions[y][x]);

                        ctx.translate(piece_xy)
                            .add(Piece {
                                wh: piece_wh,
                                piece_index,
                                ltrb_edge: ltrb_edges[y][x],
                                image: image.src.clone(),
                                image_wh,
                                piece_state: PieceState::None,
                            })
                            .attach_event(|event| {
                                if let Event::MouseDown { event } = event {
                                    if event.is_local_xy_in() {
                                        event.stop_propagation();
                                        set_dragging_piece_state.set(Some(DraggingPieceState {
                                            piece_index,
                                            anchor_xy: event.local_xy(),
                                            last_mouse_xy: event.global_xy,
                                        }));
                                    }
                                }
                            });
                    });
                }
            },
        );

        ctx.compose_2("dragging piece", |ctx| {
            let Some(dragging_piece_state) = dragging_piece_state.as_ref() else {
                return;
            };
            let piece_index = dragging_piece_state.piece_index;
            let piece_xy = dragging_piece_state.last_mouse_xy - dragging_piece_state.anchor_xy;
            let PiecePosition::SolutionBoard { slot_index } = get_piece_position(piece_xy) else {
                return;
            };

            let piece_xy_on_slot = to_piece_xy(PiecePosition::SolutionBoard { slot_index });

            ctx.translate(piece_xy_on_slot).add(Piece {
                wh: piece_wh,
                piece_index,
                ltrb_edge: ltrb_edges[piece_index.y][piece_index.x],
                image: image.src.clone(),
                image_wh,
                piece_state: PieceState::Dragging,
            });
        });

        ctx.translate(solution_board_xy).add(SolutionBoard {
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
