use super::*;
use crate::*;

impl TextInput<'_> {
    /// Caret is drawn at the end of the text.
    pub(crate) fn draw_caret(
        &self,
        props: &TextInput,
        paragraph: &Paragraph,
        selection: &Selection,
    ) -> RenderingTree {
        let Selection::Range(range) = selection else {
            return RenderingTree::Empty;
        };
        let caret = paragraph.caret(range.end);

        let line_height = props.line_height_px();

        let multiline_y_baseline_offset =
            get_multiline_y_baseline_offset(props.text_baseline, line_height, paragraph.line_len());

        let y = props.text_y() + multiline_y_baseline_offset + line_height * caret.line_index;

        let right_new_line_by = paragraph.get_line(caret.line_index).unwrap().new_line_by;
        let line = {
            let line = paragraph
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

        let left_text_width = paragraph.group_glyph.width(&left_text_string);
        let right_text_width = paragraph.group_glyph.width(&right_text_string);

        let total_width = left_text_width + right_text_width;

        let left = match props.text_align {
            TextAlign::Left => props.text_x() - 1.px(),
            TextAlign::Center => props.text_x() - total_width / 2.0,
            TextAlign::Right => props.text_x() - total_width + 1.px(),
        } + left_text_width;

        let font_metrics = paragraph.group_glyph.font_metrics();
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
