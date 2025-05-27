use namui::*;
use std::ops::Deref;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(App);
    })
}

struct App;

impl Component for App {
    fn render(self, ctx: &RenderCtx) {
        let size = namui::screen::size();
        let (image_fit, set_image_fit) = ctx.state(|| ImageFit::Fill);
        let (image_location, set_image_location) =
            ctx.state(|| ResourceLocation::bundle("resources/0.jpg"));
        let image = ctx.image(&image_location);

        let image_xy = size.into_type::<Px>().to_xy() / 4.0;
        let image_wh = size.into_type::<Px>() / 2.0;

        ctx.add(namui_prebuilt::typography::center_text(
            size.into_type(),
            format!(
                "current: {:?}\n1: Fill, 2: Contain, 3: Cover, 4: ScaleDown, 5: None\nr: rotate",
                *image_fit
            ),
            Color::RED,
            16.int_px(),
        ));

        ctx.compose(|ctx| {
            let Some(Ok(image)) = image.deref() else {
                return;
            };
            ctx.add(
                namui::image(ImageParam {
                    rect: Rect::from_xy_wh(image_xy, image_wh),
                    image: image.clone(),
                    style: ImageStyle {
                        fit: *image_fit,
                        paint: None,
                    },
                })
                .attach_event(|event| {
                    if let Event::KeyDown { event } = event {
                        match event.code {
                            Code::Digit1 => set_image_fit.set(ImageFit::Fill),
                            Code::Digit2 => set_image_fit.set(ImageFit::Contain),
                            Code::Digit3 => set_image_fit.set(ImageFit::Cover),
                            Code::Digit4 => set_image_fit.set(ImageFit::ScaleDown),
                            Code::Digit5 => set_image_fit.set(ImageFit::None),
                            Code::KeyR => {
                                if *image_location == ResourceLocation::bundle("resources/0.jpg") {
                                    set_image_location
                                        .set(ResourceLocation::bundle("resources/rotated.jpg"));
                                } else {
                                    set_image_location
                                        .set(ResourceLocation::bundle("resources/0.jpg"));
                                }
                            }
                            _ => {}
                        }
                    }
                }),
            );
        });

        ctx.add(namui::rect(RectParam {
            rect: Rect::from_xy_wh(image_xy, image_wh),
            style: RectStyle {
                stroke: Some(RectStroke {
                    color: Color::RED,
                    width: 2.px(),
                    border_position: BorderPosition::Middle,
                }),
                fill: None,
                round: None,
            },
        }));
    }
}
