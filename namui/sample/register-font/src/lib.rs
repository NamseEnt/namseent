use namui::*;

pub fn main() {
    namui::start(|| FontExample)
}

#[namui::component]
struct FontExample;

impl Component for FontExample {
    fn render(self, ctx: &RenderCtx)  {
        const TYPEFACE_NAME: &str = "MoiraiOne-Regular";

        let (loading, set_loading) = ctx.state(|| false);

        ctx.effect("load font", || {
            let set_loading = set_loading.cloned();
            namui::spawn(async move {
                set_loading.set(true);
                let font = namui::file::bundle::read("bundle:resources/MoiraiOne-Regular.ttf")
                    .await
                    .unwrap();
                typeface::register_typeface(TYPEFACE_NAME, &font);
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
                    margin: None,
                }),
                line_height_percent: 100.percent(),
                underline: None,
            },
            max_width: None,
        }));

        
    }
}
