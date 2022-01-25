use super::JobExecute;
use crate::app::types::*;

#[derive(Debug, Clone)]
pub struct MoveSubtitleClipJob {
    pub clip_id: String,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
}

impl JobExecute for MoveSubtitleClipJob {
    fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        self.move_subtitle_clip_in(sequence)
    }
}

impl MoveSubtitleClipJob {
    pub fn move_subtitle_clip_in<T>(&self, subtitle_clip_replacer: T) -> Result<T, String>
    where
        T: ClipReplacer<SubtitleClip>,
    {
        match subtitle_clip_replacer.replace_clip(&self.clip_id, |clip| {
            let mut clip = clip.clone();
            self.move_subtitle_clip(&mut clip);
            Ok(clip)
        }) {
            UpdateResult::Updated(replacer) => Ok(replacer),
            UpdateResult::NotUpdated => Err("Subtitle clip not found".to_string()),
            UpdateResult::Err(error) => Err(error),
        }
    }
    fn move_subtitle_clip(&self, clip: &mut SubtitleClip) {
        let delta_time = self.last_mouse_position_in_time - self.click_anchor_in_time;

        let moved_start_at = clip.start_at + delta_time;

        clip.start_at = moved_start_at;
    }
}
