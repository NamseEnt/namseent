use super::*;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
mod camera_track;
pub use camera_track::*;
mod subtitle_track;
pub use subtitle_track::*;

#[derive(Serialize, Deserialize, Clone)]
pub enum Track {
    Camera(CameraTrack),
    Subtitle(SubtitleTrack),
}

impl Track {
    pub fn get_id(&self) -> &str {
        match self {
            Track::Camera(track) => &track.id,
            Track::Subtitle(track) => &track.id,
        }
    }

    pub fn get_clips(&self) -> Vec<Clip> {
        match self {
            Track::Camera(track) => track
                .clips
                .iter()
                .map(|clip| Clip::Camera(clip.clone()))
                .collect::<Vec<_>>(),
            Track::Subtitle(track) => track
                .clips
                .iter()
                .map(|clip| Clip::Subtitle(clip.clone()))
                .collect::<Vec<_>>(),
        }
    }

    pub fn find_clip(&self, clip_id: &str) -> Option<Clip> {
        self.get_clips()
            .iter()
            .find(|clip| clip.get_id().eq(clip_id))
            .map(|clip| clip.clone())
    }

    pub(crate) fn move_clips_delta(&mut self, clip_ids: &[&String], delta_time: Time) {
        match self {
            Track::Camera(track) => track.move_clips_delta(clip_ids, delta_time),
            Track::Subtitle(track) => track.move_clips_delta(clip_ids, delta_time),
        }
    }
}

impl From<Track> for CameraTrack {
    fn from(track: Track) -> Self {
        match track {
            Track::Camera(track) => track,
            _ => panic!("Track is not a camera track"),
        }
    }
}
impl From<CameraTrack> for Track {
    fn from(track: CameraTrack) -> Self {
        Track::Camera(track)
    }
}
impl From<Track> for SubtitleTrack {
    fn from(track: Track) -> Self {
        match track {
            Track::Subtitle(track) => track,
            _ => panic!("Track is not a subtitle track"),
        }
    }
}
impl From<SubtitleTrack> for Track {
    fn from(track: SubtitleTrack) -> Self {
        Track::Subtitle(track)
    }
}
impl From<Track> for ResizableTrack {
    fn from(track: Track) -> Self {
        match track {
            Track::Camera(track) => ResizableTrack::Camera(track),
            _ => panic!("Track is not a resizable track"),
        }
    }
}
impl From<ResizableTrack> for Track {
    fn from(track: ResizableTrack) -> Self {
        match track {
            ResizableTrack::Camera(track) => Track::Camera(track),
        }
    }
}

pub enum ResizableTrack {
    Camera(CameraTrack),
}
impl ResizableTrack {
    pub(crate) fn resize_clip_delta(&mut self, clip_id: &str, get_delta_time: Time) {
        match self {
            ResizableTrack::Camera(track) => track.resize_clip_delta(clip_id, get_delta_time),
        }
    }
}
impl From<CameraTrack> for ResizableTrack {
    fn from(track: CameraTrack) -> Self {
        ResizableTrack::Camera(track)
    }
}
impl From<ResizableTrack> for CameraTrack {
    fn from(track: ResizableTrack) -> Self {
        match track {
            ResizableTrack::Camera(track) => track,
        }
    }
}
