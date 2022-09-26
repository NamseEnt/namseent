use crate::app::game::{GameObject, Position, Tile};
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = Tile(1.0);
const VISUAL_HEIGHT: Tile = Tile(1.0);
const VISUAL_OFFSET_X: Tile = Tile(-0.5);
const VISUAL_OFFSET_Y: Tile = Tile(-0.5);

pub struct Floor {
    id: Uuid,
    position: Position,
}
impl Floor {
    pub fn new(position: Position) -> Floor {
        Floor {
            id: Uuid::new_v4(),
            position,
        }
    }
}
impl GameObject for Floor {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn render(
        &self,
        _game_context: &crate::app::game::GameState,
        rendering_context: &crate::app::game::RenderingContext,
    ) -> namui::RenderingTree {
        let position = self.get_position(rendering_context.current_time);
        render([translate(
            rendering_context.px_per_tile * (position.x + VISUAL_OFFSET_X),
            rendering_context.px_per_tile * (position.y + VISUAL_OFFSET_Y),
            simple_rect(
                Wh {
                    width: rendering_context.px_per_tile * VISUAL_WIDTH,
                    height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                },
                Color::TRANSPARENT,
                0.px(),
                Color::from_f01(0.3, 0.9, 0.3, 1.0),
            ),
        )])
    }

    fn get_position(&self, _current_time: namui::Time) -> namui::Xy<crate::app::game::Tile> {
        self.position
    }

    fn get_z_index(&self) -> i32 {
        -1
    }

    fn get_visual_area(&self, current_time: Time) -> super::VisualArea {
        let position = self.get_position(current_time);
        Rect::Xywh {
            x: position.x + VISUAL_OFFSET_X,
            y: position.y + VISUAL_OFFSET_Y,
            width: VISUAL_WIDTH,
            height: VISUAL_HEIGHT,
        }
    }

    fn get_mover(&mut self) -> Option<&mut dyn super::Mover> {
        None
    }

    fn get_collider(&mut self) -> Option<&mut dyn super::Collider> {
        None
    }
}
