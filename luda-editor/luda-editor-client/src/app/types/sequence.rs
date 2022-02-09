use super::*;
use serde::{Deserialize, Serialize};
use std::{collections::VecDeque, sync::Arc};

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
    pub fn find_track(&self, callback: impl Fn(&Track) -> bool) -> Option<Arc<Track>> {
        for track in self.tracks.iter() {
            if callback(track.as_ref()) {
                return Some(track.clone());
            }
        }
        None
    }
    pub fn find_track_by_clip_id(&self, clip_id: &str) -> Option<Arc<Track>> {
        self.find_track(|track| match track {
            Track::Camera(track) => track.clips.iter().any(|clip| clip.id.eq(clip_id)),
            Track::Subtitle(track) => track.clips.iter().any(|clip| clip.id.eq(clip_id)),
        })
    }
    pub fn into_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for Sequence {
    fn try_from(value: &str) -> Result<Sequence, String> {
        match serde_json::from_str::<Sequence>(&value) {
            Ok(sequence) => Ok(sequence),
            Err(error) => Err(error.to_string()),
        }
    }

    type Error = String;
}

impl Default for Sequence {
    fn default() -> Self {
        Self {
            tracks: Arc::new([
                Arc::new(Track::Camera(CameraTrack {
                    id: "camera-track".to_string(),
                    clips: vec![].into(),
                })),
                Arc::new(Track::Subtitle(SubtitleTrack {
                    id: "subtitle-track".to_string(),
                    clips: vec![].into(),
                })),
            ]),
        }
    }
}

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleTrack {
    pub id: String,
    pub clips: Arc<[Arc<SubtitleClip>]>,
}

pub const DEFAULT_SUBTITLE_INSERT_INTERVAL_MS: f32 = 1000.0;

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
                                Time::zero(),
                                right_time / (subtitles_to_insert_in_the_middle.len() as f32 + 1.0),
                            ),
                        }
                    } else {
                        let interval = Time::from_ms(DEFAULT_SUBTITLE_INSERT_INTERVAL_MS);
                        let left_time = match result_clips.last() {
                            Some(clip) => clip.start_at + interval,
                            None => Time::zero(),
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
                    let interval = Time::from_ms(DEFAULT_SUBTITLE_INSERT_INTERVAL_MS);

                    let left_time = match result_clips.last() {
                        Some(clip) => clip.start_at + interval,
                        None => Time::zero(),
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
    pub fn get_new_id() -> String {
        format!("CameraClip-{}", namui::nanoid())
    }
}

#[derive(Debug, Clone)]
pub enum Clip {
    Camera(Arc<CameraClip>),
    Subtitle(Arc<SubtitleClip>),
}

impl Clip {
    pub fn get_id(&self) -> &str {
        match self {
            Clip::Camera(clip) => &clip.id,
            Clip::Subtitle(clip) => &clip.id,
        }
    }
    pub fn as_camera_clip(&self) -> Option<&CameraClip> {
        match self {
            Clip::Camera(clip) => Some(clip.as_ref()),
            _ => None,
        }
    }

    pub(crate) fn get_start_time(&self) -> Time {
        match self {
            Clip::Camera(clip) => clip.start_at,
            Clip::Subtitle(clip) => clip.start_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleClip {
    pub id: String,
    pub start_at: Time,
    pub subtitle: Subtitle,
    #[serde(skip_serializing_if = "is_false")]
    #[serde(default)]
    pub is_needed_to_update_position: bool,
}

fn is_false(value: impl std::borrow::Borrow<bool>) -> bool {
    !value.borrow()
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

impl Track {
    pub fn find_clip(&self, clip_id: &str) -> Option<Clip> {
        self.get_clips()
            .iter()
            .find(|clip| clip.get_id().eq(clip_id))
            .map(|clip| clip.clone())
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

impl TrackReplacer<SubtitleTrack> for Sequence {
    fn replace_track(
        mut self,
        track_id: &str,
        replace_callback: impl FnOnce(&SubtitleTrack) -> Result<SubtitleTrack, String> + Copy,
    ) -> UpdateResult<Self, String> {
        match update_arcs(&mut self.tracks, |track| {
            if let Track::Subtitle(subtitle_track) = track {
                if subtitle_track.id == *track_id {
                    match replace_callback(subtitle_track) {
                        Ok(track) => UpdateResult::Updated(Track::Subtitle(track)),
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
