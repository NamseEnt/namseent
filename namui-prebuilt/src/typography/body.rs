use super::*;

pub const FONT_SIZE: IntPx = int_px(12);

pub fn left(height: Px, text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: 0.px(),
        y: height / 2.0,
        align: TextAlign::Left,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::REGULAR,
            language: Language::Ko,
            serif: false,
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}
pub fn left_top(text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: 0.px(),
        y: 0.px(),
        align: TextAlign::Left,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::REGULAR,
            language: Language::Ko,
            serif: false,
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
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
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
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
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}

pub fn left_bold(height: Px, text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: 0.px(),
        y: height / 2.0,
        align: TextAlign::Left,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::BOLD,
            language: Language::Ko,
            serif: false,
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}
pub fn left_top_bold(text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: 0.px(),
        y: 0.px(),
        align: TextAlign::Left,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::BOLD,
            language: Language::Ko,
            serif: false,
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}
pub fn center_bold(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: wh.width / 2.0,
        y: wh.height / 2.0,
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::BOLD,
            language: Language::Ko,
            serif: false,
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}
pub fn right_bold(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: wh.width,
        y: wh.height / 2.0,
        align: TextAlign::Right,
        baseline: TextBaseline::Middle,
        font_type: FontType {
            font_weight: FontWeight::BOLD,
            language: Language::Ko,
            serif: false,
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}
