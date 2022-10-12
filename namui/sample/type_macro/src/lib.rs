#![allow(dead_code)]

use namui::prelude::*;

namui::common_for_f32_type!(Tile, tile, TileExt);

namui::vector_types!(MyXy, { x, y });

fn test() {
    let mut xy = MyXy::new(1.tile(), 2.tile());
    xy *= 5;
}
