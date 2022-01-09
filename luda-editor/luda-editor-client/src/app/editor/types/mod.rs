use crate::app::types::{Subtitle, Time};
use namui::prelude::*;
use std::{array::IntoIter, collections::HashMap};

pub struct SubtitlePlayDurationMeasurer {
    minimum_play_durations: HashMap<Language, Time>,
    play_duration_per_character: HashMap<Language, Time>,
}
impl SubtitlePlayDurationMeasurer {
    pub fn get_play_duration(&self, subtitle: &Subtitle, language: &Language) -> Time {
        let minimum_play_duration = self.minimum_play_durations.get(language).unwrap();
        let play_duration_per_character = self.play_duration_per_character.get(language).unwrap();
        let play_duration = Time::from_ms(
            (subtitle.language_text_map.get(language).unwrap().len() as f64
                * play_duration_per_character.milliseconds as f64)
                .ceil() as i64,
        );
        if play_duration < *minimum_play_duration {
            *minimum_play_duration
        } else {
            play_duration
        }
    }

    pub(crate) fn new() -> SubtitlePlayDurationMeasurer {
        SubtitlePlayDurationMeasurer {
            // TODO: Check minimum play duration
            minimum_play_durations: HashMap::<_, _>::from_iter(IntoIter::new([(
                Language::Ko,
                Time::from_ms(1000),
            )])),
            play_duration_per_character: HashMap::<_, _>::from_iter(IntoIter::new([(
                Language::Ko,
                Time::from_ms(100),
            )])),
        }
    }
}
