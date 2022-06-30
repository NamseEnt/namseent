use namui::prelude::*;

pub fn center_text(wh: Wh<f32>, text: &str, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: text.to_string(),
        x: wh.width / 2.0,
        y: wh.height / 2.0,
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::REGULAR,
            language: Language::Ko,
            serif: false,
            size: adjust_font_size(wh.height),
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
    })
}

fn adjust_font_size(height: f32) -> i16 {
    // 0, 4, 8, 16, 20, ...
    let mut font_size = (height * 0.7) as i16;
    font_size -= font_size % 4;
    font_size
}
