use super::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraTrack {
    pub id: String,
    pub clips: Arc<[Arc<CameraClip>]>,
}
#[derive(Debug)]
struct MovingChunk {
    clips: Vec<Arc<CameraClip>>,
    center_time: Time,
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
    pub(crate) fn move_clips_delta<T: AsRef<str>>(&mut self, clip_ids: &[T], delta: Time) {
        let mut chunks = vec![];

        let not_moving_clips = self
            .clips
            .iter()
            .filter(|clip| !clip_ids.iter().any(|clip_id| clip_id.as_ref() == clip.id));

        not_moving_clips.for_each(|clip| {
            chunks.push(MovingChunk {
                clips: vec![clip.clone()],
                center_time: (clip.start_at + clip.end_at) / 2.0,
            });
        });

        chunks.append(&mut self.convert_moving_clips_to_chunks(clip_ids, delta));

        chunks.sort_by_key(|chunk| chunk.center_time);

        let mut clips = chunks
            .into_iter()
            .flat_map(|chunk| chunk.clips)
            .collect::<Vec<_>>();

        push_front_camera_clips(&mut clips);

        self.clips = clips.into();
    }

    fn convert_moving_clips_to_chunks<T: AsRef<str>>(
        &mut self,
        clip_ids: &[T],
        delta: Time,
    ) -> Vec<MovingChunk> {
        let mut chunks = vec![];
        let mut moving_clips = self
            .clips
            .iter()
            .filter(|clip| clip_ids.iter().any(|clip_id| clip_id.as_ref() == clip.id))
            .collect::<Vec<_>>();

        let get_clip_index =
            |clip: &CameraClip| self.clips.iter().position(|c| c.id == clip.id).unwrap();

        while !moving_clips.is_empty() {
            let clip = moving_clips.remove(0);
            let mut chunk_clips = vec![clip.clone()];
            let mut searching_clips = vec![clip];

            while !searching_clips.is_empty() {
                let searching_clip = searching_clips.pop().unwrap();
                let searching_clip_index = get_clip_index(searching_clip);

                let mut clips_next_to_searching_clip = vec![];
                moving_clips
                    .iter()
                    .filter(|clip| {
                        let clip_index = get_clip_index(clip);
                        clip_index - 1 == searching_clip_index
                            || clip_index + 1 == searching_clip_index
                    })
                    .for_each(|clip| {
                        clips_next_to_searching_clip.push(clip.clone());
                    });

                for clip in clips_next_to_searching_clip {
                    moving_clips.remove(moving_clips.iter().position(|c| c.id == clip.id).unwrap());
                    chunk_clips.push(clip.clone());
                    searching_clips.push(clip);
                }
            }

            chunks.push(MovingChunk {
                clips: chunk_clips,
                center_time: (clip.start_at + clip.end_at) / 2.0 + delta,
            });
        }
        chunks
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
