use super::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub tracks: Arc<[Arc<Track>]>,
}

impl Sequence {
    pub fn get_clip(&self, id: &str) -> Option<Clip> {
        for track in self.tracks.iter() {
            match track.as_ref() {
                Track::Camera(track) => {
                    for clip in track.clips.iter() {
                        if clip.id == id {
                            return Some(Clip::Camera(clip.clone()));
                        }
                    }
                }
                Track::Subtitle(track) => {
                    for clip in track.clips.iter() {
                        if clip.id == id {
                            return Some(Clip::Subtitle(clip.clone()));
                        }
                    }
                }
            }
        }
        None
    }
}

impl TryFrom<Vec<u8>> for Sequence {
    fn try_from(value: Vec<u8>) -> Result<Sequence, String> {
        match String::from_utf8(value) {
            Ok(string) => match serde_json::from_str::<Sequence>(&string) {
                Ok(sequence) => Ok(sequence),
                Err(error) => Err(error.to_string()),
            },
            Err(error) => Err(error.to_string()),
        }
    }

    type Error = String;
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            tracks: Arc::new([]),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Track {
    Camera(CameraTrack),
    Subtitle(SubtitleTrack),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub id: String,
    pub clips: Arc<[Arc<SubtitleClip>]>,
}
impl SubtitleTrack {
    pub(crate) fn get_clip_at_time(
        &self,
        time: &Time,
        language: Language,
        duration_measurer: &SubtitlePlayDurationMeasurer,
    ) -> Option<&SubtitleClip> {
        self.clips.iter().find_map(|clip| {
            if clip.is_at_time(&time, language, duration_measurer) {
                Some(clip.as_ref())
            } else {
                None
            }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CameraClip {
    pub id: String,
    pub start_at: Time,
    pub end_at: Time,
    pub camera_angle: CameraAngle,
}
impl CameraClip {
    pub fn is_at_time(&self, time: &Time) -> bool {
        self.start_at <= time && time < self.end_at
    }
    pub fn duplicate(&self) -> CameraClip {
        CameraClip {
            id: CameraClip::get_new_id(),
            start_at: self.start_at,
            end_at: self.end_at,
            camera_angle: self.camera_angle.clone(),
        }
    }
    fn get_new_id() -> String {
        format!("CameraClip-{}", namui::nanoid())
    }
}

#[derive(Debug)]
pub enum Clip {
    Camera(Arc<CameraClip>),
    Subtitle(Arc<SubtitleClip>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleClip {
    pub id: String,
    pub start_at: Time,
    pub subtitle: Subtitle,
}
impl SubtitleClip {
    fn is_at_time(
        &self,
        time: &Time,
        language: Language,
        duration_measurer: &SubtitlePlayDurationMeasurer,
    ) -> bool {
        self.start_at <= time && time < self.end_at(language, duration_measurer)
    }

    pub(crate) fn end_at(
        &self,
        language: Language,
        duration_measurer: &SubtitlePlayDurationMeasurer,
    ) -> Time {
        self.start_at + duration_measurer.get_play_duration(&self.subtitle, &language)
    }
}

pub enum UpdateResult<T, Error> {
    Updated(T),
    NotUpdated,
    Err(Error),
}

impl<T, Error> UpdateResult<T, Error>
where
    Error: std::fmt::Display,
{
    pub fn unwrap(self) -> T {
        match self {
            UpdateResult::Updated(value) => value,
            UpdateResult::NotUpdated => panic!("Not updated"),
            UpdateResult::Err(error) => panic!("{}", error),
        }
    }
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

pub trait ClipReplacer<ClipType> {
    fn replace_clip(
        self,
        clip_id: &str,
        replace_callback: impl FnOnce(&ClipType) -> Result<ClipType, String> + Copy,
    ) -> UpdateResult<Self, String>
    where
        Self: Sized;
}

impl ClipReplacer<SubtitleClip> for Sequence {
    fn replace_clip(
        mut self,
        clip_id: &str,
        replace_callback: impl FnOnce(&SubtitleClip) -> Result<SubtitleClip, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.tracks, |track| {
            if let Track::Subtitle(track) = track {
                let track = track.clone();
                match track.replace_clip(clip_id, replace_callback) {
                    UpdateResult::Updated(track) => UpdateResult::Updated(Track::Subtitle(track)),
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

impl ClipReplacer<CameraClip> for Sequence {
    fn replace_clip(
        mut self,
        clip_id: &str,
        replace_callback: impl FnOnce(&CameraClip) -> Result<CameraClip, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.tracks, |track| {
            if let Track::Camera(track) = track {
                let track = track.clone();
                match track.replace_clip(clip_id, replace_callback) {
                    UpdateResult::Updated(track) => UpdateResult::Updated(Track::Camera(track)),
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

impl ClipReplacer<CameraClip> for CameraTrack {
    fn replace_clip(
        mut self,
        clip_id: &str,
        replace_callback: impl FnOnce(&CameraClip) -> Result<CameraClip, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.clips, |clip| {
            if clip.id == *clip_id {
                match replace_callback(clip) {
                    Ok(clip) => UpdateResult::Updated(clip),
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

impl ClipReplacer<SubtitleClip> for SubtitleTrack {
    fn replace_clip(
        mut self,
        clip_id: &str,
        replace_callback: impl FnOnce(&SubtitleClip) -> Result<SubtitleClip, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.clips, |clip| {
            if clip.id == *clip_id {
                match replace_callback(clip) {
                    Ok(clip) => UpdateResult::Updated(clip),
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

pub trait ClipFind<ClipType> {
    fn find_clip(&self, clip_id: &str) -> Option<&ClipType>
    where
        Self: Sized;
}

impl ClipFind<CameraClip> for Sequence {
    fn find_clip(&self, clip_id: &str) -> Option<&CameraClip> {
        self.tracks.iter().find_map(|track| {
            if let Track::Camera(track) = track.as_ref() {
                track
                    .clips
                    .iter()
                    .find(|clip| clip.id == *clip_id)
                    .map(|clip| clip.as_ref())
            } else {
                None
            }
        })
    }
}

pub trait TrackReplacer<TrackType> {
    fn replace_track(
        self,
        track_id: &str,
        replace_callback: impl FnOnce(&TrackType) -> Result<TrackType, String> + Copy,
    ) -> UpdateResult<Self, String>
    where
        Self: Sized;
}

impl TrackReplacer<CameraTrack> for Sequence {
    fn replace_track(
        mut self,
        track_id: &str,
        replace_callback: impl FnOnce(&CameraTrack) -> Result<CameraTrack, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.tracks, |track| {
            if let Track::Camera(camera_track) = track {
                if camera_track.id == *track_id {
                    match replace_callback(camera_track) {
                        Ok(track) => UpdateResult::Updated(Track::Camera(track)),
                        Err(error) => {
                            return UpdateResult::Err(error);
                        }
                    }
                } else {
                    return UpdateResult::NotUpdated;
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
