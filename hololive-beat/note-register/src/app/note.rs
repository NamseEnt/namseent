use futures::future::join_all;
use namui::{file::bundle, prelude::*};
use std::{
    io::{self, BufRead},
    iter,
};

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

    // pub fn as_instrument(&self) -> Instrument {
    //     match self {
    //         Direction::Up => Instrument::Cymbals,
    //         Direction::Right => unimplemented!(),
    //         Direction::Down => Instrument::Kick,
    //         Direction::Left => Instrument::Snare,
    //     }
    // }
}

#[derive(Debug)]
pub struct Note {
    pub start_time: Duration,
    pub end_time: Duration,
    pub direction: Direction,
    pub instrument: Instrument,
}

pub async fn load_notes() -> Vec<Note> {
    let note_loading_futures = INSTRUMENTS.map(|instrument| async move {
        let instrument_path = format!("bundle:{}.txt", instrument.to_string());
        let time_sequence_file = bundle::read(instrument_path.as_str())
            .await
            .map_err(|error| {
                println!("error: {:?}", error);
                error
            })
            .unwrap();
        let start_times = io::BufReader::<&[u8]>::new(time_sequence_file.as_ref())
            .lines()
            .map(|line| line.unwrap().parse::<f64>().unwrap().sec())
            .collect::<Vec<_>>();
        let end_times = start_times
            .iter()
            .cloned()
            .skip(1)
            .chain(iter::once(start_times.last().unwrap() + 5.sec()));
        start_times
            .iter()
            .zip(end_times)
            .map(|(start_time, end_time)| Note {
                start_time: *start_time,
                end_time,
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
    notes.sort_by(|a, b| a.start_time.cmp(&b.start_time));
    notes
}
