use super::SequenceList;
use namui::{
    Color, FontType, FontWeight, Language, RenderingTree, TextAlign, TextBaseline, TextStyle, Wh,
};

impl SequenceList {
    pub fn render_button_text(&self, wh: Wh<f32>, text: String) -> RenderingTree {
        namui::text(namui::TextParam {
            text,
            x: wh.width / 2.0,
            y: wh.height / 2.0,
            align: TextAlign::Center,
            baseline: TextBaseline::Middle,
            font_type: FontType {
                serif: false,
                size: (wh.height / 3.0 * 2.0) as i16,
                language: Language::Ko,
                font_weight: FontWeight::REGULAR,
            },
            style: TextStyle {
                color: Color::from_u8(255, 255, 255, 255),
                ..Default::default()
            },
        })
    }
}
