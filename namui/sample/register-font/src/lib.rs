use namui::*;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(FontExample);
    })
}

struct FontExample;

impl Component for FontExample {
    fn render(self, ctx: &RenderCtx) {
        const TYPEFACE_NAME: &str = "MoiraiOne-Regular";

        let (loading, set_loading) = ctx.state(|| false);

        ctx.effect("load font", || {
            namui::spawn(async move {
                set_loading.set(true);
                let font = namui::file::bundle::read("resources/MoiraiOne-Regular.ttf")
                    .await
                    .unwrap();
                typeface::register_typeface(TYPEFACE_NAME, font)
                    .await
                    .unwrap();
                set_loading.set(false);
            });
        });

        let text = match *loading {
            true => "Loading...",
            false => "Hello, world!",
        }
        .to_string();
        ctx.add(namui::text(TextParam {
            text,
            x: 0.px(),
            y: 0.px(),
            align: TextAlign::Left,
            baseline: TextBaseline::Top,
            font: Font {
                size: 48.int_px(),
                name: TYPEFACE_NAME.to_string(),
            },
            style: TextStyle {
                border: None,
                drop_shadow: None,
                color: Color::WHITE,
                background: Some(TextStyleBackground {
                    color: Color::BLACK,
                    ..Default::default()
                }),
                line_height_percent: 100.percent(),
                underline: None,
            },
            max_width: None,
        }));
    }
}
