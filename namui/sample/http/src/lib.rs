use namui::*;
use namui_prebuilt::{table, typography};
use network::http::HttpError;

pub fn main() {
    namui::start(render)
}

fn render(ctx: &RenderCtx) {
    let (geojs_content, set_geojs_content) = ctx.state(|| None);
    let (google_content, set_google_content) = ctx.state(|| None);
    let (stream_content, set_stream_content) = ctx.state(|| None);
    let (stream_post_content, set_stream_post_content) = ctx.state(|| None);

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
            let content = match request("https://google.com/").await {
                Ok(content) => content,
                Err(err) => err.to_string(),
            };
            set_google_content.set(Some(content));
        });
    });

    ctx.effect("stream", || {
        ctx.spawn(async move {
            let stream = async move {
                Result::<_, HttpError>::Ok(
                    namui::system::network::http::Request::builder()
                        // no cors
                        .uri("http://localhost:5174/@fs/home/ubuntu/namseent/namui/sample/http/target/namui/target/wasm32-wasip1-threads/debug/bundle.sqlite")
                        .body(())?
                        .send()
                        .await?
                        .stream(),
                )
            }
            .await;

            match stream {
                Ok(mut stream) => {
                    let mut length = 0;
                    while let Some(bytes) = stream.next().await {
                        match bytes {
                            Ok(bytes) => {
                                length += bytes.len();

                                let bytes_hex = bytes
                                    .iter()
                                    .map(|byte| format!("{:02x}", byte))
                                    .collect::<Vec<String>>()
                                    .join("");
                                let line = format!("\n[{}] {}", bytes_hex.len(), bytes_hex);

                                set_stream_content.mutate(move |content| {
                                    if let Some(content) = content {
                                        *content += line.as_str();
                                    } else {
                                        *content = Some(line);
                                    }
                                });

                                if length > 512 * 1024 {
                                    break;
                                }
                            }
                            Err(err) => {
                                let err = err.to_string();
                                set_stream_content.mutate(move |content| {
                                    let line = format!("\nstream error: {}", err);
                                    if let Some(content) = content {
                                        *content += &line;
                                    } else {
                                        *content = Some(line);
                                    }
                                });
                            }
                        }
                    }
                }
                Err(err) => {
                    let content = err.to_string();
                    set_stream_content.set(Some(format!("stream error: {}", content)));
                }
            }
        });
    });

    ctx.effect("stream post", || {
        ctx.spawn(async move {
            let result = async move {
                namui::system::network::http::Request::builder()
                    .uri("http://localhost:8123")
                    .method("POST")
                    .body(vec![1, 2, 3])?
                    .send()
                    .await?
                    .bytes()
                    .await
            }
            .await;

            match result {
                Ok(bytes) => {
                    let content = String::from_utf8(bytes).unwrap();
                    set_stream_post_content.set(Some(content));
                }
                Err(err) => {
                    let content = err.to_string();
                    set_stream_post_content.set(Some(format!("stream error: {}", content)));
                }
            }
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
            table::ratio(1, |_, ctx| {
                ctx.add(typography::body::left_top(
                    match stream_content.as_ref() {
                        Some(content) => "stream: ".to_string() + &content.to_string(),
                        None => "stream loading...".to_string(),
                    },
                    Color::BLACK,
                ));
            }),
            table::ratio(1, |_, ctx| {
                ctx.add(typography::body::left_top(
                    match stream_post_content.as_ref() {
                        Some(content) => "stream post: ".to_string() + &content.to_string(),
                        None => "stream post loading...".to_string(),
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
