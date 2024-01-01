use futures::future::join_all;
use namui::{file::bundle, prelude::*, Instant};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}
impl TryFrom<Code> for Direction {
    type Error = ();

    fn try_from(value: Code) -> Result<Self, Self::Error> {
        match value {
            Code::ArrowUp => Ok(Direction::Up),
            Code::ArrowLeft => Ok(Direction::Left),
            Code::ArrowRight => Ok(Direction::Right),
            Code::ArrowDown => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}
impl Direction {
    pub fn lane(&self) -> usize {
        match self {
            Direction::Up => 3,
            Direction::Right => 2,
            Direction::Down => 1,
            Direction::Left => 0,
        }
    }

    pub fn as_instrument(&self) -> Instrument {
        match self {
            Direction::Up => Instrument::Cymbals,
            Direction::Right => unimplemented!(),
            Direction::Down => Instrument::Kick,
            Direction::Left => Instrument::Snare,
        }
    }
}

#[derive(Debug)]
pub struct Note {
    pub time: Instant,
    pub direction: Direction,
    pub instrument: Instrument,
}

pub async fn load_notes() -> Vec<Note> {
    let note_loading_futures = INSTRUMENTS.map(|instrument| async move {
        let instrument_path = format!("bundle:{}.txt", instrument.to_string());
        println!("instrument_path: {}", instrument_path);
        let time_sequence_file = bundle::read(instrument_path.as_str())
            .await
            .map_err(|error| {
                println!("error: {:?}", error);
                error
            })
            .unwrap();

        println!("time_sequence_file: {:?}", time_sequence_file.len());
        io::BufReader::<&[u8]>::new(time_sequence_file.as_ref())
            .lines()
            .map(|line| line.unwrap().parse::<f64>().unwrap())
            .map(|time_sec| Note {
                time: Instant::new(time_sec.sec()),
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
