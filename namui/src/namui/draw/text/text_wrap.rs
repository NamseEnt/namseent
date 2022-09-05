use super::*;
use textwrap::{core::Fragment, word_splitters::split_words, wrap_algorithms::wrap_first_fit, *};
use unicode_segmentation::UnicodeSegmentation;

impl TextDrawCommand {
    pub(crate) fn get_line_texts<'a>(
        &self,
        fonts: &Vec<Arc<Font>>,
        paint: &Arc<Paint>,
    ) -> Vec<String> {
        if let Some(max_width) = self.max_width {
            let word_separator = WordSeparator::UnicodeBreakProperties;
            let word_splitter = WordSplitter::NoHyphenation;

            let mut line_texts = vec![];

            for line in self.text.split("\n") {
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

                for words_in_line in wrapped_words {
                    let mut line_text = "".to_string();
                    for (i, word) in words_in_line.iter().enumerate() {
                        line_text += word.word;

                        let last_word = i == words_in_line.len() - 1;
                        line_text += if last_word {
                            word.penalty
                        } else {
                            word.whitespace
                        };
                    }

                    line_texts.push(line_text);
                }
            }
            line_texts
        } else {
            self.text.lines().map(|s| s.to_string()).collect()
        }
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
    fn from_word(word: core::Word<'a>, fonts: &Vec<Arc<Font>>, paint: &Arc<Paint>) -> Self {
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
        paint: &Arc<Paint>,
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
