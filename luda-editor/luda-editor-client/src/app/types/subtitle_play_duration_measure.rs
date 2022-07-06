use super::meta::Meta;
use crate::app::types::{Subtitle, Time};
use linked_hash_map::LinkedHashMap;
use namui::prelude::*;

const IGNORED_CHARACTERS: &[char] = &[' ', '?', '.', ','];
pub trait SubtitlePlayDurationMeasure {
    fn get_minimum_play_duration(&self, language: &Language) -> Time;
    fn get_play_duration_per_character(&self, language: &Language) -> Time;
    fn get_specific_text_token_play_duration_map(&self) -> LinkedHashMap<String, Time>;
    fn get_play_duration(&self, subtitle: &Subtitle, language: &Language) -> Time {
        let minimum_play_duration = self.get_minimum_play_duration(language);
        let play_duration_per_character = self.get_play_duration_per_character(language);
        let specific_text_token_play_duration_map =
            self.get_specific_text_token_play_duration_map();
        let text = subtitle.language_text_map.get(language).unwrap();

        let mut play_duration: Time = Time::Ms(0.0);
        let text_without_token: String = specific_text_token_play_duration_map.iter().fold(
            text.clone(),
            |text_without_token, (token, duration)| {
                play_duration += count_token_in_text(&text_without_token, token) * duration;
                remove_token_from_text(&text_without_token, token)
            },
        );

        play_duration += text_without_token
            .chars()
            .filter(|char| !IGNORED_CHARACTERS.contains(char))
            .count()
            * play_duration_per_character;
        if play_duration < minimum_play_duration {
            minimum_play_duration
        } else {
            play_duration
        }
    }
}

fn count_token_in_text(text: &String, token: &String) -> usize {
    text.matches(token).count()
}

fn remove_token_from_text(text: &String, token: &String) -> String {
    text.replace(token, "")
}

impl SubtitlePlayDurationMeasure for Meta {
    fn get_minimum_play_duration(&self, language: &Language) -> Time {
        self.subtitle_language_minimum_play_duration_map
            .get(language)
            .unwrap()
            .clone()
    }

    fn get_play_duration_per_character(&self, language: &Language) -> Time {
        self.subtitle_language_play_duration_per_character_map
            .get(language)
            .unwrap()
            .clone()
    }

    fn get_specific_text_token_play_duration_map(&self) -> LinkedHashMap<String, Time> {
        self.subtitle_specific_text_token_play_duration_map.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::app::types::{
        subtitle_play_duration_measure::{count_token_in_text, remove_token_from_text},
        Meta, Subtitle, SubtitlePlayDurationMeasure, Time,
    };
    use std::collections::HashMap;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn count_no_token() {
        let text = format!("Now testing");
        let token = format!("..");
        assert_eq!(count_token_in_text(&text, &token), 0);
    }

    #[test]
    #[wasm_bindgen_test]
    fn count_consecutive_tokens() {
        let text = format!("Now.... testing");
        let token = format!("..");
        assert_eq!(count_token_in_text(&text, &token), 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn count_end_token() {
        let text = format!("Now.. testing..");
        let token = format!("..");
        assert_eq!(count_token_in_text(&text, &token), 2);
    }

    #[test]
    #[wasm_bindgen_test]
    fn remove_no_token() {
        let text = format!("Now testing");
        let token = format!("..");
        let expected = format!("Now testing");
        assert_eq!(remove_token_from_text(&text, &token), expected);
    }

    #[test]
    #[wasm_bindgen_test]
    fn remove_consecutive_tokens() {
        let text = format!("Now.... testing");
        let token = format!("..");
        let expected = format!("Now testing");
        assert_eq!(remove_token_from_text(&text, &token), expected);
    }

    #[test]
    #[wasm_bindgen_test]
    fn remove_end_token() {
        let text = format!("Now testing");
        let token = format!("..");
        let expected = format!("Now testing");
        assert_eq!(remove_token_from_text(&text, &token), expected);
    }

    #[test]
    #[wasm_bindgen_test]
    fn two_dot_first() {
        let meta: Meta = serde_json::from_str(
            "{
            \"subtitle_language_minimum_play_duration_map\": {
                \"Ko\": {
                    \"Ms\": 1000.0
                }
            },
            \"subtitle_language_play_duration_per_character_map\": {
                \"Ko\": {
                    \"Ms\": 1000.0
                }
            },
            \"subtitle_specific_text_token_play_duration_map\": {
                \"..\": {
                    \"Ms\": 1.0
                },
                \"...\": {
                    \"Ms\": 10.0
                }
            },
            \"subtitle_character_color_map\": {}
        }",
        )
        .unwrap();
        let subtitle: Subtitle = mock_subtitle("Now... testing.....");
        // 1. two dot: 3 * 1.0 = 3.0
        // Now. testing.

        // 2. three dot: 0 * 10.0 = 0.0
        // Now. testing.

        // 3. remaining without IGNORED_CHARACTERS: 10 * 1000.0
        // Nowtesting

        // total: 10003.0
        let expected: Time = Time::Ms(10003.0);

        assert_eq!(
            meta.get_play_duration(&subtitle, &namui::Language::Ko),
            expected
        );
    }

    #[test]
    #[wasm_bindgen_test]
    fn three_dot_first() {
        let meta: Meta = serde_json::from_str(
            "{
            \"subtitle_language_minimum_play_duration_map\": {
                \"Ko\": {
                    \"Ms\": 1000.0
                }
            },
            \"subtitle_language_play_duration_per_character_map\": {
                \"Ko\": {
                    \"Ms\": 1000.0
                }
            },
            \"subtitle_specific_text_token_play_duration_map\": {
                \"...\": {
                    \"Ms\": 1.0
                },
                \"..\": {
                    \"Ms\": 10.0
                }
            },
            \"subtitle_character_color_map\": {}
        }",
        )
        .unwrap();
        let subtitle: Subtitle = mock_subtitle("Now... testing.....");
        // 1. three dot: 2 * 1.0 = 2.0
        // Now testing..

        // 2. two dot: 1 * 10.0 = 10.0
        // Now testing

        // 3. remaining without IGNORED_CHARACTERS: 10 * 1000.0
        // Nowtesting

        // total: 10012.0
        let expected: Time = Time::Ms(10012.0);

        assert_eq!(
            meta.get_play_duration(&subtitle, &namui::Language::Ko),
            expected
        );
    }

    fn mock_subtitle(content: &str) -> Subtitle {
        let mut language_text_map = HashMap::new();
        language_text_map.insert(namui::Language::Ko, format!("{}", content));
        Subtitle {
            id: format!("id"),
            speaker: format!("speaker"),
            language_text_map,
        }
    }
}
