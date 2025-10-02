use namui_light::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static VALUE: AtomicUsize = AtomicUsize::new(0);

#[unsafe(no_mangle)]
pub extern "C" fn callable_from_c() -> usize {
    println!("before call thread spawn");
    std::thread::spawn(|| {
        println!("hello on thread");
        let value = VALUE.fetch_add(1, Ordering::SeqCst);
        println!("value: {}", value);
    });
    println!("after call thread spawn");
    VALUE.load(Ordering::SeqCst)
}

fn main() {
    namui_light::start(render).unwrap();
}

fn render(ctx: &RenderCtx) {
    let (content, set_content) = ctx.state(|| None);

    ctx.effect("load media", || {
        namui_light::spawn(async move {
            let buffer = namui_light::system::file::bundle::read("resources/text.txt")
                .await
                .unwrap();
            let content = std::str::from_utf8(&buffer).unwrap();
            set_content.set(Some(content.to_string()));
        });
    });

    ctx.add(namui_light::text(TextParam {
        text: match content.as_ref() {
            Some(content) => content.to_string(),
            None => "loading...".to_string(),
        },
        x: 0.px(),
        y: 0.px(),
        align: TextAlign::Left,
        baseline: TextBaseline::Top,
        font: Font {
            size: 12.int_px(),
            name: "NotoSansKR-Regular".to_string(),
        },
        style: TextStyle::default(),
        max_width: None,
    }));

    println!("component done");
}
