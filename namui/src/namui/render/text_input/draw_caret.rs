use super::*;
use crate::{
    draw::text::get_bottom_of_baseline,
    namui::{self, get_text_width_internal, RenderingTree},
    TextParam,
};

impl TextInput {
    pub(crate) fn draw_caret(&self, text_param: &TextParam) -> RenderingTree {
        let selection = &self.selection;
        if selection.is_none() {
            return RenderingTree::Empty;
        };
        let selection = selection.as_ref().unwrap();

        let char_vec = text_param.text.chars().collect::<Vec<_>>();
        let char_vec_len = char_vec.len();
        let selection = selection.start.min(char_vec_len)..selection.end.min(char_vec_len); // Note: https://docs.google.com/document/d/1BYWl4DeCih52fjgxa8DHszISA3fOJxKrAtCYba6uJXE/edit#heading=h.xoyzkdbf352y
        let left_text_string: String = char_vec[..selection.end].iter().collect();
        let right_text_string: String = char_vec[selection.end..].iter().collect();

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
            namui::TextAlign::Left => text_param.x,
            namui::TextAlign::Center => text_param.x - total_width / 2.0,
            namui::TextAlign::Right => text_param.x - total_width,
        } + left_text_width;

        let font_metrics = font.metrics;
        let font_height = -font_metrics.ascent + font_metrics.descent;
        let top = get_bottom_of_baseline(&text_param.baseline, &font_metrics)
            + font_metrics.ascent
            + text_param.y;

        crate::rect(RectParam {
            x: left,
            y: top,
            width: 1.0,
            height: font_height,
            style: RectStyle {
                fill: Some(RectFill {
                    color: Color::grayscale_f01(0.5),
                }),
                ..Default::default()
            },
        })
    }
}
