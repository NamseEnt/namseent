use super::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTrack {
    pub id: String,
    pub clips: Arc<[Arc<CameraClip>]>,
}
impl CameraTrack {
    pub(crate) fn get_clip_at_time(&self, time: &Time) -> Option<&CameraClip> {
        self.clips.iter().find_map(|clip| {
            if clip.is_at_time(time) {
                Some(clip.as_ref())
            } else {
                None
            }
        })
    }

    pub(crate) fn insert_clip(&mut self, clip: Arc<CameraClip>, at: Time) {
        let mut clips = self.clips.to_vec();

        let clip_index = clips
            .iter()
            .position(|c| {
                if at < c.start_at {
                    return true;
                }
                if c.end_at < at {
                    return false;
                }

                let middle = (c.start_at + c.end_at) / 2.0;
                return at < middle;
            })
            .unwrap_or(clips.len());

        clips.insert(clip_index, clip);

        push_front_camera_clips(&mut clips);

        self.clips = clips.into();
    }
    pub(crate) fn move_clip_delta(&mut self, clip_id: &str, delta: Time) {
        let moved_at = self
            .clips
            .iter()
            .find(|clip| clip.id.eq(clip_id))
            .unwrap()
            .start_at
            + delta;

        self.move_clip(clip_id, moved_at);
    }

    pub(crate) fn move_clip(&mut self, clip_id: &str, at: Time) {
        let mut clips = self.clips.to_vec();

        let moving_clip_id = clip_id;
        let moving_clip = clips
            .iter()
            .find(|clip| clip.id.eq(moving_clip_id))
            .unwrap();

        let clip_duration = moving_clip.end_at - moving_clip.start_at;
        let moved_start_at = at;
        let moved_end_at = moved_start_at + clip_duration;

        let moving_clip_index = clips
            .iter()
            .position(|clip| clip.id.eq(moving_clip_id))
            .unwrap();

        let mut next_moving_clip_index = moving_clip_index;

        for (index, clip) in clips.iter().enumerate() {
            if index == moving_clip_index {
                continue;
            }

            let clip_center_at = (clip.start_at + clip.end_at) / 2.0;

            if index < moving_clip_index {
                if moved_start_at < clip_center_at {
                    next_moving_clip_index = index;

                    break;
                }
            } else {
                if clip_center_at < moved_end_at {
                    next_moving_clip_index = index;
                }
            }
        }

        if moving_clip_index != next_moving_clip_index {
            let min = moving_clip_index.min(next_moving_clip_index);
            let max = moving_clip_index.max(next_moving_clip_index);

            let slice_to_rotate = &mut clips[min..max + 1];
            if moving_clip_index < next_moving_clip_index {
                slice_to_rotate.rotate_left(1);
            } else {
                slice_to_rotate.rotate_right(1);
            }
        }

        push_front_camera_clips(&mut clips);

        self.clips = clips.into();
    }
}

fn push_front_camera_clips(clips: &mut [Arc<CameraClip>]) {
    let mut next_start_at = Time::zero();
    clips.iter_mut().for_each(|clip| {
        let duration = clip.end_at - clip.start_at;
        let new_clip = Arc::new(CameraClip {
            id: clip.id.clone(),
            start_at: next_start_at,
            end_at: next_start_at + duration,
            camera_angle: clip.camera_angle.clone(),
        });
        let _ = std::mem::replace(clip, new_clip);
        next_start_at = clip.end_at;
    });
}
