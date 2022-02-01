use super::{get_camera_track_id, JobExecute};
use crate::app::types::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct AddCameraClipJob {
    pub camera_clip: Arc<CameraClip>,
    pub time_to_insert: Time,
}

impl JobExecute for AddCameraClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let camera_track_id = get_camera_track_id(&sequence);
        match sequence.replace_track(&camera_track_id, |track: &CameraTrack| {
            let mut track = track.clone();
            track.insert_clip(self.camera_clip.clone(), self.time_to_insert);
            Ok(track)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Camera track not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use namui::prelude::*;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_for_empty_track_should_works() {
        let sequence = mock_sequence(&[]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("0", Time::from_ms(0.0), Time::from_ms(1.0)),
            time_to_insert: Time::from_ms(1.0),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_between_clips_should_works() {
        // before : 0 1 2 3
        // job : insert 4 between 1 and 2
        // result : 0 1 4 2 3

        let sequence = mock_sequence(&["0", "1", "2", "3"]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("4", Time::from_ms(0.0), Time::from_ms(1.0)),
            time_to_insert: Time::from_ms(1.75),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "4", "2", "3"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_front_should_works() {
        // before : 0 1 2 3
        // job : insert 4 at front
        // result : 4 0 1 2 3

        let sequence = mock_sequence(&["0", "1", "2", "3"]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("4", Time::from_ms(0.0), Time::from_ms(1.0)),
            time_to_insert: Time::from_ms(0.0),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["4", "0", "1", "2", "3"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    #[test]
    #[wasm_bindgen_test]
    fn insert_camera_clip_back_should_works() {
        // before : 0 1 2 3
        // job : insert 4 at back
        // result : 0 1 2 3 4

        let sequence = mock_sequence(&["0", "1", "2", "3"]);
        let job = AddCameraClipJob {
            camera_clip: mock_camera_clip("4", Time::from_ms(0.0), Time::from_ms(1.0)),
            time_to_insert: Time::from_ms(4.0),
        };

        let result = job.execute(&sequence).unwrap();
        let expected_clip_ids = ["0", "1", "2", "3", "4"];
        let result_clip_ids = extract_camera_clip_ids(&result);
        assert_eq!(result_clip_ids, expected_clip_ids);
    }

    fn extract_camera_clip_ids(sequence: &Sequence) -> Vec<String> {
        sequence
            .tracks
            .iter()
            .filter_map::<Vec<String>, _>(|track| {
                if let Track::Camera(camera_track) = track.as_ref() {
                    Some(
                        camera_track
                            .clips
                            .iter()
                            .map(|clip| clip.id.clone())
                            .collect(),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    fn mock_sequence(camera_clip_ids: &[&str]) -> Sequence {
        let mut clips = Vec::new();
        camera_clip_ids.iter().enumerate().for_each(|(index, id)| {
            let start_at = Time::from_ms(index as f32);
            let end_at = Time::from_ms((index + 1) as f32);
            clips.push(mock_camera_clip(id, start_at, end_at));
        });
        Sequence {
            tracks: vec![Arc::new(Track::Camera(CameraTrack {
                id: "track-1".to_string(),
                clips: clips.into(),
            }))]
            .into(),
        }
    }

    fn mock_camera_clip(clip_id: &str, start_at: Time, end_at: Time) -> Arc<CameraClip> {
        Arc::new(CameraClip {
            id: clip_id.to_string(),
            start_at,
            end_at,
            camera_angle: CameraAngle {
                character_pose_emotion: CharacterPoseEmotion(
                    "c".to_string(),
                    "p".to_string(),
                    "e".to_string(),
                ),
                source_01_circumscribed: Circumscribed {
                    center: Xy { x: 0.0, y: 0.0 },
                    radius: 1.0,
                },
                crop_screen_01_rect: LtrbRect {
                    left: 0.0,
                    top: 0.0,
                    right: 1.0,
                    bottom: 1.0,
                },
            },
        })
    }
}
