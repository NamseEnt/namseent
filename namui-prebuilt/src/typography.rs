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
        max_width: None,
    })
}

pub mod body {
    use super::*;

    const BODY_FONT_SIZE: IntPx = int_px(12);

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
                size: BODY_FONT_SIZE,
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
                size: BODY_FONT_SIZE,
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
                size: BODY_FONT_SIZE,
            },
            style: TextStyle {
                color,
                ..Default::default()
            },
            max_width: None,
        })
    }
}

fn adjust_font_size(height: Px) -> IntPx {
    // 0, 4, 8, 16, 20, ...
    let mut font_size: Px = height * 0.7;
    font_size -= font_size % 4;
    let result = font_size.into();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn adjust_font_size_should_work() {
        assert_eq!(adjust_font_size(10.0.px()), 4.int_px());
        assert_eq!(adjust_font_size(20.0.px()), 12.int_px());
        assert_eq!(adjust_font_size(30.0.px()), 20.int_px());
    }
}
