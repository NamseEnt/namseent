mod game_state;

use game_state::*;
use namui::{rand::seq::SliceRandom, *};
use namui_prebuilt::*;
use std::collections::BTreeMap;

pub fn main() {
    namui::start(|ctx| {
        let (game_state, set_game_state) = ctx.init_atom(&GAME_STATE_ATOM, GameState::new);

        ctx.on_raw_event(|event| {
            let event = event.clone();
            set_game_state.mutate(move |game_state| {
                game_state.on_namui_event(event);
            });
        });

        ctx.interval("tick", 33.ms(), |_dt| {
            set_game_state.mutate(|game_state| {
                game_state.tick();
            });
        });

        ctx.add(game_state.as_ref());

        ctx.add(simple_rect(
            namui::screen::size().map(|v| v.into_px()),
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK,
        ));
    });
}

struct ItemInfo {
    pub name: &'static str,
    pub image_path: &'static str,
    pub price: usize,
}

const ITEM_INFOS: [ItemInfo; 3] = [
    ItemInfo {
        name: "배지",
        image_path: "asset/badge.jpg",
        price: 3000,
    },
    ItemInfo {
        name: "스탠드",
        image_path: "asset/stand.jpg",
        price: 12000,
    },
    ItemInfo {
        name: "스티커",
        image_path: "asset/sticker.jpg",
        price: 1000,
    },
];

struct App {}
impl Component for App {
    fn render(self, ctx: &RenderCtx) {
        let (clicked_goods_counts, set_clicked_goods_counts) =
            ctx.state(BTreeMap::<usize, usize>::new);
        let (game_flow, set_game_flow) = ctx.state(|| GameFlow::Idle {
            start_at: Instant::now(),
        });
        let (revenue, set_revenue) = ctx.state(|| 0);

        let screen_wh = screen::size().into_type::<Px>();

        ctx.on_raw_event(|event| {
            if let GameFlow::CalculatingPrice {
                answer, candidates, ..
            } = *game_flow
            {
                let RawEvent::KeyUp { event } = event else {
                    return;
                };

                let index = match event.code {
                    Code::Digit1 => 0,
                    Code::Digit2 => 1,
                    Code::Digit3 => 2,
                    _ => return,
                };
                let candidate = candidates[index];
                set_game_flow.set(if candidate == answer {
                    GameFlow::CustomerPurchasing {
                        end_at: Instant::now() + Duration::from_secs(2),
                        amount: answer,
                    }
                } else {
                    GameFlow::CalculatingPrice {
                        answer,
                        candidates,
                        is_wrong_answer: true,
                    }
                });
            }
        });

        ctx.interval("tick", 33.ms(), |_dt| match *game_flow {
            GameFlow::Idle { start_at } => {
                if start_at + 4.sec() < Instant::now() {
                    set_game_flow.set(GameFlow::CustomerWaitingGoods {
                        goods_index: rand::random::<usize>() % ITEM_INFOS.len(),
                        goods_count: 1 + rand::random::<usize>() % 3,
                    });
                }
            }
            GameFlow::CustomerPurchasing { end_at, amount } => {
                if end_at < Instant::now() {
                    set_game_flow.set(GameFlow::CustomerLeaving {
                        start_at: Instant::now(),
                    });
                    set_revenue.mutate(move |revenue| {
                        *revenue += amount;
                    });
                }
            }
            GameFlow::CustomerLeaving { start_at } => {
                if start_at + 2.sec() < Instant::now() {
                    set_game_flow.set(GameFlow::Idle {
                        start_at: Instant::now(),
                    });
                }
            }
            GameFlow::CustomerWaitingGoods { .. } | GameFlow::CalculatingPrice { .. } => {}
        });

        ctx.add(namui::text(TextParam {
            text: match *game_flow {
                GameFlow::Idle { .. } => "".to_string(),
                GameFlow::CustomerWaitingGoods {
                    goods_index,
                    goods_count,
                } => {
                    format!("{}번 {goods_count}개 주세요", goods_index + 1)
                }
                GameFlow::CalculatingPrice {
                    candidates,
                    is_wrong_answer,
                    ..
                } => {
                    let thought = if is_wrong_answer {
                        "틀렸어. 다시..."
                    } else {
                        "얼마지...?"
                    };
                    format!(
                        "'{}' - 1. {}, 2. {}, 3. {}",
                        thought, candidates[0], candidates[1], candidates[2]
                    )
                }
                GameFlow::CustomerPurchasing { .. } => "잠시만요...! 지갑이... 여깄다!".to_string(),
                GameFlow::CustomerLeaving { .. } => "감사합니다~".to_string(),
            },
            x: 300.px(),
            y: 200.px(),
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font: Font {
                name: "NotoSansKR-Regular".to_string(),
                size: 24.int_px(),
            },
            style: TextStyle {
                color: Color::WHITE,
                ..Default::default()
            },
            max_width: None,
        }));

        let clicked_goods_text = clicked_goods_counts
            .iter()
            .map(|(index, count)| format!("{}: {}", ITEM_INFOS[*index].name, count))
            .collect::<Vec<_>>()
            .join(", ");
        ctx.add(typography::body::left(
            24.px(),
            format!("[수익 - {revenue}][선택된 아이템] {clicked_goods_text}"),
            Color::WHITE,
        ));

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(20.px(), 404.px()))
                .add(GoodsStockBox {
                    on_click_goods: &|goods_index| {
                        set_clicked_goods_counts.mutate(move |counts| {
                            *counts.entry(goods_index).or_insert(0) += 1;
                        });
                    },
                });
        });

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(330.px(), 260.px()))
                .add(display_stand);
        });

        ctx.compose(|ctx| {
            ctx.translate(Xy::new(10.px(), 500.px())).add(booth_table);
        });

        ctx.compose(|ctx| {
            if matches!(*game_flow, GameFlow::Idle { .. }) {
                return;
            }
            ctx.translate(Xy::new(210.px(), 350.px())).add(Customer {
                on_click: &|| match *game_flow {
                    GameFlow::Idle { .. } => unreachable!(),
                    GameFlow::CustomerWaitingGoods {
                        goods_count,
                        goods_index,
                    } => {
                        let Some(clicked_count) = clicked_goods_counts.get(&goods_index) else {
                            return;
                        };

                        if *clicked_count < goods_count {
                            return;
                        }
                        set_clicked_goods_counts.set(BTreeMap::new());
                        let answer = ITEM_INFOS[goods_index].price * goods_count;
                        let mut candidates = [
                            answer,
                            answer + 1000,
                            if answer <= 1000 {
                                answer + 2000
                            } else {
                                answer - 1000
                            },
                        ];
                        let mut rng = namui::rand::thread_rng();
                        candidates.shuffle(&mut rng);
                        set_game_flow.set(GameFlow::CalculatingPrice {
                            answer,
                            candidates,
                            is_wrong_answer: false,
                        });
                    }
                    GameFlow::CalculatingPrice { .. }
                    | GameFlow::CustomerLeaving { .. }
                    | GameFlow::CustomerPurchasing { .. } => {}
                },
            });
        });

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK,
        ));
    }
}

