use namui::*;
use namui_prebuilt::{table::*, *};

struct GameState {
    map: GameMap,
    monsters: Vec<Monster>,
    towers: Vec<Tower>,
    camera: Camera,
}

struct GameMap {}
struct Monster {
    travel_destination: TravelDestination,
    position: Xy<f32>,
    kind: MonsterKind,
}
enum TravelDestination {
    One,
    Two,
    Three,
    Four,
    Five,
    Finish,
}
enum MonsterKind {}
struct Tower {
    position: Xy<usize>,
    kind: TowerKind,
}
enum TowerKind {}
struct Camera {
    left_top: Xy<f32>,
    zoom_level: ZoomLevel,
}
enum ZoomLevel {
    Default,
    ZoomOut,
}

pub fn main() {
    namui::start(|ctx| {
        return;
    });
}
