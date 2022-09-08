use super::Props;
use crate::{
    namui::{self, RenderingTree, TextInput},
    render,
    text::*,
    Font, Paint, Px, TextParam, TextStyleBackground,
};
use std::sync::Arc;

impl TextInput {
    pub(crate) fn draw_texts_divided_by_selection(
        &self,
        props: &Props,
        fonts: &Vec<Arc<Font>>,
        paint: &Arc<Paint>,
        line_texts: &LineTexts,
    ) -> RenderingTree {
        let is_not_divided_by_selection = self
            .selection
            .as_ref()
            .map_or(true, |selection| selection.start == selection.end);

        if is_not_divided_by_selection {
            return namui::text(props.text_param());
        };

        let selection = self.selection.as_ref().unwrap();

        let (left_selection_index, right_selection_index) = if selection.start < selection.end {
            (selection.start, selection.end)
        } else {
            (selection.end, selection.start)
        };

        let left_caret = line_texts.get_multiline_caret(left_selection_index);
        let right_caret = line_texts.get_multiline_caret(right_selection_index);

        let render_checking_background_order =
            |render_only_selection_background: bool| -> RenderingTree {
                let non_selected_upper_lines = if render_only_selection_background {
                    RenderingTree::Empty
                } else {
                    render((0..left_caret.line_index).map(|line_index| {
                        let y = props.rect.y() + get_line_height(props.font_type.size) * line_index;
                        let line_text = &line_texts.iter_str().nth(line_index).unwrap();
                        crate::text(TextParam {
                            y,
                            text: line_text.to_string(),
                            ..props.text_param()
                        })
                    }))
                };

                let is_single_line = left_caret.line_index == right_caret.line_index;
                let selected_lines = if is_single_line {
                    let y = props.rect.y()
                        + get_line_height(props.font_type.size) * left_caret.line_index;
                    self.render_single_line(
                        &props,
                        &fonts,
                        &line_texts.iter_chars().nth(left_caret.line_index).unwrap(),
                        y,
                        left_caret.caret_index_in_line,
                        right_caret.caret_index_in_line,
                        render_only_selection_background,
                        false,
                        &paint,
                    )
                } else {
                    let line_indexes_in_the_middle =
                        left_caret.line_index + 1..right_caret.line_index;

                    let first_line = {
                        let y = props.rect.y()
                            + get_line_height(props.font_type.size) * left_caret.line_index;

                        let first_line_text_with_newline =
                            line_texts.iter_chars().nth(left_caret.line_index).unwrap();

                        self.render_single_line(
                            &props,
                            &fonts,
                            &first_line_text_with_newline,
                            y,
                            left_caret.caret_index_in_line,
                            first_line_text_with_newline.len(),
                            render_only_selection_background,
                            true,
                            &paint,
                        )
                    };

                    let middle_lines = line_indexes_in_the_middle.map(|line_index| {
                        let y = props.rect.y() + get_line_height(props.font_type.size) * line_index;

                        let line_text_with_newline =
                            line_texts.iter_chars().nth(line_index).unwrap();

                        self.render_single_line(
                            &props,
                            &fonts,
                            &line_text_with_newline,
                            y,
                            0,
                            line_text_with_newline.len(),
                            render_only_selection_background,
                            true,
                            &paint,
                        )
                    });

                    let last_line = {
                        let y = props.rect.y()
                            + get_line_height(props.font_type.size) * right_caret.line_index;

                        let text = if line_texts.line_len() == right_caret.line_index {
                            "".chars().collect()
                        } else {
                            line_texts
                                .iter_chars()
                                .nth(right_caret.line_index)
                                .unwrap()
                                .clone()
                        };

                        self.render_single_line(
                            &props,
                            &fonts,
                            &text,
                            y,
                            0,
                            right_caret.caret_index_in_line,
                            render_only_selection_background,
                            false,
                            &paint,
                        )
                    };

                    render([first_line, render(middle_lines), last_line])
                };

                let non_selected_lower_lines = if render_only_selection_background {
                    RenderingTree::Empty
                } else {
                    render(
                        (right_caret.line_index + 1..line_texts.line_len()).map(|line_index| {
                            let y =
                                props.rect.y() + get_line_height(props.font_type.size) * line_index;
                            let line_text = &line_texts.iter_str().nth(line_index).unwrap();
                            crate::text(TextParam {
                                y,
                                text: line_text.clone(),
                                ..props.text_param()
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
        props: &Props,
        fonts: &Vec<Arc<Font>>,
        text: &Vec<char>,
        y: Px,
        left_caret_index: usize,
        right_caret_index: usize,
        render_only_selection_background: bool,
        with_newline_background: bool,
        paint: &Arc<Paint>,
    ) -> RenderingTree {
        let (left_text_string, selected_text_string, right_text_string) = (
            &text[..left_caret_index].iter().collect::<String>(),
            &text[left_caret_index..right_caret_index]
                .iter()
                .collect::<String>(),
            &text[right_caret_index..].iter().collect::<String>(),
        );
        let (left_text_left, selected_text_left, right_text_left) = self.get_text_lefts(
            &props,
            fonts,
            left_text_string,
            selected_text_string,
            right_text_string,
            paint,
        );

        if render_only_selection_background {
            let left = selected_text_left;
            let top = y;
            let line_height = get_line_height(props.font_type.size);

            let mut width = get_text_width_with_fonts(fonts, &selected_text_string, paint);
            if with_newline_background {
                width += get_text_width_with_fonts(fonts, " ", paint)
            };

            namui::rect(crate::RectParam {
                rect: crate::Rect::Xywh {
                    x: left,
                    y: top,
                    width,
                    height: line_height,
                },
                style: crate::RectStyle {
                    stroke: None,
                    fill: Some(crate::RectFill {
                        color: namui::Color::BLUE,
                    }),
                    round: None,
                },
            })
        } else {
            let left_text_text_param = namui::TextParam {
                x: left_text_left,
                y,
                text: left_text_string.to_string(),
                align: crate::TextAlign::Left,
                ..props.text_param()
            };

            let selected_text_text_param = namui::TextParam {
                x: selected_text_left,
                y,
                text: selected_text_string.to_string(),
                style: namui::TextStyle {
                    color: namui::Color::WHITE,
                    background: Some(TextStyleBackground {
                        color: namui::Color::TRANSPARENT,
                        ..Default::default()
                    }),
                    ..left_text_text_param.style
                },
                align: crate::TextAlign::Left,
                ..props.text_param()
            };
            let right_text_text_param = namui::TextParam {
                x: right_text_left,
                y,
                text: right_text_string.to_string(),
                align: crate::TextAlign::Left,
                ..props.text_param()
            };

            let left_text = namui::text(left_text_text_param);
            let selected_text = namui::text(selected_text_text_param);
            let right_text = namui::text(right_text_text_param);

            render([left_text, selected_text, right_text])
        }
    }

    fn get_text_lefts(
        &self,
        props: &Props,
        fonts: &Vec<Arc<Font>>,
        left_text_string: &str,
        selected_text_string: &str,
        right_text_string: &str,
        paint: &Arc<Paint>,
    ) -> (Px, Px, Px) {
        let (left_text_width, selected_text_width, right_text_width) = (
            get_text_width_with_fonts(&fonts, left_text_string, paint),
            get_text_width_with_fonts(&fonts, selected_text_string, paint),
            get_text_width_with_fonts(&fonts, right_text_string, paint),
        );

        let total_width = left_text_width + selected_text_width + right_text_width;

        let result = (
            props.text_x(),
            props.text_x() + left_text_width,
            props.text_x() + left_text_width + selected_text_width,
        );

        match props.text_align {
            namui::TextAlign::Left => result,
            namui::TextAlign::Center => (
                result.0 - total_width / 2.0,
                result.1 - total_width / 2.0,
                result.2 - total_width / 2.0,
            ),
            namui::TextAlign::Right => (
                result.0 - total_width,
                result.1 - total_width,
                result.2 - total_width,
            ),
        }
    }
}
