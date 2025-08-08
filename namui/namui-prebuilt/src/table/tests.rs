use super::*;
use crate::simple_rect;
use crate::scroll_view::AutoScrollViewWithCtx;
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

    let mut world = World::init(Instant::now, Arc::new(MockSkCalculate));

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

    let mut world = World::init(Instant::now, Arc::new(MockSkCalculate));

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

#[tokio::test]
async fn auto_scroll_view_with_fit_timing_issue() {
    namui::system::init_for_test().await.unwrap();
    
    let content_height_tracker = Arc::new(Mutex::new(Vec::<Px>::new()));
    let scroll_bounding_box_tracker = Arc::new(Mutex::new(Vec::<Option<Px>>::new()));
    
    let mut world = World::init(Instant::now, Arc::new(MockSkCalculate));

    struct Test {
        content_height_tracker: Arc<Mutex<Vec<Px>>>,
        scroll_bounding_box_tracker: Arc<Mutex<Vec<Option<Px>>>>,
        dynamic_content_height: Px,
    }

    impl Component for Test {
        fn render(self, ctx: &RenderCtx) {
            
            let Self { 
                content_height_tracker,
                scroll_bounding_box_tracker,
                dynamic_content_height,
            } = self;

            ctx.add(AutoScrollViewWithCtx {
                wh: Wh::new(200.px(), 100.px()),
                scroll_bar_width: 10.px(),
                content: move |ctx| {
                    // 실제 inventory처럼 동적 크기를 가지는 콘텐츠 시뮬레이션
                    let content = ctx.ghost_compose("dynamic_content", |ctx| {
                        vertical([
                            fit(FitAlign::LeftTop, move |compose_ctx| {
                                // 동적으로 변경되는 텍스트 콘텐츠
                                println!("  fit render called with height: {:?}", dynamic_content_height);
                                compose_ctx.add(simple_rect(
                                    Wh::new(150.px(), dynamic_content_height),
                                    Color::TRANSPARENT,
                                    0.px(),
                                    Color::BLACK,
                                ));
                                content_height_tracker.lock().unwrap().push(dynamic_content_height);
                            }),
                        ])(Wh::new(190.px(), f32::MAX.px()), ctx);
                    });
                    
                    // bounding_box 계산 시점의 높이 추적 - AutoScrollView가 실제로 계산하는 방식과 동일
                    let bounding_box = namui::bounding_box(&content);
                    scroll_bounding_box_tracker.lock().unwrap().push(
                        bounding_box.map(|bb| bb.height())
                    );
                    
                    // 디버깅: rendering_tree의 내용 확인
                    println!("Frame: content_height={:?}, calculated_bounding_box={:?}", 
                        dynamic_content_height, bounding_box);
                    
                    ctx.add(content);
                },
            });
        }
    }

    // 첫 번째 렌더링: 작은 콘텐츠 (무시)
    world.run(Test {
        content_height_tracker: content_height_tracker.clone(),
        scroll_bounding_box_tracker: scroll_bounding_box_tracker.clone(),
        dynamic_content_height: 50.px(),
    });
    println!("=== Frame 1 (ignored) ===");

    // 두 번째 렌더링: table::fit이 크기를 측정하고 실제로 렌더링
    world.run(Test {
        content_height_tracker: content_height_tracker.clone(),
        scroll_bounding_box_tracker: scroll_bounding_box_tracker.clone(),
        dynamic_content_height: 50.px(),
    });
    println!("=== Frame 2 (should be correct) ===");

    // 세 번째 렌더링: 큰 콘텐츠로 변경 (inventory에 아이템 추가와 같은 상황)
    world.run(Test {
        content_height_tracker: content_height_tracker.clone(),
        scroll_bounding_box_tracker: scroll_bounding_box_tracker.clone(),
        dynamic_content_height: 200.px(),
    });
    println!("=== Frame 3 (content changed to 200px, should be wrong) ===");

    // 네 번째 렌더링: table::fit이 새로운 크기로 재측정
    world.run(Test {
        content_height_tracker: content_height_tracker.clone(),
        scroll_bounding_box_tracker: scroll_bounding_box_tracker.clone(),
        dynamic_content_height: 200.px(),
    });
    println!("=== Frame 4 (should be correct again) ===");

    let content_heights = content_height_tracker.lock().unwrap();
    let scroll_bounding_boxes = scroll_bounding_box_tracker.lock().unwrap();

    println!("Content heights: {:?}", *content_heights);
    println!("Scroll bounding boxes: {:?}", *scroll_bounding_boxes);

    // 2-frame 지연 패턴 분석 (첫 번째 프레임 무시)
    println!("\n=== 2-Frame Delay Pattern Analysis ===");
    
    if scroll_bounding_boxes.len() >= 4 {
        println!("Frame 1 (ignored): {:?}", scroll_bounding_boxes[0]);
        println!("Frame 2 (should be 50px): {:?}", scroll_bounding_boxes[1]);
        println!("Frame 3 (content=200px, but should show 50px): {:?}", scroll_bounding_boxes[2]);
        println!("Frame 4 (should be 200px): {:?}", scroll_bounding_boxes[3]);
        
        // Frame 2: 정상 동작 확인
        if let Some(Some(frame2_height)) = scroll_bounding_boxes.get(1) {
            assert_eq!(*frame2_height, 50.px(), "Frame 2 should show correct 50px");
        }
        
        // Frame 3: 1프레임 지연 확인 (200px 콘텐츠인데 50px 표시)
        if let Some(Some(frame3_height)) = scroll_bounding_boxes.get(2) {
            println!("Frame 3 analysis: content is 200px, but bounding_box shows {}px", frame3_height);
            // 이것이 1프레임 지연의 증거
        }
        
        // Frame 4: 지연된 업데이트 확인
        if let Some(Some(frame4_height)) = scroll_bounding_boxes.get(3) {
            assert!(*frame4_height >= 150.px(), 
                "Frame 4 should finally show correct 200px content, got {:?}", frame4_height);
        }
    }
}

