use super::PlayerCharacter;
use crate::app::game::{Collider, GameObject, Mover, Position, Tile, TileExt, Velocity};
use float_cmp::assert_approx_eq;
use namui::prelude::*;
use wasm_bindgen_test::wasm_bindgen_test;

#[test]
#[wasm_bindgen_test]
fn move_to_wall_then_move_along_wall_finally_stop_at_corner() {
    // Character going upper right
    // Step 0: Init
    // ══════════════════╗
    //                   ║
    //    ▲              ║
    //
    // *******************
    //
    // Step 1: Move to wall
    // ══════════════════╗
    //     ▲             ║
    //    /              ║
    //
    // *******************
    //
    // Step 2: Move along wall
    // ══════════════════╗
    //          -▲       ║
    //                   ║
    //
    // *******************
    //
    // Step 3: Stay corner forever
    // ══════════════════╗
    //                  ▲║
    //                   ║

    // Step 0: Init
    let mut walls = mock_walls();
    let collision_box_list_without_character = walls
        .iter_mut()
        .filter_map(|wall| wall.get_collider())
        .map(|wall| wall.get_collision_box(0.sec()))
        .collect::<Vec<_>>();

    let mut character = PlayerCharacter::new(
        Position {
            x: 2.tile(),
            y: 4.tile(),
        },
        0.sec(),
    );
    character.set_velocity(
        0.sec(),
        Velocity {
            x: Per::new(1.tile(), 1.sec()),
            y: Per::new((-1).tile(), 1.sec()),
        },
        f32::INFINITY.sec(),
    );
    while character.get_predicted_movement_end_time() < 20.sec() {
        character.predict_movement(&collision_box_list_without_character);
    }

    // Step 1: Move to wall
    let position_at_step_1 = character.get_position(2.sec());
    assert_approx_eq!(f32, position_at_step_1.x.0, 4.0);
    assert_approx_eq!(f32, position_at_step_1.y.0, 2.0);

    // Step 2: Move along wall
    let position_at_step_2 = character.get_position(4.sec());
    assert_approx_eq!(f32, position_at_step_2.x.0, 6.0);
    assert_approx_eq!(f32, position_at_step_2.y.0, 2.0);

    // Step 3: Stay corner forever
    let position_at_step_3 = character.get_position(20.sec());
    assert_approx_eq!(f32, position_at_step_3.x.0, 7.0);
    assert_approx_eq!(f32, position_at_step_3.y.0, 2.0);
}

fn mock_walls() -> Vec<Box<impl GameObject>> {
    let mut walls = Vec::new();
    for x in 0..10 {
        match x {
            9 => {
                for y in 0..10 {
                    walls.push(mock_wall(x, y))
                }
            }
            _ => walls.push(mock_wall(x, 0)),
        }
    }
    walls
}

fn mock_wall(x: i32, y: i32) -> Box<impl GameObject> {
    Box::new(MockWall {
        position: Position::new(x.tile(), y.tile()),
    })
}

struct MockWall {
    position: Xy<Tile>,
}
impl GameObject for MockWall {
    fn get_id(&self) -> namui::Uuid {
        todo!()
    }
    fn render(
        &self,
        _game_state: &crate::app::game::GameState,
        _rendering_context: &crate::app::game::RenderingContext,
    ) -> namui::RenderingTree {
        todo!()
    }
    fn get_position(&self, _current_time: namui::Time) -> crate::app::game::Position {
        todo!()
    }
    fn get_z_index(&self) -> i32 {
        todo!()
    }
    fn get_visual_area(&self, _current_time: namui::Time) -> crate::app::game::VisualArea {
        todo!()
    }
    fn get_mover(&mut self) -> Option<&mut dyn crate::app::game::Mover> {
        todo!()
    }
    fn get_collider(&mut self) -> Option<&mut dyn crate::app::game::Collider> {
        return Some(self);
    }
}
impl Collider for MockWall {
    fn get_collision_box(&self, _current_time: namui::Time) -> crate::app::game::CollisionBox {
        namui::Rect::Xywh {
            x: self.position.x - 0.5.tile(),
            y: self.position.y - 0.5.tile(),
            width: 1.tile(),
            height: 1.tile(),
        }
    }
}
