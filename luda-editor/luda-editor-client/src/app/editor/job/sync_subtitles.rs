use super::JobExecute;
use crate::app::types::*;

#[derive(Debug, Clone)]
pub struct SyncSubtitlesJob {
    pub subtitles: Vec<Subtitle>,
}

impl JobExecute for SyncSubtitlesJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let subtitle_track_id = get_subtitle_track_id(&sequence);
        match sequence.replace_track(&subtitle_track_id, |mut track: SubtitleTrack| {
            track.sync(&self.subtitles);
            Ok(track)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Subtitle track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

pub(crate) fn get_subtitle_track_id(sequence: &Sequence) -> String {
    sequence
        .tracks
        .iter()
        .find_map(|track| match track.as_ref() {
            Track::Subtitle(track) => Some(track.id.clone()),
            _ => None,
        })
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use namui::Language;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn change_subtitle_text() {
        let sequence = mock_sequence(&[], &["0"]);

        let mut subtitle = mock_subtitle("0");
        let changed_subtitle_text = "changed subtitle text";
        subtitle
            .language_text_map
            .insert(Language::Ko, changed_subtitle_text.to_string());

        let job = SyncSubtitlesJob {
            subtitles: vec![subtitle],
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0"];

        let clips = extract_subtitle_clips(&result);
        let clip_ids = clips.iter().map(|clip| clip.id.clone()).collect::<Vec<_>>();
        assert_eq!(clip_ids, expected_clip_ids);
        assert_eq!(
            clips[0]
                .subtitle
                .language_text_map
                .get(&Language::Ko)
                .unwrap(),
            changed_subtitle_text
        );
        assert_eq!(clips[0].is_needed_to_update_position, false);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_new_subtitle_clip_in_empty_subtitle_track() {
        let sequence = mock_sequence(&[], &[]);
        let job = SyncSubtitlesJob {
            subtitles: vec![mock_subtitle("0")],
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0"];

        let clips = extract_subtitle_clips(&result);
        let clip_ids = clips.iter().map(|clip| clip.id.clone()).collect::<Vec<_>>();
        assert_eq!(clip_ids, expected_clip_ids);

        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[0].is_needed_to_update_position, true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_new_subtitle_clips_in_empty_subtitle_track() {
        let sequence = mock_sequence(&[], &[]);
        let job = SyncSubtitlesJob {
            subtitles: vec![mock_subtitle("0"), mock_subtitle("1")],
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1"];

        let clips = extract_subtitle_clips(&result);
        let clip_ids = clips.iter().map(|clip| clip.id.clone()).collect::<Vec<_>>();
        assert_eq!(clip_ids, expected_clip_ids);

        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(
            clips[1].start_at,
            Time::from_ms(DEFAULT_SUBTITLE_INSERT_INTERVAL_MS)
        );
        assert_eq!(clips[0].is_needed_to_update_position, true);
        assert_eq!(clips[1].is_needed_to_update_position, true);
    }
    #[test]
    #[wasm_bindgen_test]
    fn insert_new_subtitle_clips_in_front_of_subtitle_track() {
        let sequence = mock_sequence(&[], &["0", "1"]);
        let job = SyncSubtitlesJob {
            subtitles: vec![
                mock_subtitle("2"),
                mock_subtitle("3"),
                mock_subtitle("0"),
                mock_subtitle("1"),
            ],
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["2", "3", "0", "1"];

        let clips = extract_subtitle_clips(&result);
        let clip_ids = clips.iter().map(|clip| clip.id.clone()).collect::<Vec<_>>();
        assert_eq!(clip_ids, expected_clip_ids);

        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[1].start_at, Time::from_ms(0.0));
        assert_eq!(clips[2].start_at, Time::from_ms(0.0));
        assert_eq!(clips[3].start_at, Time::from_ms(1.0));

        assert_eq!(clips[0].is_needed_to_update_position, true);
        assert_eq!(clips[1].is_needed_to_update_position, true);
        assert_eq!(clips[2].is_needed_to_update_position, false);
        assert_eq!(clips[3].is_needed_to_update_position, false);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_new_subtitle_clips_in_back_of_subtitle_track() {
        let sequence = mock_sequence(&[], &["0", "1"]);
        let job = SyncSubtitlesJob {
            subtitles: vec![
                mock_subtitle("0"),
                mock_subtitle("1"),
                mock_subtitle("2"),
                mock_subtitle("3"),
            ],
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "2", "3"];

        let clips = extract_subtitle_clips(&result);
        let clip_ids = clips.iter().map(|clip| clip.id.clone()).collect::<Vec<_>>();
        assert_eq!(clip_ids, expected_clip_ids);

        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[1].start_at, Time::from_ms(1.0));
        assert_eq!(
            clips[2].start_at,
            Time::from_ms(1.0 + DEFAULT_SUBTITLE_INSERT_INTERVAL_MS)
        );
        assert_eq!(
            clips[3].start_at,
            Time::from_ms(1.0 + DEFAULT_SUBTITLE_INSERT_INTERVAL_MS * 2.0)
        );

        assert_eq!(clips[0].is_needed_to_update_position, false);
        assert_eq!(clips[1].is_needed_to_update_position, false);
        assert_eq!(clips[2].is_needed_to_update_position, true);
        assert_eq!(clips[3].is_needed_to_update_position, true);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_new_subtitle_clips_in_middle_of_subtitle_track() {
        let sequence = mock_sequence(&[], &["0", "1"]);
        let job = SyncSubtitlesJob {
            subtitles: vec![
                mock_subtitle("0"),
                mock_subtitle("2"),
                mock_subtitle("3"),
                mock_subtitle("1"),
            ],
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "2", "3", "1"];

        let clips = extract_subtitle_clips(&result);
        let clip_ids = clips.iter().map(|clip| clip.id.clone()).collect::<Vec<_>>();
        assert_eq!(clip_ids, expected_clip_ids);

        assert_eq!(clips[0].start_at, Time::from_ms(0.0));
        assert_eq!(clips[1].start_at, Time::from_ms(1.0 / 3.0));
        assert_eq!(clips[2].start_at, Time::from_ms(2.0 / 3.0));
        assert_eq!(clips[3].start_at, Time::from_ms(1.0));

        assert_eq!(clips[0].is_needed_to_update_position, false);
        assert_eq!(clips[1].is_needed_to_update_position, true);
        assert_eq!(clips[2].is_needed_to_update_position, true);
        assert_eq!(clips[3].is_needed_to_update_position, false);
    }

    #[test]
    #[wasm_bindgen_test]
    fn sync_twice_should_keep_value_of_boolean_is_needed_to_update_position() {
        let mut sequence = mock_sequence(&[], &["0"]);

        for _ in 0..2 {
            let job = SyncSubtitlesJob {
                subtitles: vec![mock_subtitle("0"), mock_subtitle("1")],
            };

            sequence = job.execute(&sequence).unwrap();
            let clips = extract_subtitle_clips(&sequence);
            assert_eq!(clips[0].is_needed_to_update_position, false);
            assert_eq!(clips[1].is_needed_to_update_position, true);
        }
    }
}
