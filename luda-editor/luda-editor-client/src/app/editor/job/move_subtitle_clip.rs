use crate::app::{editor::Editor, types::*};

#[derive(Debug, Clone)]
pub struct MoveSubtitleClipJob {
    pub clip_id: String,
    pub click_anchor_in_global: namui::Xy<f32>,
    pub last_global_mouse_xy: namui::Xy<f32>,
}

fn replace_subtitle_clip<'a>(
    sequence: &'a mut Sequence,
    clip_id: &'a String,
    replace_callback: impl FnOnce(&'a mut SubtitleClip) -> Result<(), String>,
) -> Result<(), String> {
    for track in sequence.tracks.into_iter() {
        if let Track::Subtitle(subtitle_track) = track {
            for clip in subtitle_track.clips.iter_mut() {
                if clip.id == *clip_id {
                    return replace_callback(clip);
                }
            }
        }
    }
    Err("Subtitle clip not found".to_string())
}

impl MoveSubtitleClipJob {
    pub fn move_subtitle_clip_by_job(&self, clip: &mut SubtitleClip) {
        let delta_x = self.last_global_mouse_xy.x - self.click_anchor_in_global.x;
        let delta_time = PixelSize(delta_x) * *time_per_pixel;

        let moved_start_at = clip.start_at + delta_time;

        clip.start_at = moved_start_at;
    }
    pub fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        let selected_subtitle_clip = find_subtitle_clip(&mut sequence, &self.clip_id);
        if selected_subtitle_clip.is_none() {
            return Err(format!("Subtitle clip with id {} not found", self.clip_id));
        }
        let mut selected_subtitle_clip = selected_subtitle_clip.unwrap();

        self.move_subtitle_clip_by_job(&mut selected_subtitle_clip);

        Ok(sequence)
    }
}

fn find_camera_track_of_clip<'a>(
    clip_id: &'a String,
    sequence: &'a Sequence,
) -> Option<&'a CameraTrack> {
    for track in sequence.tracks.iter() {
        if let Track::Camera(camera_track) = track {
            for clip in camera_track.clips.iter() {
                if clip.id == *clip_id {
                    return Some(&camera_track);
                }
            }
        }
    }
    None
}
