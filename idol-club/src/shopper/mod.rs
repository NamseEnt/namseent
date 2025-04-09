mod looking_for_item;

use crate::*;

// <살게 있는 손님>
// 가게를 찾아가는 중 -가게에 도착-> 살려는 물건이 어디에 있는지 찾는 중 -살려는 물건을 찾음 + 기대한 가격 이하임-> 구매하는 중 -구매 완료-> 집으로 돌아감
//                                                                       ㄴ아니라면-> 집으로 돌아감

#[derive(Debug)]
pub struct Shopper {
    state: State,
    xy: GameXy,
}

#[derive(Debug)]
struct Moving {
    path: Vec<GameXy>,
    /// 0.0 ~ path.len()
    path_progress: f32,
}
impl Moving {
    fn tick(&mut self, dt: Duration) {
        self.path_progress += dt.as_secs_f32();
        if self.path_progress >= self.path.len() as f32 {
            self.path_progress = self.path.len() as f32;
        }
    }

    fn done(&self) -> bool {
        self.path_progress >= self.path.len() as f32
    }

    fn new(path: Vec<GameXy>) -> Self {
        Self {
            path,
            path_progress: 0.0,
        }
    }

    fn now(&self) -> GameXy {
        self.path[self.path_progress as usize]
    }
}

struct ShopperFlow {
    state: State,
}

#[derive(Debug)]
enum State {
    GoingToStore { moving: Moving },
    LookingForItem { flow: looking_for_item::Flow },
    BuyingItem,
    GoingHome,
}

impl Shopper {
    pub fn tick(&mut self, dt: Duration) {
        match &mut self.state {
            State::GoingToStore { moving } => {
                moving.tick(dt);
                if moving.done() {
                    self.state = State::LookingForItem {
                        flow: looking_for_item::Flow::new(moving.now()),
                    };
                }
            }
            State::LookingForItem { flow } => match flow.tick(dt) {
                looking_for_item::Event::None => {}
            },
            State::BuyingItem => todo!(),
            State::GoingHome => todo!(),
        }
    }
}
