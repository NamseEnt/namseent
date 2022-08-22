use super::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub id: String,
    pub clips: Arc<[Arc<SubtitleClip>]>,
}

pub const DEFAULT_SUBTITLE_INSERT_INTERVAL_MS: f32 = 1000.0;

impl SubtitleTrack {
    #[allow(dead_code)]
    pub(crate) fn get_clip_at_time(
        &self,
        time: Time,
        language: Language,
        duration_measurer: &dyn SubtitlePlayDurationMeasure,
    ) -> Option<&SubtitleClip> {
        self.clips.iter().find_map(|clip| {
            if clip.is_at_time(time, language, duration_measurer) {
                Some(clip.as_ref())
            } else {
                None
            }
        })
    }

    pub(crate) fn sync(&mut self, subtitles: &[Subtitle]) {
        let mut clip_queue: VecDeque<Arc<SubtitleClip>> = self.clips.to_vec().into();
        let mut subtitle_queue: VecDeque<Subtitle> = subtitles.to_vec().into();

        let mut result_clips = vec![];

        loop {
            let front_clip = clip_queue.front();
            let front_subtitle = subtitle_queue.front();
            if front_subtitle.is_none() {
                break;
            }
            let front_subtitle = front_subtitle.unwrap();

            match front_clip {
                Some(front_clip) => {
                    if subtitle_queue
                        .iter()
                        .all(|subtitle| subtitle.id != front_clip.id)
                    {
                        clip_queue.pop_front();
                        continue;
                    }

                    if front_clip.id == front_subtitle.id {
                        result_clips.push(Arc::new(SubtitleClip {
                            id: front_clip.id.clone(),
                            start_at: front_clip.start_at,
                            subtitle: front_subtitle.clone(),
                            is_needed_to_update_position: front_clip.is_needed_to_update_position,
                        }));
                        clip_queue.pop_front();
                        subtitle_queue.pop_front();
                        continue;
                    }

                    if clip_queue.iter().any(|clip| clip.id == front_subtitle.id) {
                        clip_queue.pop_front();
                        continue;
                    }

                    let mut subtitles_to_insert_in_the_middle =
                        vec![subtitle_queue.pop_front().unwrap()];

                    while let Some(subtitle) = subtitle_queue.front() {
                        if subtitle.id != front_clip.id {
                            subtitles_to_insert_in_the_middle
                                .push(subtitle_queue.pop_front().unwrap());
                        } else {
                            break;
                        }
                    }

                    let right_clip = subtitle_queue.front().map(|_| front_clip);

                    let (left_time, interval) = if let Some(right_clip) = right_clip {
                        let right_time = right_clip.start_at;

                        match result_clips.last() {
                            Some(clip) => {
                                let interval = (right_time - clip.start_at)
                                    / (subtitles_to_insert_in_the_middle.len() as f32 + 1.0);
                                let left_time = clip.start_at + interval;
                                (left_time, interval)
                            }
                            None => (
                                Time::Ms(0.0),
                                right_time / (subtitles_to_insert_in_the_middle.len() as f32 + 1.0),
                            ),
                        }
                    } else {
                        let interval = Time::Ms(DEFAULT_SUBTITLE_INSERT_INTERVAL_MS);
                        let left_time = match result_clips.last() {
                            Some(clip) => clip.start_at + interval,
                            None => Time::Ms(0.0),
                        };
                        (left_time, interval)
                    };

                    subtitles_to_insert_in_the_middle
                        .iter()
                        .enumerate()
                        .for_each(|(index, subtitle)| {
                            let start_at = left_time + interval * (index as f32);
                            result_clips.push(Arc::new(SubtitleClip {
                                id: subtitle.id.clone(),
                                start_at,
                                subtitle: subtitle.clone(),
                                is_needed_to_update_position: true,
                            }));
                        });
                }
                None => {
                    let interval = Time::Ms(DEFAULT_SUBTITLE_INSERT_INTERVAL_MS);

                    let left_time = match result_clips.last() {
                        Some(clip) => clip.start_at + interval,
                        None => Time::Ms(0.0),
                    };

                    subtitle_queue
                        .iter()
                        .enumerate()
                        .for_each(|(index, subtitle)| {
                            let start_at = left_time + interval * (index as f32);
                            result_clips.push(Arc::new(SubtitleClip {
                                id: subtitle.id.clone(),
                                start_at,
                                subtitle: subtitle.clone(),
                                is_needed_to_update_position: true,
                            }));
                        });
                    subtitle_queue.clear();
                    break;
                }
            }
        }
        self.clips = result_clips.into();
    }

    pub(crate) fn move_clips_delta<T: AsRef<str>>(&mut self, clip_ids: &[T], delta: Time) {
        self.clips = self
            .clips
            .iter()
            .map(|clip| {
                if clip_ids.iter().any(|clip_id| clip_id.as_ref() == clip.id) {
                    Arc::new(SubtitleClip {
                        id: clip.id.clone(),
                        start_at: clip.start_at + delta,
                        subtitle: clip.subtitle.clone(),
                        is_needed_to_update_position: false,
                    })
                } else {
                    clip.clone()
                }
            })
            .collect::<Vec<_>>()
            .into();
    }
}
