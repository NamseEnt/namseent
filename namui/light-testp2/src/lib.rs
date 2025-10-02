use std::sync::atomic;

wit_bindgen::generate!({
    world: "host",
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct MyHost;

static VALUE: atomic::AtomicUsize = atomic::AtomicUsize::new(0);

impl Guest for MyHost {
    fn run() {
        std::thread::spawn(|| {
            println!("hello on thread");
        });
        println!(
            "Hello, world! {}",
            VALUE.fetch_add(1, atomic::Ordering::SeqCst),
        );
    }
}

// export! defines that the `MyHost` struct defined below is going to define
// the exports of the `world`, namely the `run` function.
export!(MyHost);

// use namui_light::*;

// pub fn main() {
//     namui_light::start(render).unwrap();
// }

// fn render(ctx: &RenderCtx) {
//     let (content, set_content) = ctx.state(|| None);

//     ctx.effect("load media", || {
//         namui_light::spawn(async move {
//             let buffer = namui_light::system::file::bundle::read("resources/text.txt")
//                 .await
//                 .unwrap();
//             let content = std::str::from_utf8(&buffer).unwrap();
//             set_content.set(Some(content.to_string()));
//         });
//     });

//     ctx.add(namui_light::text(TextParam {
//         text: match content.as_ref() {
//             Some(content) => content.to_string(),
//             None => "loading...".to_string(),
//         },
//         x: 0.px(),
//         y: 0.px(),
//         align: TextAlign::Left,
//         baseline: TextBaseline::Top,
//         font: Font {
//             size: 12.int_px(),
//             name: "NotoSansKR-Regular".to_string(),
//         },
//         style: TextStyle::default(),
//         max_width: None,
//     }));
// }
