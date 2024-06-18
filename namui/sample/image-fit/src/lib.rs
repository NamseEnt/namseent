use namui::*;

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
        let (image_url, set_image_url) =
            ctx.state(|| Url::parse("bundle:resources/0.jpg").unwrap());

        let image_xy = size.into_type::<Px>().as_xy() / 4.0;
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

        ctx.add(
            namui::image(ImageParam {
                rect: Rect::from_xy_wh(image_xy, image_wh),
                source: ImageSource::Url {
                    url: image_url.to_string(),
                },
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
                            if *image_url == Url::parse("bundle:resources/0.jpg").unwrap() {
                                set_image_url
                                    .set(Url::parse("bundle:resources/rotated.jpg").unwrap());
                            } else {
                                set_image_url.set(Url::parse("bundle:resources/0.jpg").unwrap());
                            }
                        }
                        _ => {}
                    }
                }
            }),
        );

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
