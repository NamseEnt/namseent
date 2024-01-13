use namui::prelude::*;

#[component]
pub struct Drummer {
    pub wh: Wh<Px>,
}

impl Component for Drummer {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;
        let image_wh = Wh::new(wh.width, wh.width * 0.5);

        ctx.component(image(ImageParam {
            rect: Rect::Xywh {
                x: 0.px(),
                y: wh.height - image_wh.height,
                width: image_wh.width,
                height: image_wh.height,
            },
            source: ImageSource::Url {
                url: Url::parse("bundle:ui/drummer/drummer.png").unwrap(),
            },
            style: ImageStyle {
                fit: ImageFit::Contain,
                paint: None,
            },
        }));

        ctx.done()
    }
}
