use namui::prelude::*;

pub async fn main() {
    let namui_context = namui::init().await;

    namui::start(namui_context, &mut App::new(), &()).await
}

struct App {}

impl App {
    fn new() -> Self {
        Self {}
    }
}

impl Entity for App {
    type Props = ();

    fn render(&self, _props: &Self::Props) -> RenderingTree {
        let size = namui::screen::size();
        let now = namui::now();

        if now < 5.sec() {
            return RenderingTree::Empty;
        }

        let jpg_length = 14;
        let png_length = 42;
        let jpgs = (0..jpg_length)
            .map(|index| Url::parse(&format!("bundle:resources/{index}.jpg")).unwrap());
        let pngs = (0..png_length)
            .map(|index| Url::parse(&format!("bundle:resources/{index}.png")).unwrap());

        let image_urls = jpgs.chain(pngs).collect::<Vec<_>>();

        let x_index_length = 6;
        let y_index_length = (image_urls.len() as f32 / x_index_length as f32).ceil() as usize;
        let image_width = size.width / x_index_length as f32;
        let image_height = size.height / y_index_length as f32;

        let mut images = vec![];
        for x in 0..x_index_length {
            for y in 0..y_index_length {
                if let Some(image_url) = image_urls.get(x + y * 6) {
                    let image = namui::image(ImageParam {
                        rect: Rect::Xywh {
                            x: image_width * x,
                            y: image_height * y,
                            width: image_width,
                            height: image_height,
                        },
                        source: ImageSource::Url(image_url.clone()),
                        style: ImageStyle {
                            fit: ImageFit::Contain,
                            paint_builder: None,
                        },
                    });

                    images.push(image);
                }
            }
        }

        render(images)
    }

    fn update(&mut self, _event: &dyn std::any::Any) {}
}
