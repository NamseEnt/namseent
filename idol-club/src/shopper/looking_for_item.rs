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
    item_id: usize,
    quantity: usize,
    max_price: usize,
}
impl Flow {
    pub fn new(xy: GameXy) -> Self {
        Self {
            state: State::LookingAround,
            searched_item_informer_ids: vec![],
            moving: Moving::new(vec![xy]),
            view_angle: todo!(),
            view_radius: todo!(),
            item_id: todo!(),
            quantity: todo!(),
            max_price: todo!(),
        }
    }

    pub fn tick(&mut self, mut context: ShopperTickContext) -> Event {
        self.moving.tick(context.dt);

        match &self.state {
            State::LookingAround => {
                if self.moving.done() {
                    // choose any point to go around
                }

                for informer in self.informers_in_sight(context.item_informers) {
                    let Some(path) = context.path_find(self.moving.now(), informer.xy) else {
                        continue;
                    };
                    self.state = State::GoingToItemInformer {
                        informer_id: informer.id,
                    };
                    self.moving = Moving::new(path);
                    break;
                }

                Event::None
            }
            State::GoingToItemInformer { informer_id } => {
                if !self.moving.done() {
                    return Event::None;
                }

                let answer =
                    context.ask_for_item(*informer_id, self.item_id, self.quantity, self.max_price);

                match answer {
                    AskForItemAnswer::Idk => todo!(),
                    AskForItemAnswer::GotIt { quantity } => todo!(),
                    AskForItemAnswer::IDontHaveButKnowWhoHasIt { informer_id } => todo!(),
                    AskForItemAnswer::LetMeCheck => todo!(),
                    AskForItemAnswer::Expensive => todo!(),
                }

                todo!()
            }
        }
    }
    fn informers_in_sight<'a>(
        &self,
        informers: &'a [ItemInformer],
    ) -> impl Iterator<Item = &'a ItemInformer> + use<'a> {
        let nowf = self.moving.nowf();
        let mut informers_with_distance = informers
            .into_iter()
            .filter(|informer| {
                self.in_sight(nowf, self.moving.heading_unit_vector(), informer.xy)
                    && !self.searched_item_informer_ids.contains(&informer.id)
            })
            .map(|informer| {
                let to_target_vector = informer.xy.map(|v| v as f32) - nowf;
                (to_target_vector.length_squared(), informer)
            })
            .collect::<Vec<_>>();

        informers_with_distance.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        informers_with_distance
            .into_iter()
            .map(|(_, informer)| informer)
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
    LookingAround,
    GoingToItemInformer { informer_id: usize },
}

#[derive(Debug)]
pub enum Event {
    None,
}
