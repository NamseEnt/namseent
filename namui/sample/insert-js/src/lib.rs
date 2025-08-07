use namui::*;
use namui_prebuilt::typography;
use std::sync::{Arc, atomic::AtomicBool};

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (content, set_content) = ctx.state(String::new);

    ctx.effect("Insert google gsi html api", || {
        println!("insert js start");

        let is_component_unmounted = Arc::new(AtomicBool::new(false));
        tokio::spawn({
            let is_component_unmounted = is_component_unmounted.clone();
            async move {
                let js_handle = namui::wasi::insert_js(
                    include_str!("login.js"),
                    Some(move |data: &[u8]| {
                        let string = std::str::from_utf8(data).unwrap().to_string();
                        println!("data from js: {string}");
                        set_content.mutate(move |content| {
                            *content += &string;
                            *content += "\n"
                        })
                    }),
                );

                js_handle.send_data([1, 2, 3, 4, 5]);

                // To keep _js_handle alive
                while !is_component_unmounted.load(std::sync::atomic::Ordering::Relaxed) {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
            }
        });

        move || {
            is_component_unmounted.store(true, std::sync::atomic::Ordering::Relaxed);
        }
    });

    ctx.add(typography::body::left_top(
        content.to_string(),
        Color::BLACK,
    ));
}
