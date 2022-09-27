use super::Heading;
use crate::app::game::*;
use namui::prelude::*;
use namui_prebuilt::simple_rect;

const VISUAL_WIDTH: Tile = Tile(3.0);
const VISUAL_HEIGHT: Tile = Tile(4.0);
const VISUAL_OFFSET_X: Tile = Tile(-1.5);
const VISUAL_OFFSET_Y: Tile = Tile(-2.5);

pub struct PlayerCharacter {}
impl PlayerCharacter {
    pub fn new(_position: Position, _current_time: Time) -> Self {
        Self {
            // movement_plan: MovementPlan::stay_forever(position, current_time),
        }
    }
}

pub fn render_player_character(
    character: &Mover,
    rendering_context: &crate::app::game::RenderingContext,
) -> namui::RenderingTree {
    let position = character.get_position(rendering_context.current_time);
    translate(
        rendering_context.px_per_tile * (position.x + VISUAL_OFFSET_X),
        rendering_context.px_per_tile * (position.y + VISUAL_OFFSET_Y),
        render([
            simple_rect(
                Wh {
                    width: rendering_context.px_per_tile * VISUAL_WIDTH,
                    height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                },
                Color::TRANSPARENT,
                0.px(),
                Color::from_f01(0.5, 0.5, 1.0, 0.5),
            ),
            namui_prebuilt::typography::center_text(
                Wh {
                    width: rendering_context.px_per_tile * VISUAL_WIDTH,
                    height: rendering_context.px_per_tile * VISUAL_HEIGHT,
                },
                match character.heading {
                    Heading::Left => "L",
                    Heading::Right => "R",
                },
                Color::WHITE,
            ),
        ]),
    )
}

impl GameObject for PlayerCharacter {
    fn get_id(&self) -> Uuid {
        known_id::object::PLAYER_CHARACTER_OBJECT
    }

    fn get_z_index(&self) -> i32 {
        0
    }

    fn get_visual_area(&self, current_time: Time) -> crate::app::game::VisualArea {
        let position = self.get_position(current_time);
        Rect::Xywh {
            x: position.x + VISUAL_OFFSET_X,
            y: position.y + VISUAL_OFFSET_Y,
            width: VISUAL_WIDTH,
            height: VISUAL_HEIGHT,
        }
    }
    fn render(
        &self,
        _game_state: &GameState,
        _rendering_context: &RenderingContext,
    ) -> namui::RenderingTree {
        todo!()
    }

    fn get_position(&self, _current_time: Time) -> Position {
        todo!()
    }
}
