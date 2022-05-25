use crate::{
    namui::{self, get_text_width_internal, RenderingTree, TextInput},
    render, TextParam, TextStyleBackground,
};

impl TextInput {
    pub(crate) fn draw_texts_divided_by_selection(&self, text_param: TextParam) -> RenderingTree {
        let is_not_divided_by_selection = self
            .selection
            .as_ref()
            .map_or(true, |selection| selection.start == selection.end);

        if is_not_divided_by_selection {
            return namui::text(text_param);
        };

        let selection = self.selection.as_ref().unwrap();

        let (left_selection_index, right_selection_index) = if selection.start < selection.end {
            (selection.start, selection.end)
        } else {
            (selection.end, selection.start)
        };

        let (left_text_string, selected_text_string, right_text_string) = (
            &text_param.text[..left_selection_index],
            &text_param.text[left_selection_index..right_selection_index],
            &text_param.text[right_selection_index..],
        );

        let result = self.get_text_lefts(
            &text_param,
            left_text_string,
            selected_text_string,
            right_text_string,
        );
        if result.is_none() {
            return RenderingTree::Empty;
        };

        let (left_text_left, selected_text_left, right_text_left) = result.unwrap();

        let left_text_text_param = namui::TextParam {
            x: left_text_left,
            text: left_text_string.to_string(),
            align: crate::TextAlign::Left,
            ..text_param
        };

        let selected_text_text_param = namui::TextParam {
            x: selected_text_left,
            text: selected_text_string.to_string(),
            style: namui::TextStyle {
                color: namui::Color::WHITE,
                background: Some(TextStyleBackground {
                    color: namui::Color::BLUE,
                    ..Default::default()
                }),
                ..left_text_text_param.style
            },
            align: crate::TextAlign::Left,
            ..text_param
        };
        let right_text_text_param = namui::TextParam {
            x: right_text_left,
            text: right_text_string.to_string(),
            align: crate::TextAlign::Left,
            ..text_param
        };

        let left_text = namui::text(left_text_text_param);
        let selected_text = namui::text(selected_text_text_param);
        let right_text = namui::text(right_text_text_param);

        return render![left_text, selected_text, right_text];
    }

    fn get_text_lefts(
        &self,
        text_param: &TextParam,
        left_text_string: &str,
        selected_text_string: &str,
        right_text_string: &str,
    ) -> Option<(f32, f32, f32)> {
        let font = namui::managers()
            .font_manager
            .get_font(&text_param.font_type);

        if font.is_none() {
            return None;
        }
        let font = font.unwrap();

        let drop_shadow_x = text_param.style.drop_shadow.map(|shadow| shadow.x);

        let (left_text_width, selected_text_width, right_text_width) = (
            get_text_width_internal(&font, left_text_string, drop_shadow_x),
            get_text_width_internal(&font, selected_text_string, drop_shadow_x),
            get_text_width_internal(&font, right_text_string, drop_shadow_x),
        );

        let total_width = left_text_width + selected_text_width + right_text_width;

        let result = (
            text_param.x,
            text_param.x + left_text_width,
            text_param.x + left_text_width + selected_text_width,
        );

        match text_param.align {
            namui::TextAlign::Left => Some(result),
            namui::TextAlign::Center => Some((
                result.0 - total_width / 2.0,
                result.1 - total_width / 2.0,
                result.2 - total_width / 2.0,
            )),
            namui::TextAlign::Right => Some((
                result.0 - total_width,
                result.1 - total_width,
                result.2 - total_width,
            )),
        }
    }
}