enum GameFlow {
    Idle {
        start_at: Instant,
    },
    CustomerWaitingGoods {
        goods_index: usize,
        goods_count: usize,
    },
    CalculatingPrice {
        answer: usize,
        candidates: [usize; 3],
        is_wrong_answer: bool,
    },
    CustomerPurchasing {
        end_at: Instant,
        amount: usize,
    },
    CustomerLeaving {
        start_at: Instant,
    },
}

struct Customer<'a> {
    pub on_click: &'a dyn Fn(),
}
impl Component for Customer<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { on_click } = self;

        let head_radius = 40.px();
        let neck_length = 20.px();
        let body_length = 100.px();
        let arm_length = 60.px();
        let leg_length = 60.px();

        /*
             O
             |
            /|\
             |
            / \
        */

        // --- Person Drawing ---
        let person_path = Path::new()
            // Head
            .add_oval(Rect::from_xy_wh(
                Xy::zero(),
                Wh::new(head_radius * 2, head_radius * 2),
            ))
            // Neck + Body
            .move_to(head_radius, head_radius * 2) // Top of neck
            .line_to(head_radius, head_radius * 2 + neck_length + body_length) // Bottom of body (Leg joint)
            // Arms
            .move_to(head_radius, head_radius * 2 + neck_length) // Arm joint (Bottom of neck)
            .line_to(
                head_radius - arm_length,      // x coordinate for left hand (horizontal)
                head_radius * 2 + neck_length, // y coordinate for left hand (same as joint)
            ) // Left hand (90 deg)
            .move_to(head_radius, head_radius * 2 + neck_length) // Arm joint
            .line_to(
                head_radius + arm_length,      // x coordinate for right hand (horizontal)
                head_radius * 2 + neck_length, // y coordinate for right hand (same as joint)
            ) // Right hand (90 deg)
            // Legs
            .move_to(head_radius, head_radius * 2 + neck_length + body_length) // Leg joint
            .line_to(
                head_radius - leg_length,
                head_radius * 2 + neck_length + body_length + leg_length,
            ) // Left foot (approx 45 deg)
            .move_to(head_radius, head_radius * 2 + neck_length + body_length) // Leg joint
            .line_to(
                head_radius + leg_length,
                head_radius * 2 + neck_length + body_length + leg_length,
            ); // Right foot (approx 45 deg)

        let person_paint = Paint::new(Color::WHITE)
            .set_style(PaintStyle::Stroke)
            .set_stroke_width(2.px());

        ctx.add(namui::path(person_path, person_paint))
            .attach_event(|event| {
                let Event::MouseUp { event } = event else {
                    return;
                };
                if event.is_local_xy_in() {
                    on_click();
                }
            });
    }
}

