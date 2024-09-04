use namui::*;
use namui_prebuilt::{table, typography};

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (geojs_content, set_geojs_content) = ctx.state(|| None);
    let (google_content, set_google_content) = ctx.state(|| None);

    ctx.effect("geojs - no cors", || {
        ctx.spawn(async move {
            let content = match request("https://get.geojs.io/v1/ip/country.json?ip=8.8.8.8").await
            {
                Ok(content) => content,
                Err(err) => err.to_string(),
            };
            set_geojs_content.set(Some(content));
        });
    });

    ctx.effect("google - yes cors", || {
        ctx.spawn(async move {
            let content = match request("https://google.com").await {
                Ok(content) => content,
                Err(err) => err.to_string(),
            };
            set_google_content.set(Some(content));
        });
    });

    ctx.compose(|ctx| {
        table::vertical([
            table::ratio(1, |_, ctx| {
                ctx.add(typography::body::left_top(
                    match geojs_content.as_ref() {
                        Some(content) => "geojs: ".to_string() + &content.to_string(),
                        None => "geojs loading...".to_string(),
                    },
                    Color::BLACK,
                ));
            }),
            table::ratio(1, |_, ctx| {
                ctx.add(typography::body::left_top(
                    match google_content.as_ref() {
                        Some(content) => "google: ".to_string() + &content.to_string(),
                        None => "google loading...".to_string(),
                    },
                    Color::BLACK,
                ));
            }),
        ])(namui::screen::size().map(|x| x.into_px()), ctx);
    });
}

async fn request(url: &str) -> namui::Result<String> {
    let bytes = namui::system::network::http::Request::builder()
        // no cors
        .uri(url)
        .body(())?
        .send()
        .await?
        .bytes()
        .await?;

    let content = String::from_utf8(bytes).unwrap();

    Ok(content)
}
