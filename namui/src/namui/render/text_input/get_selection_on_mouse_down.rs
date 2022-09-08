use super::{selection_index::get_selection_index_of_x, Props};
use crate::{text::*, *};
use std::ops::Range;

impl TextInput {
    pub(crate) fn get_selection_on_mouse_movement(
        &self,
        props: &Props,
        click_local_xy: Xy<Px>,
        is_dragging_by_mouse: bool,
    ) -> Option<Range<usize>> {
        let font = crate::font::get_font(props.font_type);

        if font.is_none() {
            return None;
        };
        let font = font.unwrap();

        let is_shift_key_pressed =
            crate::keyboard::any_code_press([namui::Code::ShiftLeft, namui::Code::ShiftRight]);

        let fonts = std::iter::once(font.clone())
            .chain(std::iter::once_with(|| get_fallback_fonts(font.size)).flatten())
            .collect::<Vec<_>>();

        let paint = get_text_paint(props.text_style.color).build();

        let line_texts = LineTexts::new(&props.text, &fonts, &paint, Some(props.rect.width()));

        // const continouslyFastClickCount: number;

        // if (continouslyFastClickCount >= 3) {
        //   return getMoreTripleClickSelection({ text });
        // }
        // if (continouslyFastClickCount === 2) {
        //   return getDoubleClickSelection({ text, font, x: localX });
        // }

        let is_dragging = is_shift_key_pressed || is_dragging_by_mouse;

        Some(get_one_click_selection(
            props.text_align,
            &font,
            &line_texts,
            click_local_xy,
            is_dragging,
            &self.selection,
        ))
    }
}

fn get_one_click_selection(
    text_align: TextAlign,
    font: &namui::Font,
    line_texts: &LineTexts,
    click_local_xy: Xy<Px>,
    is_dragging: bool,
    last_selection: &Option<Range<usize>>,
) -> Range<usize> {
    let selection_index_of_xy =
        get_selection_index_of_x(text_align, font, line_texts, click_local_xy);

    let start = match last_selection {
        Some(last_selection) => {
            if !is_dragging {
                selection_index_of_xy
            } else {
                last_selection.start
            }
        }
        None => selection_index_of_xy,
    };

    start..selection_index_of_xy
}