struct GoodsStockBox<'a> {
    pub on_click_goods: &'a dyn Fn(usize),
}

impl Component for GoodsStockBox<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self { on_click_goods } = self;

        let hole_wh = Wh::new(48.px(), 24.px());
        let width_hole = 3;
        let height_hole = 4;

        let hole = simple_rect(hole_wh, Color::WHITE, 1.px(), Color::grayscale_f01(0.8));

        for x in 0..width_hole {
            for y in 0..height_hole {
                let index = x + y * width_hole;

                ctx.compose(|mut ctx| {
                    ctx = ctx
                        .mouse_cursor(MouseCursor::Standard(StandardCursor::Pointer))
                        .translate((hole_wh.width * x, hole_wh.height * y));

                    let item = ITEM_INFOS.get(index);

                    if let Some(item) = item {
                        ctx.add(namui::ImageRender {
                            rect: Rect::from_xy_wh(Xy::zero(), hole_wh),
                            source: ImageSource::ResourceLocation {
                                resource_location: ResourceLocation::Bundle(item.image_path.into()),
                            },
                            fit: ImageFit::Contain,
                            paint: None,
                        });
                    }

                    ctx.add(hole.clone());

                    ctx.attach_event(|event| {
                        if item.is_none() {
                            return;
                        }
                        let Event::MouseUp { event } = event else {
                            return;
                        };
                        if event.is_local_xy_in() {
                            on_click_goods(index);
                        }
                    });
                });
            }
        }
    }
}

fn display_stand(ctx: &RenderCtx) {
    let hole_wh = Wh::new(16.px(), 16.px());
    let width_hole = 8;
    let height_hole = 15;

    let border = simple_rect(
        Wh::new(hole_wh.width * width_hole, hole_wh.height * height_hole),
        Color::WHITE,
        4.px(),
        Color::TRANSPARENT,
    );

    ctx.add(border);

    let keyring_path = {
        let rect_wh = Wh::new(12.px(), 16.px());
        let ring_radius = 4.px();
        let chain_length = 8.px();

        Path::new()
            .add_oval(Rect::from_xy_wh(
                Xy::new(rect_wh.width / 2 - ring_radius, 0.px()),
                Wh::new(ring_radius * 2, ring_radius * 2),
            ))
            .move_to(rect_wh.width / 2, ring_radius * 2)
            .line_to(rect_wh.width / 2, ring_radius * 2 + chain_length)
            .add_rect(Rect::from_xy_wh(
                Xy::new(0.px(), ring_radius * 2 + chain_length),
                rect_wh,
            ))
    };
    let keyring_front_paint = Paint::new(Color::WHITE)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(2.px());
    let keyring_back_paint = Paint::new(Color::BLACK)
        .set_style(PaintStyle::Stroke)
        .set_stroke_width(3.px());

    for x in 0..3 {
        for y in 0..2 {
            let x_offset = (hole_wh.width * width_hole) / 3 * (x as f32 + 0.5);
            let y_offset = (hole_wh.height * height_hole) / 2 * (y as f32 + 0.5);
            ctx.add(namui::translate(
                x_offset,
                y_offset,
                render([
                    namui::path(keyring_path.clone(), keyring_front_paint.clone()),
                    namui::path(keyring_path.clone(), keyring_back_paint.clone()),
                ]),
            ));
        }
    }

    let hole = simple_rect(
        hole_wh,
        Color::grayscale_f01(0.5),
        1.px(),
        Color::TRANSPARENT,
    );

    for x in 0..width_hole {
        for y in 0..height_hole {
            let x_offset = hole_wh.width * x;
            let y_offset = hole_wh.height * y;
            ctx.add(namui::translate(x_offset, y_offset, hole.clone()));
        }
    }
}

fn booth_table(ctx: &RenderCtx) {
    let dark_brown = Color::from_u8(0x8B, 0x45, 0x13, 0xFF);
    let light_brown = Color::from_u8(0xD2, 0xB4, 0x8C, 0xFF);
    let ivory = Color::from_u8(0xFF, 0xF8, 0xDC, 0xFF);

    // 상판
    let table_top = simple_rect(Wh::new(480.px(), 20.px()), Color::WHITE, 1.px(), dark_brown);
    // 다리
    let table_leg = simple_rect(
        Wh::new(20.px(), 100.px()),
        Color::WHITE,
        1.px(),
        light_brown,
    );
    // 가리개
    let table_cover = simple_rect(Wh::new(480.px(), 75.px()), Color::WHITE, 1.px(), ivory);

    ctx.add(table_top);
    ctx.add(namui::translate(20.px(), 20.px(), table_leg.clone()));
    ctx.add(namui::translate(440.px(), 20.px(), table_leg));
    ctx.add(table_cover);
}
