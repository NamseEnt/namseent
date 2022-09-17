use super::*;
use crate::{namui::*, text::*};

impl TextInput {
    /// Caret is drawn at the end of the text.
    pub(crate) fn draw_caret(
        &self,
        props: &Props,
        line_texts: &LineTexts,
        selection: &Selection,
    ) -> RenderingTree {
        if selection.is_none() {
            return RenderingTree::Empty;
        };
        let selection = selection.as_ref().unwrap();
        let caret = line_texts.get_multiline_caret(selection.end);

        let line_height = get_line_height(props.font_type.size);

        let multiline_y_baseline_offset = get_multiline_y_baseline_offset(
            props.text_baseline,
            line_height,
            line_texts.line_len(),
        );

        let y = props.text_y() + multiline_y_baseline_offset + line_height * caret.line_index;

        let right_new_line_by = line_texts.get_line(caret.line_index).unwrap().new_line_by;
        let line = {
            let line = line_texts
                .iter_str()
                .nth(caret.line_index)
                .unwrap()
                .to_string();
            match right_new_line_by {
                Some(new_line_by) => match new_line_by {
                    NewLineBy::Wrap => line,
                    NewLineBy::LineFeed => format!("{line} "),
                },
                None => line,
            }
        };
        let char_vec = line.chars().collect::<Vec<_>>();

        let left_text_string: String = char_vec[..caret.caret_index_in_line].iter().collect();
        let right_text_string: String = char_vec[caret.caret_index_in_line..].iter().collect();

        let font = namui::font::get_font(props.font_type);

        if font.is_none() {
            return RenderingTree::Empty;
        }
        let font = font.unwrap();

        let drop_shadow_x = props.text_style.drop_shadow.map(|shadow| shadow.x);

        let left_text_width = get_text_width_internal(&font, &left_text_string, drop_shadow_x);
        let right_text_width = get_text_width_internal(&font, &right_text_string, drop_shadow_x);

        let total_width = left_text_width + right_text_width;

        let left = match props.text_align {
            namui::TextAlign::Left => props.text_x() - 1.px(),
            namui::TextAlign::Center => props.text_x() - total_width / 2.0,
            namui::TextAlign::Right => props.text_x() - total_width + 1.px(),
        } + left_text_width;

        let font_metrics = font.metrics;
        let top = get_bottom_of_baseline(props.text_baseline, font_metrics)
            + font_metrics.ascent
            + font_metrics.descent
            + y;

        crate::rect(RectParam {
            rect: Rect::Xywh {
                x: left,
                y: top,
                width: 2.0.into(),
                height: line_height,
            },
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::grayscale_f01(0.5),
                }),
                ..Default::default()
            },
        })
    }
}