#[tokio::test]
async fn fit_first_frame_rendering_behavior() {
    namui::system::init_for_test().await.unwrap();
    
    let render_call_tracker = Arc::new(Mutex::new(Vec::<(usize, bool)>::new()));
    let mut world = World::init(Instant::now, Arc::new(MockSkCalculate));

    struct Test {
        render_call_tracker: Arc<Mutex<Vec<(usize, bool)>>>,
        frame_count: usize,
    }

    impl Component for Test {
        fn render(self, ctx: &RenderCtx) {
            let Self { render_call_tracker, frame_count } = self;
            
            let tracker = render_call_tracker.clone();
            
            ctx.compose(|ctx| {
                horizontal([
                    ratio(1, |_wh, _ctx| {}),
                    fit(FitAlign::LeftTop, move |compose_ctx| {
                        // fit 내부에서 실제로 렌더링이 호출되는지 추적
                        tracker.lock().unwrap().push((frame_count, true));
                        
                        compose_ctx.add(simple_rect(
                            Wh::new(100.px(), 50.px()),
                            Color::TRANSPARENT,
                            0.px(),
                            Color::BLACK,
                        ));
                    }),
                ])(Wh::new(300.px(), 50.px()), ctx);
            });
        }
    }

    // 여러 프레임에 걸쳐 렌더링하여 fit의 2단계 동작 추적
    for frame in 0..4 {
        world.run(Test {
            render_call_tracker: render_call_tracker.clone(),
            frame_count: frame,
        });
    }

    let render_calls = render_call_tracker.lock().unwrap();
    println!("Fit render calls: {:?}", *render_calls);
    
    // 첫 번째 프레임에서 fit 콘텐츠가 렌더링되지 않는 문제 확인
    let first_frame_calls: Vec<_> = render_calls.iter()
        .filter(|(frame, _)| *frame == 0)
        .collect();
    
    // 만약 첫 번째 프레임에서 렌더링이 호출되지 않는다면,
    // 이것이 AutoScrollView와의 타이밍 문제의 원인
    println!("First frame render calls: {:?}", first_frame_calls);
}
