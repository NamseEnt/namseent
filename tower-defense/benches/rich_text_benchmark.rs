use criterion::{Criterion, criterion_group, criterion_main};
use namui::*;
use tower_defense::theme::typography::{
    FontSize, PositionedRichText, TypographyBuilder, memoized_text,
};

const FONT_TTF: &str =
    "/Users/namse/namseent/namui/namui-cli/system_bundle/font/Ko/NotoSansKR-Black.ttf";

// The namui runtime normally provides these host FFI symbols. This headless
// benchmark renders text only (no images), so empty stubs satisfy the linker.
#[unsafe(no_mangle)]
extern "C" fn _get_image_count() -> usize {
    0
}
#[unsafe(no_mangle)]
extern "C" fn _get_image_infos(_buffer: *mut u8) {}

fn load_fonts() {
    let bytes = std::fs::read(FONT_TTF).expect("font ttf");
    let _ = NativeTypeface::load("NotoSansKR-Regular", &bytes);
    let _ = NativeTypeface::load("NotoSansKR-Bold", &bytes);
}

fn red() -> Color {
    Color::from_u8(220, 70, 70, 255)
}
fn green() -> Color {
    Color::from_u8(70, 200, 110, 255)
}
fn blue() -> Color {
    Color::from_u8(90, 150, 230, 255)
}

/// Builds a tower-skill-description-like rich text: several text segments with
/// colored style spans, Korean text, wrapped to a fixed width (≈3 lines).
fn skill_desc(builder: TypographyBuilder, kind: usize) -> PositionedRichText {
    let mut b = builder;
    b.paragraph().size(FontSize::Medium).max_width(px(240.0));
    match kind % 4 {
        0 => {
            b.static_text("주변 타워의 ")
                .with_style(|s| {
                    s.color(red()).text("공격력");
                })
                .static_text("을 ")
                .with_style(|s| {
                    s.color(green()).text("+25%");
                })
                .static_text(" 증가시킵니다 (반경 ")
                .with_style(|s| {
                    s.color(blue()).text("3 타일");
                })
                .static_text(")");
        }
        1 => {
            b.static_text("적을 처치할 때마다 ")
                .with_style(|s| {
                    s.color(green()).text("12 골드");
                })
                .static_text("를 추가로 획득하며, 처치한 적의 ")
                .with_style(|s| {
                    s.color(red()).text("이동 속도");
                })
                .static_text("가 느려집니다.");
        }
        2 => {
            b.static_text("탑 카드가 ")
                .with_style(|s| {
                    s.color(blue()).text("스페이드");
                })
                .static_text("일 때 모든 타워의 공격력이 ")
                .with_style(|s| {
                    s.color(green()).text("크게 증가");
                })
                .static_text("하고 사정거리도 함께 늘어납니다.");
        }
        _ => {
            b.static_text("이 타워는 ")
                .with_style(|s| {
                    s.color(red()).text("범위 피해");
                })
                .static_text("를 입히며, 주변 몬스터 ")
                .with_style(|s| {
                    s.color(blue()).text("전체");
                })
                .static_text("에게 지속 효과를 적용합니다.");
        }
    }
    b.render_left_top()
}

/// Number of rich-text blocks rendered per frame (≈ a tower info popup).
const BLOCKS: usize = 8;

struct BenchRoot {
    /// `None` => stable deps (memo hits). `Some(tick)` => deps change per frame.
    tick: Option<usize>,
}
impl Component for BenchRoot {
    fn render(self, ctx: &RenderCtx) {
        let dep: u32 = self.tick.map(|t| t as u32).unwrap_or(0);
        for i in 0..BLOCKS {
            ctx.translate((px(0.0), px(70.0 * i as f32)))
                .add(memoized_text(&dep, move |builder| skill_desc(builder, i)));
        }
    }
}

fn benchmarks(c: &mut Criterion) {
    load_fonts();

    let mut group = c.benchmark_group("rich_text");
    group.sample_size(50);

    group.bench_function("create", |b| {
        b.iter(|| {
            let mut world = World::init(Instant::now);
            std::hint::black_box(World::run(&mut world, BenchRoot { tick: None }));
        });
    });

    group.bench_function("rerender_stable", |b| {
        let mut world = World::init(Instant::now);
        World::run(&mut world, BenchRoot { tick: None });
        b.iter(|| {
            std::hint::black_box(World::run(&mut world, BenchRoot { tick: None }));
        });
    });

    group.bench_function("rerender_changing", |b| {
        let mut world = World::init(Instant::now);
        let mut tick = 0usize;
        World::run(&mut world, BenchRoot { tick: Some(tick) });
        b.iter(|| {
            tick += 1;
            std::hint::black_box(World::run(&mut world, BenchRoot { tick: Some(tick) }));
        });
    });

    group.finish();
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
