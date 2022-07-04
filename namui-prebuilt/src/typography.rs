use namui::prelude::*;

pub fn center_text(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
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

pub mod body {
    use super::*;
    pub fn left(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
        namui::text(TextParam {
            text: String::from(text.as_ref()),
            x: px(0.0),
            y: wh.height / 2.0,
            align: TextAlign::Left,
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
    pub fn center(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
        namui::text(TextParam {
            text: String::from(text.as_ref()),
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
    pub fn right(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
        namui::text(TextParam {
            text: String::from(text.as_ref()),
            x: wh.width,
            y: wh.height / 2.0,
            align: TextAlign::Right,
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
}

fn adjust_font_size(height: Px) -> IntPx {
    // 0, 4, 8, 16, 20, ...
    let mut font_size: Px = height * 0.7;
    font_size -= font_size % 4;
    font_size.into()
}
