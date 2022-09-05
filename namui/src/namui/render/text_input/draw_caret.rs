use super::*;
use crate::{draw::text::*, namui::*, TextParam};

impl TextInput {
    /// Caret is drawn at the end of the text.
    pub(crate) fn draw_caret(&self, text_param: &TextParam) -> RenderingTree {
        let selection = &self.selection;
        if selection.is_none() {
            return RenderingTree::Empty;
        };
        let selection = selection.as_ref().unwrap();
        let line_texts = text_param.text.split_inclusive("\n").collect::<Vec<_>>();

        let caret = get_multiline_caret(selection.end, &text_param.text);

        let line_height = get_line_height(text_param.font_type.size);

        let multiline_y_baseline_offset =
            get_multiline_y_baseline_offset(text_param.baseline, line_height, line_texts.len());

        let y = text_param.y + multiline_y_baseline_offset + line_height * caret.line_index;
        let line_text = if caret.line_index == line_texts.len() {
            ""
        } else {
            line_texts[caret.line_index]
        };
        let char_vec = line_text.chars().collect::<Vec<_>>();

        let left_text_string: String = char_vec[..caret.caret_index_in_line].iter().collect();
        let right_text_string: String = char_vec[caret.caret_index_in_line..].iter().collect();

        let font = namui::font::get_font(text_param.font_type);

        if font.is_none() {
            return RenderingTree::Empty;
        }
        let font = font.unwrap();

        let drop_shadow_x = text_param.style.drop_shadow.map(|shadow| shadow.x);

        let left_text_width = get_text_width_internal(&font, &left_text_string, drop_shadow_x);
        let right_text_width = get_text_width_internal(&font, &right_text_string, drop_shadow_x);

        let total_width = left_text_width + right_text_width;

        let left = match text_param.align {
            namui::TextAlign::Left => text_param.x - 1.px(),
            namui::TextAlign::Center => text_param.x - total_width / 2.0,
            namui::TextAlign::Right => text_param.x - total_width + 1.px(),
        } + left_text_width;

        let font_metrics = font.metrics;
        let top = get_bottom_of_baseline(text_param.baseline, font_metrics)
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

pub struct MultilineCaret {
    pub line_index: usize,
    pub caret_index_in_line: usize,
}
pub fn get_multiline_caret(selection_index: usize, text: &str) -> MultilineCaret {
    let line_texts = text
        .split_inclusive("\n")
        .map(|text| text.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut line_index = 0;
    let mut line_start_character_index = 0;
    loop {
        if line_texts.len() == line_index {
            break MultilineCaret {
                line_index,
                caret_index_in_line: 0,
            };
        }
        let line_text = &line_texts[line_index];
        if selection_index < line_start_character_index + line_text.len()
            || !line_text.ends_with(&['\n'])
        {
            break MultilineCaret {
                line_index,
                caret_index_in_line: selection_index - line_start_character_index,
            };
        }
        line_index += 1;
        line_start_character_index += line_text.len();
    }
}
