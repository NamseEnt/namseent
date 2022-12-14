use super::{
    multiline_caret::{get_multiline_caret, MultilineCaret},
    *,
};
use crate::*;
use std::sync::Arc;
use textwrap::{core::Fragment, word_splitters::split_words, wrap_algorithms::wrap_first_fit, *};
use unicode_segmentation::UnicodeSegmentation;

#[derive(Clone, Copy, Debug)]
pub(crate) enum NewLineBy {
    Wrap,
    /// `\n`
    LineFeed,
}

#[derive(Clone, Debug)]
pub(crate) struct Line {
    /// Should not have `\n`
    pub chars: Vec<char>,
    pub new_line_by: Option<NewLineBy>,
}

#[derive(Debug)]
pub(crate) struct LineTexts<'a> {
    vec: Vec<Line>,
    pub fonts: &'a Vec<Arc<Font>>,
    pub paint: Option<&'a Paint>,
}
impl<'a> LineTexts<'a> {
    pub fn new(
        text: &str,
        fonts: &'a Vec<Arc<Font>>,
        paint: Option<&'a Paint>,
        max_width: Option<Px>,
    ) -> LineTexts<'a> {
        let vec = if let Some(max_width) = max_width {
            let word_separator = WordSeparator::UnicodeBreakProperties;
            let word_splitter = WordSplitter::NoHyphenation;

            let mut vec: Vec<Line> = vec![];

            for (index, line) in text.split("\n").enumerate() {
                if index > 0 {
                    vec.last_mut().unwrap().new_line_by = Some(NewLineBy::LineFeed);
                }
                let words = word_separator.find_words(line);
                let split_words = split_words(words, &word_splitter);

                let namui_words = split_words
                    .flat_map(|word| {
                        NamuiWord::from_word(word, fonts, paint)
                            .break_apart(max_width, fonts, paint)
                    })
                    .collect::<Vec<_>>();

                let line_lengths = [max_width.as_f32() as f64];
                let wrapped_words = wrap_first_fit(&namui_words, &line_lengths);

                for (index, words_in_line) in wrapped_words.iter().enumerate() {
                    if index > 0 {
                        vec.last_mut().unwrap().new_line_by = Some(NewLineBy::Wrap);
                    }

                    let mut line_text = "".to_string();
                    for word in words_in_line.iter() {
                        line_text += word.word;
                        line_text += word.whitespace;
                        // NOTE: I didn't use penalty because it won't work well in right-align case.
                    }
                    vec.push(Line {
                        chars: line_text.chars().collect(),
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
        Self { vec, fonts, paint }
    }
    pub fn line_len(&self) -> usize {
        self.vec.len()
    }
    pub fn iter_str(&'a self) -> impl Iterator<Item = String> + 'a {
        self.vec.iter().map(|line| line.chars.iter().collect())
    }
    pub fn iter_chars(&'a self) -> impl Iterator<Item = &'a Vec<char>> + 'a {
        self.vec.iter().map(|line| &line.chars)
    }
    pub(crate) fn iter_lines(&self) -> impl Iterator<Item = &Line> {
        self.vec.iter()
    }
    pub fn get_line(&self, line_index: usize) -> Option<&Line> {
        self.vec.get(line_index)
    }
    pub(crate) fn char_index_before_line(&self, line_index: usize) -> usize {
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

    pub(crate) fn get_multiline_caret(&'a self, selection_index: usize) -> MultilineCaret<'a> {
        get_multiline_caret(selection_index, self)
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

impl<'a> Fragment for NamuiWord<'a> {
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
    fn from_word(word: core::Word<'a>, fonts: &Vec<Arc<Font>>, paint: Option<&Paint>) -> Self {
        Self {
            word: word.word,
            width: get_text_width_with_fonts(fonts, word.word, paint),
            whitespace: word.whitespace,
            whitespace_width: get_text_width_with_fonts(fonts, word.whitespace, paint),
            penalty: word.penalty,
            penalty_width: get_text_width_with_fonts(fonts, word.penalty, paint),
        }
    }

    fn break_apart(
        self,
        max_width: Px,
        fonts: &Vec<Arc<Font>>,
        paint: Option<&Paint>,
    ) -> Vec<NamuiWord<'a>> {
        if self.width <= max_width {
            return vec![self];
        }

        let mut start = 0;
        let mut words = Vec::new();
        for (idx, grapheme) in self.word.grapheme_indices(true) {
            let with_grapheme = &self.word[start..idx + grapheme.len()];
            let without_grapheme = &self.word[start..idx];
            if idx > 0 && get_text_width_with_fonts(fonts, with_grapheme, paint) > max_width {
                let natural_width = get_text_width_with_fonts(fonts, without_grapheme, paint);
                words.push(NamuiWord {
                    word: &without_grapheme,
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
            width: get_text_width_with_fonts(fonts, &self.word[start..], paint),
            whitespace: self.whitespace,
            whitespace_width: self.whitespace_width,
            penalty: self.penalty,
            penalty_width: self.penalty_width,
        });

        words
    }
}
