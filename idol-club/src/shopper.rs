use crate::*;

// <살게 있는 손님>
// 가게를 찾아가는 중 -가게에 도착-> 살려는 물건이 어디에 있는지 찾는 중 -살려는 물건을 찾음 + 기대한 가격 이하임-> 구매하는 중 -구매 완료-> 집으로 돌아감
//                                                                       ㄴ아니라면-> 집으로 돌아감

#[derive(Debug)]
pub struct Shopper {
    state: State,
    xy: GameXy,
}

#[derive(Debug, Clone, Copy)]
enum State {
    GoingToStore,
    LookingForItem,
    BuyingItem,
    GoingHome,
}

impl Shopper {
    pub fn tick(&mut self, dt: Duration) {
        match self.state {
            State::GoingToStore => todo!(),
            State::LookingForItem => todo!(),
            State::BuyingItem => todo!(),
            State::GoingHome => todo!(),
        }
    }
}
