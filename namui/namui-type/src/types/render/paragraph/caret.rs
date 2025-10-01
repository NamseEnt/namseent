use crate::*;

#[derive(Debug, Clone)]
pub struct Caret<'a> {
    pub line_index: usize,
    pub caret_index_in_line: usize,
    pub paragraph: &'a Paragraph,
}

pub(crate) fn get_caret(selection_index: usize, paragraph: &Paragraph) -> Caret<'_> {
    let mut line_index = 0;
    let mut left_index = selection_index;

    for line in paragraph.iter_lines() {
        match left_index.cmp(&line.chars.len()) {
            std::cmp::Ordering::Less => {
                return Caret {
                    line_index,
                    caret_index_in_line: left_index,
                    paragraph,
                };
            }
            std::cmp::Ordering::Equal => match line.new_line_by {
                Some(new_line_by) => match new_line_by {
                    NewLineBy::Wrap => {
                        return Caret {
                            line_index: line_index + 1,
                            caret_index_in_line: 0,
                            paragraph,
                        };
                    }
                    NewLineBy::LineFeed => {
                        return Caret {
                            line_index,
                            caret_index_in_line: left_index,
                            paragraph,
                        };
                    }
                },
                None => {
                    return Caret {
                        line_index,
                        caret_index_in_line: left_index,
                        paragraph,
                    };
                }
            },
            std::cmp::Ordering::Greater => {
                left_index -= line.chars.len();
                line_index += 1;
                if let Some(NewLineBy::LineFeed) = line.new_line_by {
                    left_index -= 1;
                }
            }
        }
    }
    Caret {
        line_index,
        caret_index_in_line: left_index,
        paragraph,
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CaretKey {
    ArrowUp,
    ArrowDown,
    Home,
    End,
}

impl Caret<'_> {
    pub fn get_caret_on_key(
        &self,
        key: CaretKey,
        text_align: TextAlign,
        container_width: Px,
    ) -> Caret<'_> {
        let (line_index, x) = match key {
            CaretKey::ArrowUp => {
                if self.line_index == 0 {
                    return Caret {
                        line_index: 0,
                        caret_index_in_line: 0,
                        paragraph: self.paragraph,
                    };
                }
                (self.line_index - 1, self.get_x(text_align, container_width))
            }
            CaretKey::ArrowDown => {
                if self.is_at_bottom() {
                    return Caret {
                        line_index: self.line_index,
                        caret_index_in_line: self
                            .paragraph
                            .iter_chars()
                            .nth(self.line_index)
                            .unwrap()
                            .len(),
                        paragraph: self.paragraph,
                    };
                }
                (self.line_index + 1, self.get_x(text_align, container_width))
            }
            CaretKey::Home => (self.line_index, 0.px()),
            CaretKey::End => (
                self.line_index,
                self.paragraph
                    .group_glyph
                    .width(&self.line_text(self.line_index).unwrap()),
            ),
        };

        let caret_index_on_direction =
            self.get_caret_index_on_x(x, line_index, text_align, container_width);

        Caret {
            line_index,
            caret_index_in_line: caret_index_on_direction,
            paragraph: self.paragraph,
        }
    }

    pub fn to_selection_index(&self) -> usize {
        let index_before_line = self.paragraph.char_index_before_line(self.line_index);
        index_before_line + self.caret_index_in_line
    }

    fn line_text(&self, index: usize) -> Option<String> {
        self.paragraph.iter_str().nth(index)
    }

    fn get_x(&self, text_align: TextAlign, container_width: Px) -> Px {
        let line_text = self.line_text(self.line_index).unwrap();
        let widths = self.paragraph.group_glyph.widths(line_text.as_str());

        impl_get_x(
            text_align,
            &widths,
            self.caret_index_in_line,
            container_width,
        )
    }

    fn get_caret_index_on_x(
        &self,
        x: Px,
        line_index: usize,
        text_align: TextAlign,
        container_width: Px,
    ) -> usize {
        let line_text = self.line_text(line_index).unwrap();

        let widths = self.paragraph.group_glyph.widths(line_text.as_str());

        let mut cloest_distance = x;
        let mut closest_caret_index = 0;
        for caret_index in 0..widths.len() {
            let x_in_line = impl_get_x(text_align, &widths, caret_index, container_width);
            let distance = (x - x_in_line).abs();
            if distance < cloest_distance {
                cloest_distance = distance;
                closest_caret_index = caret_index;
            }
        }

        closest_caret_index
    }

    pub fn is_at_bottom(&self) -> bool {
        let line_count = self.paragraph.iter_str().count();
        if line_count == 0 {
            return true;
        }
        self.line_index == line_count - 1
    }
}

fn impl_get_x(
    text_align: TextAlign,
    widths: &[Px],
    caret_index_in_line: usize,
    container_width: Px,
) -> Px {
    match text_align {
        TextAlign::Left => widths.iter().take(caret_index_in_line).sum::<Px>(),
        TextAlign::Center => {
            (container_width - widths.iter().sum::<Px>()) / 2.0
                + widths.iter().take(caret_index_in_line).sum::<Px>()
        }
        TextAlign::Right => container_width - widths.iter().skip(caret_index_in_line).sum::<Px>(),
    }
}
