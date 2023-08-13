use crate::*;

#[type_derives(-PartialEq, -serde::Serialize, -serde::Deserialize)]
pub struct Caret<'a> {
    pub line_index: usize,
    pub caret_index_in_line: usize,
    pub paragraph: &'a Paragraph,
}

pub(crate) fn get_caret<'a>(selection_index: usize, paragraph: &'a Paragraph) -> Caret<'a> {
    let mut line_index = 0;
    let mut left_index = selection_index;

    for line in paragraph.iter_lines() {
        if left_index < line.chars.len() {
            return Caret {
                line_index,
                caret_index_in_line: left_index,
                paragraph,
            };
        } else if left_index == line.chars.len() {
            match line.new_line_by {
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
            }
        } else {
            left_index -= line.chars.len();
            line_index += 1;
            if let Some(NewLineBy::LineFeed) = line.new_line_by {
                left_index -= 1;
            }
        }
    }
    Caret {
        line_index,
        caret_index_in_line: left_index,
        paragraph,
    }
}

#[type_derives(Copy)]
pub enum CaretKey {
    ArrowUp,
    ArrowDown,
    Home,
    End,
}

impl Caret<'_> {
    pub fn get_caret_on_key(&self, key: CaretKey) -> Caret {
        let (line_index, x) = match key {
            CaretKey::ArrowUp => {
                if self.line_index == 0 {
                    return Caret {
                        line_index: 0,
                        caret_index_in_line: 0,
                        paragraph: self.paragraph,
                    };
                }
                (self.line_index - 1, self.get_x())
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
                (self.line_index + 1, self.get_x())
            }
            CaretKey::Home => (self.line_index, 0.px()),
            CaretKey::End => (
                self.line_index,
                self.paragraph
                    .group_glyph
                    .width(&self.line_text(self.line_index).unwrap()),
            ),
        };

        let caret_index_on_direction = self.get_caret_index_on_x(x, line_index);

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

    fn get_x(&self) -> Px {
        let line_text = self.line_text(self.line_index).unwrap();

        self.paragraph
            .group_glyph
            .widths(line_text.as_str())
            .into_iter()
            .take(self.caret_index_in_line)
            .sum()
    }

    fn get_caret_index_on_x(&self, x: Px, line_index: usize) -> usize {
        let line_text = self.line_text(line_index).unwrap();

        let widths = self.paragraph.group_glyph.widths(line_text.as_str());

        let mut from_left = 0.px();
        let mut caret_index = 0;
        let mut cloest_distance = x;
        let mut closest_caret_index = 0;

        for width in widths.into_iter() {
            from_left += width;
            caret_index += 1;

            let distance = (x - from_left).abs();
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
