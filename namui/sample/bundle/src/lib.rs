use namui::prelude::*;
use namui_prebuilt::typography;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, || BundleExample).await
}

#[namui::component]
struct BundleExample;

impl Component for BundleExample {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (content, set_content) = ctx.state(|| None);

        ctx.effect("load media", || {
            namui::spawn(async move {
                let buffer = namui::system::file::bundle::read("bundle:resources/text.txt")
                    .await
                    .unwrap();
                let content = std::str::from_utf8(&buffer).unwrap();
                set_content.set(Some(content.to_string()));
            });
        });

        ctx.component(typography::body::left_top(
            match content.as_ref() {
                Some(content) => content.to_string(),
                None => "loading...".to_string(),
            },
            Color::BLACK,
        ));

        ctx.done()
    }
}
