use crate::app::types::*;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct MoveSubtitleClipJob {
    pub clip_id: String,
    pub click_anchor_in_time: Time,
    pub last_mouse_position_in_time: Time,
}

impl MoveSubtitleClipJob {
    pub fn execute(&self, sequence: &Sequence) -> Result<Sequence, String> {
        let sequence = sequence.clone();
        self.move_subtitle_clip_in(sequence)
    }
    pub fn move_subtitle_clip_in<T>(&self, subtitle_clip_replacer: T) -> Result<T, String>
    where
        T: SubtitleClipReplacer,
    {
        match subtitle_clip_replacer.replace_subtitle_clip(&self.clip_id, |clip| {
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

pub enum UpdateResult<T, Error> {
    Updated(T),
    NotUpdated,
    Err(Error),
}

fn update_arcs<T, Error>(
    arcs: &mut Arc<[Arc<T>]>,
    callback: impl Fn(&T) -> UpdateResult<T, Error>,
) -> UpdateResult<(), Error> {
    let mut vec = arcs.to_vec();
    let mut is_updated = false;
    for arc in vec.iter_mut() {
        match callback(arc.as_ref()) {
            UpdateResult::Updated(updated_element) => {
                is_updated = true;
                let _ = std::mem::replace(arc, Arc::new(updated_element));
            }
            UpdateResult::NotUpdated => continue,
            UpdateResult::Err(error) => return UpdateResult::Err(error),
        }
    }
    if is_updated {
        *arcs = vec.into();
        UpdateResult::Updated(())
    } else {
        UpdateResult::NotUpdated
    }
}

pub trait SubtitleClipReplacer {
    fn replace_subtitle_clip(
        self,
        clip_id: &str,
        replace_callback: impl FnOnce(&SubtitleClip) -> Result<SubtitleClip, String> + Copy,
    ) -> UpdateResult<Self, String>
    where
        Self: Sized;
}

impl SubtitleClipReplacer for Sequence {
    fn replace_subtitle_clip(
        mut self,
        clip_id: &str,
        replace_callback: impl FnOnce(&SubtitleClip) -> Result<SubtitleClip, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.tracks, |track| {
            if let Track::Subtitle(subtitle_track) = track {
                let subtitle_track = subtitle_track.clone();
                match subtitle_track.replace_subtitle_clip(clip_id, replace_callback) {
                    UpdateResult::Updated(subtitle_track) => {
                        UpdateResult::Updated(Track::Subtitle(subtitle_track))
                    }
                    UpdateResult::NotUpdated => UpdateResult::NotUpdated,
                    UpdateResult::Err(error) => UpdateResult::Err(error),
                }
            } else {
                return UpdateResult::NotUpdated;
            }
        }) {
            UpdateResult::Updated(_) => UpdateResult::Updated(self),
            UpdateResult::NotUpdated => UpdateResult::NotUpdated,
            UpdateResult::Err(error) => UpdateResult::Err(error),
        }
    }
}

impl SubtitleClipReplacer for SubtitleTrack {
    fn replace_subtitle_clip(
        mut self,
        clip_id: &str,
        replace_callback: impl FnOnce(&SubtitleClip) -> Result<SubtitleClip, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.clips, |clip| {
            if clip.id == *clip_id {
                match replace_callback(clip) {
                    Ok(next_subtitle_clip) => UpdateResult::Updated(next_subtitle_clip),
                    Err(error) => {
                        return UpdateResult::Err(error);
                    }
                }
            } else {
                UpdateResult::NotUpdated
            }
        }) {
            UpdateResult::Updated(_) => UpdateResult::Updated(self),
            UpdateResult::NotUpdated => UpdateResult::NotUpdated,
            UpdateResult::Err(error) => UpdateResult::Err(error),
        }
    }
}
