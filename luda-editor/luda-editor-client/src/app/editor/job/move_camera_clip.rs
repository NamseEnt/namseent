use super::JobExecute;
use crate::app::types::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MoveCameraClipJob {
    pub clip_id: String,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
}

impl JobExecute for MoveCameraClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let mut sequence = sequence.clone();
        let track = find_camera_track_of_clip(&self.clip_id, &sequence);
        if track.is_none() {
            return Err(format!("camera track not found"));
        }
        let mut track = track.unwrap().clone();
        self.order_clips_by_moving_clip(&mut track, false);
        replace_track(&mut sequence, track);
        Ok(sequence)
    }
}

impl MoveCameraClipJob {
    fn get_delta_time(&self) -> Time {
        self.last_mouse_position_in_time - self.click_anchor_in_time
    }
    pub fn order_clips_by_moving_clip(&self, camera_track: &mut CameraTrack, is_preview: bool) {
        let mut clips = camera_track.clips.to_vec();

        let moving_clip_id = &self.clip_id;
        let moving_clip = clips
            .iter()
            .find(|clip| clip.id.eq(moving_clip_id))
            .unwrap();

        let clip_duration = moving_clip.end_at - moving_clip.start_at;
        let moved_start_at = moving_clip.start_at + self.get_delta_time();
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

        if is_preview {
            clips
                .iter_mut()
                .find(|clip| clip.id.eq(moving_clip_id))
                .map(|moving_clip| {
                    let new_clip = Arc::new(CameraClip {
                        id: moving_clip.id.clone(),
                        start_at: moved_start_at,
                        end_at: moved_end_at,
                        camera_angle: moving_clip.camera_angle.clone(),
                    });
                    let _ = std::mem::replace(moving_clip, new_clip);
                });
        }
        camera_track.clips = clips.into();
    }
}

fn replace_track(sequence: &mut Sequence, track: CameraTrack) {
    let track_index = sequence
        .tracks
        .iter()
        .position(|t| {
            if let Track::Camera(t) = t.as_ref() {
                t.id == track.id
            } else {
                false
            }
        })
        .unwrap();

    let mut next_tracks = sequence.tracks.to_vec();
    next_tracks[track_index] = Arc::new(Track::Camera(track));
    sequence.tracks = next_tracks.into();
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

fn find_camera_track_of_clip<'a>(
    clip_id: &'a String,
    sequence: &'a Sequence,
) -> Option<&'a CameraTrack> {
    for track in sequence.tracks.iter() {
        if let Track::Camera(camera_track) = track.as_ref() {
            for clip in camera_track.clips.iter() {
                if clip.id == *clip_id {
                    return Some(&camera_track);
                }
            }
        }
    }
    None
}
