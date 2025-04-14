mod looking_for_item;
mod moving;

use crate::*;
use moving::Moving;

// <살게 있는 손님>
// 가게를 찾아가는 중 -가게에 도착-> 살려는 물건이 어디에 있는지 찾는 중 -살려는 물건을 찾음 + 기대한 가격 이하임-> 구매하는 중 -구매 완료-> 집으로 돌아감
//                                                                       ㄴ아니라면-> 집으로 돌아감

#[derive(Debug)]
pub struct Shopper {
    state: State,
    xy: GameXy,
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
    pub fn tick(&mut self, context: ShopperTickContext) {
        match &mut self.state {
            State::GoingToStore { moving } => {
                moving.tick(context.dt);
                if moving.done() {
                    self.state = State::LookingForItem {
                        flow: looking_for_item::Flow::new(moving.now()),
                    };
                }
            }
            State::LookingForItem { flow } => match flow.tick(context) {
                looking_for_item::Event::None => {}
            },
            State::BuyingItem => todo!(),
            State::GoingHome => todo!(),
        }
    }
}

pub struct ShopperTickContext<'a> {
    pub dt: Duration,
    pub item_informers: &'a [ItemInformer],
}

#[derive(Debug)]
pub struct ItemInformer {
    id: usize,
    xy: GameXy,
    kind: ItemInformerKind,
}

#[derive(Debug)]
enum ItemInformerKind {
    Staff,
    CheckoutCounter,
    DisplayStand,
}
