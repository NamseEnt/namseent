use super::super::Timeline;
use crate::app::types::*;

#[derive(Debug, Clone)]
pub struct MoveSubtitleClipJob {
    pub clip_id: String,
    pub click_anchor_in_global: namui::Xy<f32>,
    pub last_global_mouse_xy: namui::Xy<f32>,
}

fn find_subtitle_clip<'a>(
    sequence: &'a mut Sequence,
    clip_id: &'a String,
) -> Option<&'a mut SubtitleClip> {
    for track in &mut sequence.tracks {
        if let Track::Subtitle(subtitle_track) = track {
            for clip in &mut subtitle_track.clips {
                if clip.id == *clip_id {
                    return Some(clip);
                }
            }
        }
    }
    None
}

impl MoveSubtitleClipJob {
    pub fn move_subtitle_clip_by_job(
        &self,
        clip: &mut SubtitleClip,
        time_per_pixel: &TimePerPixel,
    ) {
        let delta_x = self.last_global_mouse_xy.x - self.click_anchor_in_global.x;
        let delta_time = PixelSize(delta_x) * *time_per_pixel;

        let moved_start_at = clip.start_at + delta_time;

        clip.start_at = moved_start_at;
    }
    pub fn execute(&self, timeline: &mut Timeline) {
        let selected_subtitle_clip = find_subtitle_clip(&mut timeline.sequence, &self.clip_id);
        if selected_subtitle_clip.is_none() {
            return;
        }
        let mut selected_subtitle_clip = selected_subtitle_clip.unwrap();

        self.move_subtitle_clip_by_job(&mut selected_subtitle_clip, &timeline.time_per_pixel);
    }
}
