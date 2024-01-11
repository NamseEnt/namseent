use namui::prelude::*;

#[component]
pub struct Drummer {
    pub wh: Wh<Px>,
}

impl Component for Drummer {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let Self { wh } = self;
        ctx.component(image(ImageParam {
            rect: Rect::zero_wh(wh),
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
