use super::*;
use crate::simple_rect;
use std::sync::{Arc, Mutex, atomic::AtomicBool};

struct MockSkCalculate;
impl SkCalculate for MockSkCalculate {
    fn group_glyph(&self, _font: &Font, _paint: &Paint) -> Arc<dyn GroupGlyph> {
        unimplemented!()
    }

    fn font_metrics(&self, _font: &Font) -> Option<FontMetrics> {
        unimplemented!()
    }

    fn load_typeface(&self, _typeface_name: String, _bytes: Vec<u8>) -> JoinHandle<Result<()>> {
        unimplemented!()
    }

    fn path_contains_xy(&self, _path: &Path, _paint: Option<&Paint>, _xy: Xy<Px>) -> bool {
        unimplemented!()
    }

    fn path_bounding_box(&self, _path: &Path, _paint: Option<&Paint>) -> Option<Rect<Px>> {
        unimplemented!()
    }

    fn load_image_from_raw(&self, _image_info: ImageInfo, _bitmap: &[u8]) -> JoinHandle<Image> {
        unimplemented!()
    }

    fn load_image_from_encoded(&self, _bytes: &[u8]) -> JoinHandle<Image> {
        todo!()
    }
}

#[tokio::test]
async fn closure_should_give_right_wh() {
    namui::system::init_for_test().await.unwrap();
    let button_render_called = Arc::new(AtomicBool::new(false));
    let label_render_called = Arc::new(AtomicBool::new(false));
    let body_render_called = Arc::new(AtomicBool::new(false));
    let body_inner_render_called = Arc::new(AtomicBool::new(false));

    let mut world = World::init(Instant::now, &MockSkCalculate);

    struct Test {
        button_render_called: Arc<AtomicBool>,
        label_render_called: Arc<AtomicBool>,
        body_render_called: Arc<AtomicBool>,
        body_inner_render_called: Arc<AtomicBool>,
    }

    impl Component for Test {
        fn render(self, ctx: &RenderCtx) {
            let Self {
                button_render_called,
                label_render_called,
                body_render_called,
                body_inner_render_called,
            } = self;

            let button = calculative(
                |parent_wh| parent_wh.height,
                |wh, _ctx| {
                    button_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                    assert_eq!(px(20.0), wh.width);
                    assert_eq!(px(20.0), wh.height);
                },
            );

            let label = ratio(1, |wh, _ctx| {
                label_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(px(280.0), wh.width);
                assert_eq!(px(20.0), wh.height);
            });

            let header = fixed(px(20.0), horizontal([("button", button), ("label", label)]));

            let body = ratio(1.0, |wh, ctx| {
                body_render_called.store(true, std::sync::atomic::Ordering::Relaxed);
                assert_eq!(px(300.0), wh.width);
                assert_eq!(px(480.0), wh.height);
                vertical([
                    (
                        "0",
                        ratio(
                            1,
                            padding(5.px(), |wh, _ctx| {
                                body_inner_render_called
                                    .store(true, std::sync::atomic::Ordering::Relaxed);
                                assert_eq!(px(290.0), wh.width);
                                assert_eq!(px(470.0), wh.height);
                            }),
                        ),
                    ),
                    // Note: RenderingTree is not testable yet, So you cannot test fit well now.
                    (
                        "empty",
                        fit(FitAlign::LeftTop, |ctx| {
                            ctx.add(RenderingTree::Empty);
                        }),
                    ),
                ])(wh, ctx)
            });

            ctx.compose(|ctx| {
                vertical([header, body])(
                    Wh {
                        width: px(300.0),
                        height: px(500.0),
                    },
                    ctx,
                );
            });
        }
    }

    world.run(Test {
        button_render_called: button_render_called.clone(),
        label_render_called: label_render_called.clone(),
        body_render_called: body_render_called.clone(),
        body_inner_render_called: body_inner_render_called.clone(),
    });

    assert!(button_render_called.load(std::sync::atomic::Ordering::Relaxed));
    assert!(label_render_called.load(std::sync::atomic::Ordering::Relaxed));
    assert!(body_render_called.load(std::sync::atomic::Ordering::Relaxed));
    assert!(body_inner_render_called.load(std::sync::atomic::Ordering::Relaxed));
}

#[tokio::test]
async fn fit_should_work() {
    namui::system::init_for_test().await.unwrap();
    let a_width = Arc::new(Mutex::new(0.px()));

    let mut world = World::init(Instant::now, &MockSkCalculate);

    struct Test {
        a_width: Arc<Mutex<Px>>,
    }

    impl Component for Test {
        fn render(self, ctx: &RenderCtx) {
            let Self { a_width } = self;

            let a = ratio(1, |wh, _ctx| {
                *a_width.lock().unwrap() = wh.width;
            });

            let b = fit(FitAlign::LeftTop, |ctx| {
                ctx.add(simple_rect(
                    Wh::new(100.px(), 32.px()),
                    Color::TRANSPARENT,
                    0.0.px(),
                    Color::BLACK,
                ));
            });

            ctx.compose(|ctx| {
                horizontal([a, b])(
                    Wh {
                        width: px(1000.0),
                        height: px(32.0),
                    },
                    ctx,
                );
            });
        }
    }

    world.run(Test {
        a_width: a_width.clone(),
    });

    assert_eq!(px(1000.0), *a_width.lock().unwrap());

    world.run(Test {
        a_width: a_width.clone(),
    });

    assert_eq!(px(900.0), *a_width.lock().unwrap());

    world.run(Test {
        a_width: a_width.clone(),
    });

    assert_eq!(px(900.0), *a_width.lock().unwrap());
}
