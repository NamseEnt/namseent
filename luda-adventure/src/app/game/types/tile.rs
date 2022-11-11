use float_cmp::{ApproxEq, F32Margin};
use std::fmt::Display;

namui::common_for_f32_type!(Tile, tile, TileExt);

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}tile", f.precision().unwrap_or(0), self.0)
    }
}

impl ApproxEq for Tile {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        self.0.approx_eq(other.0, margin.into())
    }
}

impl Tile {
    pub fn approx_le<M: Into<F32Margin>>(self, other: Self, margin: M) -> bool {
        self.approx_eq(other, margin) || self < other
    }
    pub fn approx_ge<M: Into<F32Margin>>(self, other: Self, margin: M) -> bool {
        self.approx_eq(other, margin) || self > other
    }
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}
impl Eq for Tile {}
