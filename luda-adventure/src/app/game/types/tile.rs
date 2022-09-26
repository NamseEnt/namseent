use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Tile(pub f32);

pub trait TileExt {
    fn tile(self) -> Tile;
}

impl TileExt for f32 {
    fn tile(self) -> Tile {
        Tile(self)
    }
}

impl TileExt for i32 {
    fn tile(self) -> Tile {
        Tile(self as f32)
    }
}

impl From<f32> for Tile {
    fn from(value: f32) -> Self {
        Tile(value)
    }
}

impl Div for Tile {
    type Output = f32;

    fn div(self, rhs: Self) -> Self::Output {
        self.0 / rhs.0
    }
}
impl Mul for Tile {
    type Output = Tile;

    fn mul(self, rhs: Self) -> Self::Output {
        Tile(self.0 * rhs.0)
    }
}
impl Mul<f32> for Tile {
    type Output = Tile;

    fn mul(self, rhs: f32) -> Self::Output {
        Self(self.0 * rhs)
    }
}
impl Sub for Tile {
    type Output = Tile;

    fn sub(self, rhs: Self) -> Self::Output {
        Tile(self.0 - rhs.0)
    }
}
impl Add for Tile {
    type Output = Tile;

    fn add(self, rhs: Self) -> Self::Output {
        Tile(self.0 + rhs.0)
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}tile", f.precision().unwrap_or(0), self.0)
    }
}
