use super::{
    types::{CameraClip, CameraTrack, PixelSize, Sequence, Time, TimePerPixel, Track},
    Timeline,
};

#[derive(Debug, Clone)]
pub enum Job {
    MoveCameraClip(MoveCameraClipJob),
}
#[derive(Debug, Clone)]
pub struct MoveCameraClipJob {
    pub clip_id: String,
    pub click_anchor_in_global: namui::Xy<f32>,
    pub last_global_mouse_xy: namui::Xy<f32>,
}

fn find_camera_clip_in_sequence<'a>(
    clip_id: &'a String,
    sequence: &'a mut Sequence,
) -> Option<&'a mut CameraClip> {
    for track in &mut sequence.tracks {
        if let Track::Camera(camera_track) = track {
            let clip = camera_track.0.iter_mut().find(|clip| clip.id.eq(clip_id));
            if clip.is_some() {
                return clip;
            }
        }
    }
    None
}

fn find_camera_track_of_clip<'a>(
    clip_id: &'a String,
    sequence: &'a mut Sequence,
) -> Option<&'a mut CameraTrack> {
    for track in &mut sequence.tracks {
        if let Track::Camera(camera_track) = track {
            for clip in &mut camera_track.0 {
                if clip.id == *clip_id {
                    return Some(camera_track);
                }
            }
        }
    }
    None
}

impl MoveCameraClipJob {
    pub fn move_camera_clip_by_job(&self, clip: &mut CameraClip, time_per_pixel: &TimePerPixel) {
        let delta_x = self.last_global_mouse_xy.x - self.click_anchor_in_global.x;
        let delta_time = PixelSize(delta_x) * *time_per_pixel;
        let clip_duration = clip.end_at - clip.start_at;
        let moved_start_at = clip.start_at + delta_time;
        let moved_end_at = moved_start_at + clip_duration;

        clip.start_at = moved_start_at;
        clip.end_at = moved_end_at;
    }
    pub fn order_clips_by_moving_clip(
        &self,
        clips: &mut Vec<CameraClip>,
        time_per_pixel: &TimePerPixel,
        is_preview: bool,
    ) {
        let moving_clip_id = &self.clip_id;
        let moving_clip = clips
            .iter()
            .find(|clip| clip.id.eq(moving_clip_id))
            .unwrap();

        let delta_x = self.last_global_mouse_xy.x - self.click_anchor_in_global.x;
        let delta_time = PixelSize(delta_x) * *time_per_pixel;
        let clip_duration = moving_clip.end_at - moving_clip.start_at;
        let moved_start_at = moving_clip.start_at + delta_time;
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

            let clip_center_at = (clip.start_at + clip.end_at) / 2;

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
            let moving_clip = clips.remove(moving_clip_index);
            clips.insert(next_moving_clip_index, moving_clip);
        }

        push_front_camera_clips(clips);

        if is_preview {
            clips
                .iter_mut()
                .find(|clip| clip.id.eq(moving_clip_id))
                .map(|moving_clip| {
                    moving_clip.start_at = moved_start_at;
                    moving_clip.end_at = moved_end_at;
                });
        }
    }
    pub fn execute(&self, timeline: &mut Timeline) {
        let track = find_camera_track_of_clip(&self.clip_id, &mut timeline.sequence).unwrap();
        self.order_clips_by_moving_clip(&mut track.0, &timeline.time_per_pixel, false);
    }
}

fn push_front_camera_clips(clips: &mut Vec<CameraClip>) {
    let mut next_start_at = Time::zero();
    for clip in clips {
        let duration = clip.end_at - clip.start_at;
        clip.start_at = next_start_at;
        clip.end_at = clip.start_at + duration;
        next_start_at = clip.end_at;
    }
}
