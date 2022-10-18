use crate::app::game::Tile;
use namui::prelude::*;

pub type CollisionBox = Rect<Tile>;

#[derive(ecs_macro::Component)]
pub struct Collider {
    collision_offset_rect: Rect<Tile>,
}

impl Collider {
    pub fn new(collision_offset_rect: Rect<Tile>) -> Self {
        Self {
            collision_offset_rect,
        }
    }
    pub fn get_collision_box(&self, xy: Xy<Tile>) -> CollisionBox {
        Rect::Xywh {
            x: xy.x + self.collision_offset_rect.x(),
            y: xy.y + self.collision_offset_rect.y(),
            width: self.collision_offset_rect.width(),
            height: self.collision_offset_rect.height(),
        }
    }
}

pub fn simplify_collision_box_list(collision_box_list: Vec<CollisionBox>) -> Vec<CollisionBox> {
    let mut simplified_collision_box_list = Vec::new();
    for collision_box in collision_box_list {
        merge_or_create_collision_box(collision_box, &mut simplified_collision_box_list);
    }
    simplified_collision_box_list
}
fn merge_or_create_collision_box(
    collision_box: CollisionBox,
    simplified_collision_box_list: &mut Vec<CollisionBox>,
) {
    let collision_box_area = get_collision_box_area(collision_box);
    for simplified_collision_box in simplified_collision_box_list.iter_mut() {
        let merged_collision_box =
            simplified_collision_box.get_minimum_rectangle_containing(collision_box);
        let merged_area = get_collision_box_area(merged_collision_box);
        let summed_area = collision_box_area + get_collision_box_area(*simplified_collision_box);
        let can_merge = merged_area == summed_area;
        if can_merge {
            *simplified_collision_box = merged_collision_box;
            return;
        }
    }
    simplified_collision_box_list.push(collision_box);
}
fn get_collision_box_area(collision_box: CollisionBox) -> f32 {
    collision_box.width().as_f32() * collision_box.height().as_f32()
}

#[cfg(test)]
mod test {
    use super::{simplify_collision_box_list, CollisionBox};
    use crate::app::game::types::TileExt;
    use wasm_bindgen_test::wasm_bindgen_test;

    #[test]
    #[wasm_bindgen_test]
    fn boxes_placed_nearby_should_be_merged() {
        let collision_box_list = vec![
            CollisionBox::Xywh {
                x: 0.tile(),
                y: 0.tile(),
                width: 1.tile(),
                height: 1.tile(),
            },
            CollisionBox::Xywh {
                x: 0.tile(),
                y: 1.tile(),
                width: 1.tile(),
                height: 1.tile(),
            },
        ];
        let expected_collision_box_list = vec![CollisionBox::Ltrb {
            left: 0.tile(),
            top: 0.tile(),
            right: 1.tile(),
            bottom: 2.tile(),
        }];
        let actual_collision_box_list = simplify_collision_box_list(collision_box_list);
        assert_eq!(actual_collision_box_list, expected_collision_box_list);
    }

    #[test]
    #[wasm_bindgen_test]
    fn boxes_placed_diagonally_should_not_be_merged() {
        let collision_box_list = vec![
            CollisionBox::Xywh {
                x: 0.tile(),
                y: 0.tile(),
                width: 1.tile(),
                height: 1.tile(),
            },
            CollisionBox::Xywh {
                x: 1.tile(),
                y: 1.tile(),
                width: 1.tile(),
                height: 1.tile(),
            },
        ];
        let expected_collision_box_list = vec![
            CollisionBox::Xywh {
                x: 0.tile(),
                y: 0.tile(),
                width: 1.tile(),
                height: 1.tile(),
            },
            CollisionBox::Xywh {
                x: 1.tile(),
                y: 1.tile(),
                width: 1.tile(),
                height: 1.tile(),
            },
        ];
        let actual_collision_box_list = simplify_collision_box_list(collision_box_list);
        assert_eq!(actual_collision_box_list, expected_collision_box_list);
    }
}
