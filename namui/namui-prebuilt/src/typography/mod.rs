pub mod body;
pub mod effect;
pub mod title;

use crate::*;
use namui::*;

pub fn center_text(
    wh: Wh<Px>,
    text: impl AsRef<str>,
    color: Color,
    font_size: IntPx,
) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: wh.width / 2.0,
        y: wh.height / 2.0,
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font: Font {
            name: "NotoSansKR-Regular".to_string(),
            size: font_size,
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}

pub fn center_text_full_height(wh: Wh<Px>, text: impl AsRef<str>, color: Color) -> RenderingTree {
    namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: wh.width / 2.0,
        y: wh.height / 2.0,
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font: Font {
            name: "NotoSansKR-Regular".to_string(),
            size: adjust_font_size(wh.height),
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    })
}

pub fn text_fit(
    height: Px,
    text: impl AsRef<str>,
    color: Color,
    side_padding: Px,
) -> namui::RenderingTree {
    let center_text = namui::text(TextParam {
        text: String::from(text.as_ref()),
        x: 0.px(),
        y: height / 2.0,
        align: TextAlign::Center,
        baseline: TextBaseline::Middle,
        font: Font {
            name: "NotoSansKR-Regular".to_string(),
            size: adjust_font_size(height),
        },
        style: TextStyle {
            color,
            ..Default::default()
        },
        max_width: None,
    });

    let width = match center_text.bounding_box() {
        Some(bounding_box) => bounding_box.width(),
        None => return RenderingTree::Empty,
    };

    render([
        simple_rect(
            Wh::new(width + side_padding * 2, height),
            Color::TRANSPARENT,
            0.px(),
            Color::TRANSPARENT,
        ),
        translate(width / 2 + side_padding, 0.px(), center_text),
    ])
}

pub fn adjust_font_size(height: Px) -> IntPx {
    // 0, 4, 8, 16, 20, ...
    let mut font_size: Px = height * 0.7;
    font_size -= font_size % 4;

    font_size.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjust_font_size_should_work() {
        assert_eq!(adjust_font_size(10.0.px()), 4.int_px());
        assert_eq!(adjust_font_size(20.0.px()), 12.int_px());
        assert_eq!(adjust_font_size(30.0.px()), 20.int_px());
    }
}
