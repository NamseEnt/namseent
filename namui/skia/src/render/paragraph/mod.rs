mod caret;
mod glyph;

use super::*;
use crate::*;
pub use caret::*;
pub use glyph::*;
use std::sync::Arc;
use textwrap::{core::Fragment, word_splitters::split_words, wrap_algorithms::wrap_first_fit, *};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Debug, Clone)]
pub struct Paragraph {
    pub vec: Vec<Line>,
    pub group_glyph: Arc<dyn GroupGlyph>,
}
impl Paragraph {
    pub fn new(text: &str, group_glyph: Arc<dyn GroupGlyph>, max_width: Option<Px>) -> Paragraph {
        let vec = if let Some(max_width) = max_width {
            let word_separator = WordSeparator::UnicodeBreakProperties;
            let word_splitter = WordSplitter::NoHyphenation;

            let mut vec: Vec<Line> = vec![];

            for (index, line) in text.split('\n').enumerate() {
                if index > 0 {
                    vec.last_mut().unwrap().new_line_by = Some(NewLineBy::LineFeed);
                }
                let words = word_separator.find_words(line);
                let split_words = split_words(words, &word_splitter);

                let namui_words = split_words
                    .flat_map(|word| {
                        NamuiWord::from_word(word, group_glyph.as_ref())
                            .break_apart(max_width, group_glyph.as_ref())
                    })
                    .collect::<Vec<_>>();

                let line_lengths = [max_width.as_f32() as f64];
                let wrapped_words = wrap_first_fit(&namui_words, &line_lengths);

                for (index, words_in_line) in wrapped_words.iter().enumerate() {
                    if index > 0 {
                        vec.last_mut().unwrap().new_line_by = Some(NewLineBy::Wrap);
                    }

                    let mut line = "".to_string();
                    for word in words_in_line.iter() {
                        line += word.word;
                        line += word.whitespace;
                        // NOTE: I didn't use penalty because it won't work well in right-align case.
                    }
                    vec.push(Line {
                        chars: line.chars().collect(),
                        new_line_by: None,
                    });
                }
            }
            vec
        } else {
            let lines = text.lines();

            let mut vec: Vec<Line> = vec![];

            for (index, line) in lines.enumerate() {
                if index > 0 {
                    vec.last_mut().unwrap().new_line_by = Some(NewLineBy::LineFeed);
                }
                vec.push(Line {
                    chars: line.chars().collect(),
                    new_line_by: None,
                });
            }
            vec
        };
        Self { vec, group_glyph }
    }
    pub fn line_len(&self) -> usize {
        self.vec.len()
    }
    pub fn iter_str(&self) -> impl Iterator<Item = String> + '_ {
        self.vec.iter().map(|line| line.chars.iter().collect())
    }
    pub fn iter_chars(&self) -> impl Iterator<Item = &Vec<char>> {
        self.vec.iter().map(|line| &line.chars)
    }
    pub fn iter_lines(&self) -> impl Iterator<Item = &Line> {
        self.vec.iter()
    }
    pub fn get_line(&self, line_index: usize) -> Option<&Line> {
        self.vec.get(line_index)
    }
    pub fn char_index_before_line(&self, line_index: usize) -> usize {
        self.vec
            .iter()
            .take(line_index)
            .map(|line| {
                line.chars.len()
                    + match line.new_line_by {
                        Some(new_line_by) => match new_line_by {
                            NewLineBy::Wrap => 0,
                            NewLineBy::LineFeed => 1,
                        },
                        None => 0,
                    }
            })
            .sum()
    }

    pub fn caret(&self, caret_index: usize) -> Caret<'_> {
        get_caret(caret_index, self)
    }

    pub fn selection_index_of_xy(
        &self,
        xy: Xy<Px>,
        font_size: IntPx,
        line_height_percent: Percent,
        text_baseline: TextBaseline,
        text_align: TextAlign,
    ) -> usize {
        let line_len = self.line_len();
        if line_len == 0 {
            return 0;
        }

        let line_index = {
            let line_height = font_size.into_px() * line_height_percent;

            let top_y = xy.y
                + line_height
                    * match text_baseline {
                        TextBaseline::Top => 0.0,
                        TextBaseline::Middle => line_len as f32 / 2.0,
                        TextBaseline::Bottom => line_len as f32,
                    };

            let line_index = if top_y <= 0.px() {
                0
            } else {
                (top_y / line_height).floor() as usize
            };

            let line_max_index = line_len - 1;
            line_index.min(line_max_index)
        };

        let str_index_before_line = self.char_index_before_line(line_index);

        let line = self.iter_str().nth(line_index).unwrap();

        let glyph_widths = self.group_glyph.widths(line.as_ref());

        let line_width = glyph_widths.iter().sum::<Px>();

        let aligned_x = match text_align {
            TextAlign::Left => xy.x,
            TextAlign::Center => xy.x + line_width / 2.0,
            TextAlign::Right => xy.x + line_width,
        };

        let mut left = px(0.0);
        let index = glyph_widths
            .iter()
            .position(|width| {
                let center = left + width / 2.0;
                if aligned_x < center {
                    return true;
                }
                left += *width;
                false
            })
            .unwrap_or(line.chars().count());

        str_index_before_line + index
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NamuiWord<'a> {
    word: &'a str,
    width: Px,
    whitespace: &'a str,
    whitespace_width: Px,
    penalty: &'a str,
    penalty_width: Px,
}

