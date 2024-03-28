use namui::prelude::*;

pub fn main() {
    namui::start(|| App)
}

#[namui::component]
struct App;

impl Component for App {
    fn render(self, ctx: &RenderCtx)  {
        let size = namui::screen::size();

        // let jpg_length = 14;
        let jpg_length = 1;
        let png_length = 10;
        let jpgs = (0..jpg_length)
            .map(|index| Url::parse(&format!("bundle:resources/{index}.jpg")).unwrap());
        let pngs = (0..png_length)
            .map(|index| Url::parse(&format!("bundle:resources/{index}.png")).unwrap());

        let image_urls = jpgs.chain(pngs).collect::<Vec<_>>();

        let x_index_length = 6;
        let y_index_length = (image_urls.len() as f32 / x_index_length as f32).ceil() as usize;
        let image_width = size.width.into_px() / x_index_length as f32;
        let image_height = size.height.into_px() / y_index_length as f32;

        ctx.compose(|ctx| {
            for x in 0..x_index_length {
                for y in 0..y_index_length {
                    if let Some(image_url) = image_urls.get(x + y * 6) {
                        let key = format!("{}-{}", x, y);
                        ctx.add_with_key(
                            key,
                            namui::image(ImageParam {
                                rect: Rect::Xywh {
                                    x: image_width * x,
                                    y: image_height * y,
                                    width: image_width,
                                    height: image_height,
                                },
                                source: ImageSource::Url {
                                    url: image_url.clone(),
                                },
                                style: ImageStyle {
                                    fit: ImageFit::Contain,
                                    paint: None,
                                },
                            }),
                        );
                    }
                }
            }
        });

        
    }
}
