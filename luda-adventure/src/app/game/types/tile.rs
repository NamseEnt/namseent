use std::fmt::Display;

namui::common_for_f32_type!(Tile, tile, TileExt);

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.*?}tile", f.precision().unwrap_or(0), self.0)
    }
}
