use crate::app::game::Heading;
use namui::*;

#[ecs_macro::component]
pub struct PlayerCharacter {
    pub heading: Heading,
}

impl PlayerCharacter {
    pub fn heading(&self) -> Heading {
        self.heading
    }

    pub fn update_heading(&mut self, movement_direction: Xy<f32>) {
        if movement_direction.x == 0.0 {
            return;
        }
        let tangent = movement_direction.y / movement_direction.x;
        if tangent.abs() > 8.0 {
            return;
        } else if movement_direction.x > 0.0 {
            self.heading = Heading::Right;
        } else {
            self.heading = Heading::Left;
        }
    }
}