impl Fragment for NamuiWord<'_> {
    fn width(&self) -> f64 {
        self.width.as_f32() as f64
    }

    fn whitespace_width(&self) -> f64 {
        self.whitespace_width.as_f32() as f64
    }

    fn penalty_width(&self) -> f64 {
        self.penalty_width.as_f32() as f64
    }
}

impl<'a> NamuiWord<'a> {
    fn from_word(word: core::Word<'a>, group_glyph: impl GroupGlyph) -> Self {
        Self {
            word: word.word,
            width: group_glyph.width(word.word),
            whitespace: word.whitespace,
            whitespace_width: group_glyph.width(word.whitespace),
            penalty: word.penalty,
            penalty_width: group_glyph.width(word.penalty),
        }
    }

    fn break_apart(self, max_width: Px, group_glyph: impl GroupGlyph) -> Vec<NamuiWord<'a>> {
        if self.width <= max_width {
            return vec![self];
        }

        let mut start = 0;
        let mut words = Vec::new();
        for (idx, grapheme) in self.word.grapheme_indices(true) {
            let with_grapheme = &self.word[start..idx + grapheme.len()];
            let without_grapheme = &self.word[start..idx];
            if idx > 0 && group_glyph.width(with_grapheme) > max_width {
                let natural_width = group_glyph.width(without_grapheme);
                words.push(NamuiWord {
                    word: without_grapheme,
                    width: max_width.max(natural_width),
                    whitespace: "",
                    whitespace_width: 0.px(),
                    penalty: "",
                    penalty_width: 0.px(),
                });
                start = idx;
            }
        }

        words.push(NamuiWord {
            word: &self.word[start..],
            width: group_glyph.width(&self.word[start..]),
            whitespace: self.whitespace,
            whitespace_width: self.whitespace_width,
            penalty: self.penalty,
            penalty_width: self.penalty_width,
        });

        words
    }
}

pub fn get_left_in_align(x: Px, align: TextAlign, width: Px) -> Px {
    match align {
        TextAlign::Left => x,
        TextAlign::Center => x - width / 2.0,
        TextAlign::Right => x - width,
    }
}
pub fn get_bottom_of_baseline(baseline: TextBaseline, font_metrics: FontMetrics) -> Px {
    match baseline {
        TextBaseline::Top => -font_metrics.ascent - font_metrics.descent,
        TextBaseline::Middle => (-font_metrics.ascent - font_metrics.descent) / 2.0,
        TextBaseline::Bottom => -font_metrics.descent,
    }
}
pub fn get_multiline_y_baseline_offset(
    baseline: TextBaseline,
    line_height: Px,
    paragraph_len: usize,
) -> Px {
    match baseline {
        TextBaseline::Top => 0.px(),
        TextBaseline::Middle => -line_height * 0.5 * (paragraph_len - 1),
        TextBaseline::Bottom => -line_height * (paragraph_len - 1),
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum NewLineBy {
    Wrap,
    /// `\n`
    LineFeed,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    /// Should not have `\n`
    pub chars: Vec<char>,
    pub new_line_by: Option<NewLineBy>,
}
