use namui::*;
use namui_prebuilt::simple_rect;

#[component]
pub struct VideoPlayer<'a> {
    pub wh: Wh<Px>,
    pub note_plotter_height: Px,
    pub video: &'a MediaHandle,
}

impl Component for VideoPlayer<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            video,
            note_plotter_height,
        } = self;

        ctx.compose(|ctx| {
            let Some(image) = video.get_image() else {
                return;
            };

            let image_wh = Wh::new(wh.width, wh.height - note_plotter_height);

            ctx.add(ImageRender {
                rect: Rect::zero_wh(image_wh),
                source: ImageSource::Image {
                    image: image.clone(),
                },
                fit: ImageFit::Contain,
                paint: None,
            })
            .add(ImageRender {
                rect: Rect::zero_wh(wh),
                source: ImageSource::Image { image },
                fit: ImageFit::Cover,
                paint: Some(Paint::new(Color::from_u8(0, 0, 0, 128)).set_image_filter(
                    ImageFilter::Blur {
                        sigma_xy: Xy::new(4.0.into(), 4.0.into()),
                        tile_mode: None,
                        input: None,
                        crop_rect: None,
                    },
                )),
            })
            .add(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK));
        });
    }
}
