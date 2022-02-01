use super::JobExecute;
use crate::app::types::*;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct MoveSubtitleClipJob {
    pub clip_ids: BTreeSet<String>,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
    pub is_moved: bool,
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
        let mut replacer = subtitle_clip_replacer;
        for clip_id in &self.clip_ids {
            match replacer.replace_clip(&clip_id, |clip| {
                let mut clip = clip.clone();
                self.move_subtitle_clip(&mut clip);
                Ok(clip)
            }) {
                UpdateResult::Updated(r) => {
                    replacer = r;
                }
                UpdateResult::NotUpdated => return Err("Subtitle clip not found".to_string()),
                UpdateResult::Err(error) => return Err(error),
            }
        }
        Ok(replacer)
    }
    fn move_subtitle_clip(&self, clip: &mut SubtitleClip) {
        let delta_time = self.last_mouse_position_in_time - self.click_anchor_in_time;

        let moved_start_at = clip.start_at + delta_time;

        clip.start_at = moved_start_at;
    }
}
