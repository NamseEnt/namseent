use futures::future::join_all;
use namui::{file::bundle, Time};
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy)]
pub enum Instrument {
    Kick,
    Snare,
    Cymbals,
}
const INSTRUMENTS: [Instrument; 3] = [Instrument::Kick, Instrument::Snare, Instrument::Cymbals];
impl ToString for Instrument {
    fn to_string(&self) -> String {
        match self {
            Instrument::Kick => "kick".to_string(),
            Instrument::Snare => "snare".to_string(),
            Instrument::Cymbals => "cymbals".to_string(),
        }
    }
}
impl Instrument {
    pub fn as_direction(&self) -> Direction {
        match self {
            Instrument::Kick => Direction::Down,
            Instrument::Snare => Direction::Left,
            Instrument::Cymbals => Direction::Up,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct Note {
    pub time: Time,
    pub direction: Direction,
    pub instrument: Instrument,
}

pub async fn load_notes() -> Vec<Note> {
    let note_loading_futures = INSTRUMENTS.map(|instrument| async move {
        let instrument_path = format!("bundle:{}.txt", instrument.to_string());
        let time_sequence_file = bundle::read(instrument_path.as_str()).await.unwrap();
        io::BufReader::<&[u8]>::new(time_sequence_file.as_ref())
            .lines()
            .map(|line| line.unwrap().parse::<f32>().unwrap())
            .map(|time_sec| Note {
                time: Time::Sec(time_sec),
                direction: instrument.as_direction(),
                instrument,
            })
            .collect::<Vec<Note>>()
    });
    let mut notes = join_all(note_loading_futures)
        .await
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
    notes.sort_by(|a, b| a.time.cmp(&b.time));
    notes
}
