use namui::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(App);
    })
}

struct App;

impl Component for App {
    fn render(self, ctx: &RenderCtx) {
        let size = namui::screen::size();

        // let jpg_length = 14;
        let jpg_length = 1;
        let png_length = 10;
        let jpgs =
            (0..jpg_length).map(|index| ResourceLocation::bundle(format!("resources/{index}.jpg")));
        let pngs =
            (0..png_length).map(|index| ResourceLocation::bundle(format!("resources/{index}.png")));

        let image_resource_locations = jpgs.chain(pngs).collect::<Vec<_>>();

        let x_index_length = 6;
        let y_index_length =
            (image_resource_locations.len() as f32 / x_index_length as f32).ceil() as usize;
        let image_width = size.width.into_px() / x_index_length as f32;
        let image_height = size.height.into_px() / y_index_length as f32;

        ctx.compose(|ctx| {
            for x in 0..x_index_length {
                for y in 0..y_index_length {
                    let Some(resource_location) = image_resource_locations.get(x + y * 6).cloned()
                    else {
                        continue;
                    };
                    let key = format!("{x}-{y}");
                    let rect = Rect::Xywh {
                        x: image_width * x,
                        y: image_height * y,
                        width: image_width,
                        height: image_height,
                    };
                    ctx.add_with_key(
                        key,
                        ImageRender {
                            rect,
                            source: ImageSource::ResourceLocation { resource_location },
                            fit: ImageFit::Contain,
                            paint: None,
                        },
                    );
                }
            }
        });
    }
}
