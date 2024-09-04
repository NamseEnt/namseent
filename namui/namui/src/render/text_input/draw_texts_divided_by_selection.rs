use super::*;
use crate::*;

impl TextInput<'_> {
    pub(crate) fn draw_texts_divided_by_selection(
        &self,
        paragraph: &Paragraph,
        selection: &Selection,
        text: &str,
    ) -> RenderingTree {
        let is_not_divided_by_selection =
            selection.map_or(true, |selection| selection.start == selection.end);

        if is_not_divided_by_selection {
            return super::text(self.text_param(text));
        };

        let Selection::Range(selection) = selection else {
            return RenderingTree::Empty;
        };

        let (left_selection_index, right_selection_index) = if selection.start < selection.end {
            (selection.start, selection.end)
        } else {
            (selection.end, selection.start)
        };

        let left_caret = paragraph.caret(left_selection_index);
        let right_caret = paragraph.caret(right_selection_index);

        let y_of_line = |line_index: usize| {
            let line_height = self.line_height_px();

            let multiline_y_baseline_offset = get_multiline_y_baseline_offset(
                self.text_baseline,
                line_height,
                paragraph.line_len(),
            );

            let line_height_from_top = line_height * line_index;

            self.text_y() + multiline_y_baseline_offset + line_height_from_top
        };

        let render_checking_background_order =
            |render_only_selection_background: bool| -> RenderingTree {
                let non_selected_upper_lines = if render_only_selection_background {
                    RenderingTree::Empty
                } else {
                    render((0..left_caret.line_index).map(|line_index| {
                        let y = y_of_line(line_index);
                        let line = &paragraph.iter_str().nth(line_index).unwrap();
                        crate::text(TextParam {
                            y,
                            text: line.to_string(),
                            ..self.text_param(text)
                        })
                    }))
                };

                let is_single_line = left_caret.line_index == right_caret.line_index;
                let selected_lines = if is_single_line {
                    let y = y_of_line(left_caret.line_index);
                    self.render_single_line(
                        y,
                        CaretDevidedStrings::new(
                            paragraph.iter_chars().nth(left_caret.line_index).unwrap(),
                            left_caret.caret_index_in_line,
                            right_caret.caret_index_in_line,
                        ),
                        render_only_selection_background,
                        false,
                        paragraph,
                        text,
                    )
                } else {
                    let line_indexes_in_the_middle =
                        left_caret.line_index + 1..right_caret.line_index;

                    let first_line = {
                        let y = y_of_line(left_caret.line_index);

                        let first_line_with_newline =
                            paragraph.iter_chars().nth(left_caret.line_index).unwrap();

                        self.render_single_line(
                            y,
                            CaretDevidedStrings::new(
                                first_line_with_newline,
                                left_caret.caret_index_in_line,
                                first_line_with_newline.len(),
                            ),
                            render_only_selection_background,
                            true,
                            paragraph,
                            text,
                        )
                    };

                    let middle_lines = line_indexes_in_the_middle.map(|line_index| {
                        let y = y_of_line(line_index);

                        let line_with_newline = paragraph.iter_chars().nth(line_index).unwrap();

                        self.render_single_line(
                            y,
                            CaretDevidedStrings::new(line_with_newline, 0, line_with_newline.len()),
                            render_only_selection_background,
                            true,
                            paragraph,
                            text,
                        )
                    });

                    let last_line = {
                        let y = y_of_line(right_caret.line_index);

                        let default = vec![];

                        let chars = paragraph
                            .iter_chars()
                            .nth(right_caret.line_index)
                            .unwrap_or(&default);

                        self.render_single_line(
                            y,
                            CaretDevidedStrings::new(chars, 0, right_caret.caret_index_in_line),
                            render_only_selection_background,
                            false,
                            paragraph,
                            text,
                        )
                    };

                    render([first_line, render(middle_lines), last_line])
                };

                let non_selected_lower_lines = if render_only_selection_background {
                    RenderingTree::Empty
                } else {
                    render(
                        (right_caret.line_index + 1..paragraph.line_len()).map(|line_index| {
                            let y = y_of_line(line_index);
                            let line = &paragraph.iter_str().nth(line_index).unwrap();
                            crate::text(TextParam {
                                y,
                                text: line.clone(),
                                ..self.text_param(text)
                            })
                        }),
                    )
                };

                render([
                    non_selected_upper_lines,
                    selected_lines,
                    non_selected_lower_lines,
                ])
            };

        render([
            render_checking_background_order(true),
            render_checking_background_order(false),
        ])
    }

    fn render_single_line(
        &self,
        y: Px,
        caret_devided_strings: CaretDevidedStrings,
        render_only_selection_background: bool,
        with_newline_background: bool,
        paragraph: &Paragraph,
        text: &str,
    ) -> RenderingTree {
        let (left_text_left, selected_text_left, right_text_left) = self.get_text_lefts(
            &caret_devided_strings.left,
            &caret_devided_strings.selected,
            &caret_devided_strings.right,
            paragraph,
        );

        if render_only_selection_background {
            let line_height = self.line_height_px();
            let left = selected_text_left;
            let top = y - match self.text_baseline {
                TextBaseline::Top => 0.px(),
                TextBaseline::Middle => line_height / 2,
                TextBaseline::Bottom => line_height,
            };

            let mut width = paragraph.group_glyph.width(&caret_devided_strings.selected);
            if with_newline_background {
                width += paragraph.group_glyph.width(" ");
            };

            rect(crate::RectParam {
                rect: crate::Rect::Xywh {
                    x: left,
                    y: top,
                    width,
                    height: line_height,
                },
                style: crate::RectStyle {
                    stroke: None,
                    fill: Some(crate::RectFill { color: Color::BLUE }),
                    round: None,
                },
            })
        } else {
            let left_text_text_param = TextParam {
                x: left_text_left,
                y,
                text: caret_devided_strings.left,
                align: crate::TextAlign::Left,
                ..self.text_param(text)
            };

            let selected_text_text_param = TextParam {
                x: selected_text_left,
                y,
                text: caret_devided_strings.selected,
                style: TextStyle {
                    color: Color::WHITE,
                    background: Some(TextStyleBackground {
                        color: Color::TRANSPARENT,
                        ..Default::default()
                    }),
                    ..left_text_text_param.style.clone()
                },
                align: crate::TextAlign::Left,
                ..self.text_param(text)
            };
            let right_text_text_param = TextParam {
                x: right_text_left,
                y,
                text: caret_devided_strings.right,
                align: crate::TextAlign::Left,
                ..self.text_param(text)
            };

            let left_text = super::text(left_text_text_param);
            let selected_text = super::text(selected_text_text_param);
            let right_text = super::text(right_text_text_param);

            render([left_text, selected_text, right_text])
        }
    }

    fn get_text_lefts(
        &self,
        left_text_string: &str,
        selected_text_string: &str,
        right_text_string: &str,
        paragraph: &Paragraph,
    ) -> (Px, Px, Px) {
        let (left_text_width, selected_text_width, right_text_width) = (
            paragraph.group_glyph.width(left_text_string),
            paragraph.group_glyph.width(selected_text_string),
            paragraph.group_glyph.width(right_text_string),
        );

        let total_width = left_text_width + selected_text_width + right_text_width;

        let result = (
            self.text_x(),
            self.text_x() + left_text_width,
            self.text_x() + left_text_width + selected_text_width,
        );

        match self.text_align {
            TextAlign::Left => result,
            TextAlign::Center => (
                result.0 - total_width / 2.0,
                result.1 - total_width / 2.0,
                result.2 - total_width / 2.0,
            ),
            TextAlign::Right => (
                result.0 - total_width,
                result.1 - total_width,
                result.2 - total_width,
            ),
        }
    }
}

struct CaretDevidedStrings {
    left: String,
    selected: String,
    right: String,
}

impl CaretDevidedStrings {
    fn new(chars: &[char], left_caret_index: usize, right_caret_index: usize) -> Self {
        let (left_text_string, selected_text_string, right_text_string) = (
            chars[..left_caret_index].iter().collect::<String>(),
            chars[left_caret_index..right_caret_index]
                .iter()
                .collect::<String>(),
            chars[right_caret_index..].iter().collect::<String>(),
        );

        Self {
            left: left_text_string,
            selected: selected_text_string,
            right: right_text_string,
        }
    }
}
