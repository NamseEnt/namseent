use namui::*;
use namui_prebuilt::simple_rect;

#[component]
pub struct VideoPlayer<'a> {
    pub wh: Wh<Px>,
    pub note_plotter_height: Px,
    pub video: &'a MediaHandle,
}

impl Component for VideoPlayer<'_> {
    fn render(self, ctx: &RenderCtx)  {
        let Self {
            wh,
            video,
            note_plotter_height,
        } = self;

        ctx.compose(|ctx| {
            let Some(image_handle) = video.get_image() else {
                return;
            };

            let image_wh = Wh::new(wh.width, wh.height - note_plotter_height);

            ctx.add(namui::image(ImageParam {
                rect: Rect::zero_wh(image_wh),
                source: ImageSource::ImageHandle {
                    image_handle: image_handle.clone(),
                },
                style: ImageStyle {
                    fit: ImageFit::Contain,
                    paint: None,
                },
            }))
            .add(namui::image(ImageParam {
                rect: Rect::zero_wh(wh),
                source: ImageSource::ImageHandle { image_handle },
                style: ImageStyle {
                    fit: ImageFit::Cover,
                    paint: Some(Paint::new(Color::from_u8(0, 0, 0, 128)).set_image_filter(
                        ImageFilter::Blur {
                            sigma_xy: Xy::new(4.0, 4.0),
                            tile_mode: None,
                            input: None,
                            crop_rect: None,
                        },
                    )),
                },
            }))
            .add(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK));
        });

        
    }
}
