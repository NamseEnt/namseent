use super::*;

/*
Find Target -Found-> Move to Target -> Check Item -> Next State
                ㄴNot Found -> 둘러보기      ㄴ Not Found
                                ↲               ↲
    */

#[derive(Debug)]
pub struct Flow {
    state: State,
    searched_item_informer_ids: Vec<usize>,
    view_angle: Angle,
    view_radius: f32,
    moving: Moving,
}
impl Flow {
    pub fn new(now: Xy<isize>) -> Self {
        Self {
            state: State::LookingAround {
                moving: Moving::new(vec![now]),
            },
            searched_item_informer_ids: vec![],
        }
    }

    pub fn tick(&mut self, context: ShopperTickContext) -> Event {
        self.moving.tick(context.dt);

        match &self.state {
            State::LookingAround { moving } => {
                if moving.done() {
                    // choose any point to go around
                }

                let nowf = moving.nowf();

                let closest_informer_in_sight = context
                    .item_informers
                    .iter()
                    .filter(|informer| {
                        self.in_sight(nowf, moving.heading_unit_vector(), informer.xy)
                            && !self.searched_item_informer_ids.contains(&informer.id)
                    })
                    .map(|informer| {
                        let to_target_vector = informer.xy.map(|v| v as f32) - nowf;
                        (to_target_vector.length_squared(), informer)
                    })
                    .reduce(|a, b| if a.0 < b.0 { a } else { b })
                    .map(|(_, informer)| informer);

                if let Some(closest_informer) = closest_informer_in_sight {
                    self.state = State::GoingToItemInformer {
                        informer_id: closest_informer.id,
                    };
                    self.moving = todo!(); // bfs
                }
                Event::None
            }
            State::GoingToItemInformer { informer_id } => {
                if !self.moving.done() {
                    return Event::None;
                }

                todo!()
            }
        }
    }
    fn in_sight(&self, my_xy: GameXyF, heading_unit_vector: Xy<f32>, target_xy: GameXy) -> bool {
        let target_xy = target_xy.map(|v| v as f32);
        let to_target_vector = target_xy - my_xy;
        let to_target_length = to_target_vector.length();

        if to_target_length > self.view_radius {
            return false;
        }

        let dot_product = heading_unit_vector.dot(&to_target_vector);
        let cos_theta = dot_product / to_target_length;
        let theta = cos_theta.acos();
        let view_angle = self.view_angle.as_radians();
        theta <= view_angle
    }
}

#[derive(Debug)]
enum State {
    LookingAround { moving: Moving },
    GoingToItemInformer { informer_id: usize },
}

#[derive(Debug)]
pub enum Event {
    None,
}
