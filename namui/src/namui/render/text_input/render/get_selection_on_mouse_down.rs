use crate::namui::{self, get_text_width, managers, render::Selection, TextInput};

pub(crate) fn get_selection_on_mouse_down(
    click_x: f32,
    text_input: &TextInput,
) -> Result<Selection, ()> {
    let mut managers = managers();
    let font = managers.font_manager.get_font(&text_input.font_type);
    if font.is_none() {
        return Err(());
    };
    let font = font.unwrap();

    // TODO : drag selection.

    //     const isShiftKeyPressed = namui.keyboard.anyCodePress([
    //       Code.ShiftLeft,
    //       Code.ShiftRight,
    //     ]);
    let is_shift_key_pressed = managers
        .keyboard_manager
        .any_code_press(&[namui::Code::ShiftLeft, namui::Code::ShiftRight]);

    // const continouslyFastClickCount: number;

    // if (continouslyFastClickCount >= 3) {
    //   return getMoreTripleClickSelection({ text });
    // }
    // if (continouslyFastClickCount === 2) {
    //   return getDoubleClickSelection({ text, font, x: localX });
    // }

    //     const textWidth = getTextWidth(font, text, param.dropShadowX);
    let text_width = get_text_width(
        &font,
        &text_input.text,
        text_input.text_style.drop_shadow.map(|shadow| shadow.x),
    );

    let aligned_x = match text_input.text_align {
        namui::TextAlign::Left => click_x,
        namui::TextAlign::Center => click_x - (text_input.width / 2.0 - text_width / 2.0),
        namui::TextAlign::Right => text_width - (text_input.width - click_x),
    };

    Ok(get_one_click_selection(
        &text_input.text,
        &font,
        aligned_x,
        is_shift_key_pressed,
        text_input.selection.as_ref(),
    ))
}

fn get_one_click_selection(
    text: &str,
    font: &namui::Font,
    x: f32,
    is_shift_key_pressed: bool,
    last_selection: Option<&Selection>,
) -> Selection {
    let selection_index_of_x = get_selection_index_of_x(font, text, x);

    let start = if last_selection.is_none() || !is_shift_key_pressed {
        selection_index_of_x
    } else {
        last_selection.unwrap().start
    };

    Selection {
        start,
        end: selection_index_of_x,
    }
}

fn get_selection_index_of_x(font: &namui::Font, text: &str, x: f32) -> usize {
    let glyph_ids = font.get_glyph_ids(text);
    let glyph_widths = font.get_glyph_widths(&glyph_ids, None);

    let mut left = 0.0;
    let index = glyph_widths.iter().position(|width| {
        let center = left + width / 2.0;
        if x < center {
            return true;
        }
        left += width;
        return false;
    });

    return index.unwrap_or(text.len());
}
