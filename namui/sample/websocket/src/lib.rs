use namui::*;
use namui_prebuilt::typography;

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (content, set_content) = ctx.state(String::new);

    ctx.effect("web socket", || {
        let set_content = set_content.cloned();
        tokio::spawn(async move {
            let (ws_sender, mut ws_receiver) = namui::network::ws::connect("http://localhost:8080")
                .await
                .unwrap();

            tokio::spawn(async move {
                while let Some(value) = ws_receiver.recv().await {
                    let text = std::str::from_utf8(&value).unwrap().to_string();

                    set_content.mutate(move |content| {
                        *content += &text;
                        *content += "\n"
                    });
                }
            });

            ws_sender.send(b"Hello from client");

            for i in 0..10 {
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                ws_sender.send(format!("Hello from client {}", i).as_bytes());
            }
        });
    });

    ctx.add(typography::body::left_top(
        content.to_string(),
        Color::BLACK,
    ));
}
