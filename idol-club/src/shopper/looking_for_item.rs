use super::*;

/*
Find Target -Found-> Move to Target -> Check Item -> Next State
                ㄴNot Found -> 둘러보기      ㄴ Not Found
                                ↲               ↲
    */

#[derive(Debug)]
pub struct Flow {
    state: State,
    searched_targets: Vec<Target>,
}
impl Flow {
    pub fn new(now: Xy<isize>) -> Self {
        Self {
            state: State::LookingAround {
                moving: Moving::new(vec![now]),
            },
            searched_targets: vec![],
        }
    }

    pub fn tick(&mut self, dt: Duration) -> Event {
        match &mut self.state {
            State::LookingAround { moving } => {
                moving.tick(dt);

                // 시야를 어떻게 정의하고, 그 시야 안에 들어있는 타겟을 찾는 것을 어떻게 계싼할거지?

                if moving.done() {}
            }
            State::GoingToTarget { target } => {}
        }

        Event::None
    }
}

#[derive(Debug)]
enum State {
    LookingAround { moving: Moving },
    GoingToTarget { target: Target },
}

#[derive(Debug)]
pub enum Event {
    None,
}

#[derive(Debug)]
enum Target {
    Staff,
    CheckoutCounter,
    DisplayStand,
}
