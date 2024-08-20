use namui::*;

pub struct TextEditor<'a> {
    pub wh: Wh<Px>,
    pub text: &'a String,
    pub on_edit_done: &'a dyn Fn(String),
}

impl Component for TextEditor<'_> {
    fn render(self, ctx: &RenderCtx) {
        let Self {
            wh,
            text,
            on_edit_done,
        } = self;

        ctx.add(text_input::TextInput {
            rect: Rect::zero_wh(wh),
            start_text: text,
            text_align: TextAlign::Left,
            text_baseline: TextBaseline::Top,
            font: Font {
                size: 16.int_px(),
                name: "NotoSansKR-Regular".to_string(),
            },
            style: Default::default(),
            prevent_default_codes: &[],
            focus: None,
            on_edit_done,
        });
    }
}
