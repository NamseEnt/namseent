use crate::{
    system::text_input::{ArrowUpDown, KeyInInterest},
    text::*,
    *,
};

#[derive(Debug, Clone)]
pub struct MultilineCaret {
    pub line_index: usize,
    pub caret_index_in_line: usize,
    line_texts: LineTexts,
}
pub(crate) fn get_multiline_caret(selection_index: usize, line_texts: LineTexts) -> MultilineCaret {
    let mut line_index = 0;
    let mut left_index = selection_index;

    for line in line_texts.iter_lines() {
        if left_index < line.chars.len() {
            return MultilineCaret {
                line_index,
                caret_index_in_line: left_index,
                line_texts: line_texts.clone(),
            };
        } else if left_index == line.chars.len() {
            match line.new_line_by {
                Some(new_line_by) => match new_line_by {
                    NewLineBy::Wrap => {
                        return MultilineCaret {
                            line_index: line_index + 1,
                            caret_index_in_line: 0,
                            line_texts: line_texts.clone(),
                        };
                    }
                    NewLineBy::LineFeed => {
                        return MultilineCaret {
                            line_index,
                            caret_index_in_line: left_index,
                            line_texts: line_texts.clone(),
                        };
                    }
                },
                None => {
                    return MultilineCaret {
                        line_index,
                        caret_index_in_line: left_index,
                        line_texts: line_texts.clone(),
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
    MultilineCaret {
        line_index,
        caret_index_in_line: left_index,
        line_texts,
    }
}

impl MultilineCaret {
    pub(crate) fn get_caret_on_key(&self, key: KeyInInterest) -> MultilineCaret {
        let (line_index, x) = match key {
            KeyInInterest::ArrowUpDown(up_down) => match up_down {
                ArrowUpDown::Up => {
                    if self.line_index == 0 {
                        return MultilineCaret {
                            line_index: 0,
                            caret_index_in_line: 0,
                            line_texts: self.line_texts.clone(),
                        };
                    }
                    (self.line_index - 1, self.get_x())
                }
                ArrowUpDown::Down => {
                    if self.is_at_bottom() {
                        return MultilineCaret {
                            line_index: self.line_index,
                            caret_index_in_line: self
                                .line_texts
                                .iter_chars()
                                .nth(self.line_index)
                                .unwrap()
                                .len(),
                            line_texts: self.line_texts.clone(),
                        };
                    }
                    (self.line_index + 1, self.get_x())
                }
            },
            KeyInInterest::HomeEnd(home_end) => match home_end {
                system::text_input::HomeEnd::Home => (self.line_index, 0.px()),
                system::text_input::HomeEnd::End => (self.line_index, f32::MAX.px()),
            },
        };

        let caret_index_on_direction = self.get_caret_index_on_x(x, line_index);

        MultilineCaret {
            line_index,
            caret_index_in_line: caret_index_on_direction,
            line_texts: self.line_texts.clone(),
        }
    }

    pub(crate) fn to_selection_index(&self) -> usize {
        let index_before_line = self.line_texts.char_index_before_line(self.line_index);
        index_before_line + self.caret_index_in_line
    }

    fn get_x(&self) -> Px {
        let line_text = self.line_texts.iter_str().nth(self.line_index).unwrap();

        let glyph_groups = get_glyph_groups(
            &line_text,
            &self.line_texts.fonts,
            self.line_texts.paint.clone(),
        );

        let mut x = 0.px();
        for glyph_group in glyph_groups {
            if self.caret_index_in_line <= glyph_group.end_char_index {
                let start_index = glyph_group.start_index();

                let glyph_widths_left = glyph_group
                    .widths
                    .into_iter()
                    .take(self.caret_index_in_line - start_index);
                return x + glyph_widths_left.sum::<Px>();
            } else {
                x += glyph_group.width;
            }
        }
        x
    }

    fn get_caret_index_on_x(&self, x: Px, line_index: usize) -> usize {
        let line_text = self.line_texts.iter_str().nth(line_index).unwrap();

        let glyph_groups = get_glyph_groups(
            &line_text,
            &self.line_texts.fonts,
            self.line_texts.paint.clone(),
        );

        let mut caret_positions = vec![(0.px(), 0)];
        for glyph_group in glyph_groups {
            let glyph_ids = glyph_group.glyph_ids.into_boxed_slice();
            let glyph_widths = glyph_group
                .font
                .get_glyph_widths(glyph_ids, self.line_texts.paint.as_ref());

            for glyph_width in glyph_widths.into_iter() {
                let last = caret_positions.last().unwrap().clone();
                caret_positions.push((last.0 + glyph_width, last.1 + 1));
            }
        }

        if x == f32::MAX.px() {
            return caret_positions.last().unwrap().1;
        }

        let mut closest_caret_index = 0;
        let mut closest_caret_distance = x;

        for (caret_x, caret_index) in caret_positions {
            let distance = (caret_x - x).abs();
            if distance < closest_caret_distance {
                closest_caret_index = caret_index;
                closest_caret_distance = distance;
            }
        }
        closest_caret_index
    }

    pub(crate) fn is_at_bottom(&self) -> bool {
        let line_count = self.line_texts.iter_str().count();
        if line_count == 0 {
            return true;
        }
        self.line_index == line_count - 1
    }
}
