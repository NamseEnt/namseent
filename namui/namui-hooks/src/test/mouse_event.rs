use super::*;
use std::sync::{Arc, atomic::AtomicUsize};

#[test]
fn event_local_xy_in_on_compose_translate() {
    let mut world = World::init(Instant::now);

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
    }

    const ROWS: usize = 3;
    const COLS: usize = 3;

    const RECT_WH: Wh<Px> = Wh::new(px(100.0), px(100.0));

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let rect_rt = RenderingTree::Node(DrawCommand::Path {
                command: Box::new(PathDrawCommand {
                    path: Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), RECT_WH)),
                    paint: Paint::new(Color::WHITE).set_style(PaintStyle::Fill),
                }),
            });

            for x in 0..COLS {
                for y in 0..ROWS {
                    ctx.compose(|ctx| {
                        ctx.translate((RECT_WH.width * x, RECT_WH.height * y))
                            .add(rect_rt.clone())
                            .attach_event(|event| {
                                let Event::MouseMove { event } = event else {
                                    return;
                                };
                                if event.is_local_xy_in() {
                                    let index = x * ROWS + y;
                                    self.record
                                        .store(index, std::sync::atomic::Ordering::Relaxed);
                                    assert_eq!(
                                        event.global_xy,
                                        Xy::new(
                                            RECT_WH.width * (x as f32 + 0.5),
                                            RECT_WH.height * (y as f32 + 0.5),
                                        ),
                                        "x: {x}, y: {y}",
                                    );
                                    assert_eq!(
                                        event.local_xy(),
                                        RECT_WH.to_xy() * 0.5,
                                        "x: {x}, y: {y}, global_xy: {:?}",
                                        event.global_xy
                                    );
                                }
                            });
                    });
                }
            }
        }
    }

    let record = Arc::new(AtomicUsize::new(usize::MAX));

    let mut result = [usize::MAX; ROWS * COLS];
    for x in 0..COLS {
        for y in 0..ROWS {
            let mouse_xy = Xy::new(
                RECT_WH.width * (x as f32 + 0.5),
                RECT_WH.height * (y as f32 + 0.5),
            );
            World::run_with_event(
                &mut world,
                A {
                    record: record.clone(),
                },
                RawEvent::MouseMove {
                    event: RawMouseEvent {
                        xy: mouse_xy,
                        pressing_buttons: Default::default(),
                        button: Default::default(),
                    },
                },
            );
            result[x * ROWS + y] = record.load(std::sync::atomic::Ordering::Relaxed);
        }
    }
    assert_eq!(result, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn event_local_xy_in_after_translate_at_out() {
    let mut world = World::init(Instant::now);

    #[derive(Debug)]
    struct A {
        record: Arc<AtomicUsize>,
    }

    const ROWS: usize = 3;
    const COLS: usize = 3;

    const RECT_WH: Wh<Px> = Wh::new(px(100.0), px(100.0));

    impl Component for A {
        fn render(self, ctx: &RenderCtx) {
            let rect_rt = RenderingTree::Node(DrawCommand::Path {
                command: Box::new(PathDrawCommand {
                    path: Path::new().add_rect(Rect::from_xy_wh(Xy::zero(), RECT_WH)),
                    paint: Paint::new(Color::WHITE).set_style(PaintStyle::Fill),
                }),
            });

            for x in 0..COLS {
                for y in 0..ROWS {
                    ctx.compose(|ctx| {
                        ctx.translate((RECT_WH.width * x, RECT_WH.height * y))
                            .compose(|ctx| {
                                ctx.add(rect_rt.clone()).attach_event(|event| {
                                    let Event::MouseMove { event } = event else {
                                        return;
                                    };
                                    if event.is_local_xy_in() {
                                        let index = x * ROWS + y;
                                        self.record
                                            .store(index, std::sync::atomic::Ordering::Relaxed);
                                        assert_eq!(
                                            event.global_xy,
                                            Xy::new(
                                                RECT_WH.width * (x as f32 + 0.5),
                                                RECT_WH.height * (y as f32 + 0.5),
                                            ),
                                            "x: {x}, y: {y}",
                                        );
                                        assert_eq!(
                                            event.local_xy(),
                                            RECT_WH.to_xy() * 0.5,
                                            "x: {x}, y: {y}",
                                        );
                                    }
                                });
                            });
                    });
                }
            }
        }
    }

    let record = Arc::new(AtomicUsize::new(usize::MAX));

    let mut result = [usize::MAX; ROWS * COLS];
    for x in 0..COLS {
        for y in 0..ROWS {
            let mouse_xy = Xy::new(
                RECT_WH.width * (x as f32 + 0.5),
                RECT_WH.height * (y as f32 + 0.5),
            );
            World::run_with_event(
                &mut world,
                A {
                    record: record.clone(),
                },
                RawEvent::MouseMove {
                    event: RawMouseEvent {
                        xy: mouse_xy,
                        pressing_buttons: Default::default(),
                        button: Default::default(),
                    },
                },
            );
            result[x * ROWS + y] = record.load(std::sync::atomic::Ordering::Relaxed);
        }
    }
    assert_eq!(result, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
}
