use super::{Clip, MutableClip, Track};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Sequence {
    pub tracks: Vec<Track>,
}

impl Sequence {
    pub fn get_clip(&self, id: &str) -> Option<Clip> {
        for track in &self.tracks {
            match track {
                Track::Camera(track) => {
                    for clip in &track.clips {
                        if clip.id == id {
                            return Some(Clip::Camera(clip));
                        }
                    }
                }
                Track::Subtitle(track) => {
                    for clip in &track.clips {
                        if clip.id == id {
                            return Some(Clip::Subtitle(clip));
                        }
                    }
                }
            }
        }
        None
    }
    pub fn get_mut_clip<'a>(&'a mut self, id: &str) -> Option<MutableClip<'a>> {
        for track in &mut self.tracks {
            match track {
                Track::Camera(track) => {
                    for clip in &mut track.clips {
                        if clip.id == id {
                            return Some(MutableClip::Camera(clip));
                        }
                    }
                }
                Track::Subtitle(track) => {
                    for clip in &mut track.clips {
                        if clip.id == id {
                            return Some(MutableClip::Subtitle(clip));
                        }
                    }
                }
            }
        }
        None
    }
}

impl Default for Sequence {
    fn default() -> Self {
        Sequence { tracks: Vec::new() }
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
