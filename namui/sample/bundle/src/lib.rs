use namui::*;
use namui_prebuilt::typography;

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (content, set_content) = ctx.state(|| None);

    ctx.effect("load media", || {
        let set_content = set_content.cloned();
        namui::spawn(async move {
            let buffer = namui::system::file::bundle::read("bundle:resources/text.txt")
                .await
                .unwrap();
            let content = std::str::from_utf8(&buffer).unwrap();
            set_content.set(Some(content.to_string()));
        });
    });

    ctx.add(typography::body::left_top(
        match content.as_ref() {
            Some(content) => content.to_string(),
            None => "loading...".to_string(),
        },
        Color::BLACK,
    ));
}
