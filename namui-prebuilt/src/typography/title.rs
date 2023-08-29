use super::*;

pub const FONT_SIZE: IntPx = int_px(20);

pub fn left(height: Px, text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: px(0.0),
        y: height / 2.0,
        align: TextAlign::Left,
        baseline: TextBaseline::Middle,
        font: Font {
            name: "NotoSansKR-Bold".to_string(),
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
        baseline: TextBaseline::Top,
        font: Font {
            name: "NotoSansKR-Bold".to_string(),
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
        font: Font {
            name: "NotoSansKR-Bold".to_string(),
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
        font: Font {
            name: "NotoSansKR-Bold".to_string(),
            size: FONT_SIZE,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}
