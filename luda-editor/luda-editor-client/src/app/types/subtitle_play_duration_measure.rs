use super::meta::Meta;
use crate::app::types::{Subtitle, Time};
use namui::prelude::*;

const IGNORED_CHARACTERS: &[char] = &[' ', '?', '.', ','];
pub trait SubtitlePlayDurationMeasure {
    fn get_minimum_play_duration(&self, language: &Language) -> Time;
    fn get_play_duration_per_character(&self, language: &Language) -> Time;
    fn get_play_duration(&self, subtitle: &Subtitle, language: &Language) -> Time {
        let minimum_play_duration = self.get_minimum_play_duration(language);
        let play_duration_per_character = self.get_play_duration_per_character(language);
        let text = subtitle.language_text_map.get(language).unwrap();
        let play_duration = text
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
}
