mod looking_for_item;

use std::collections::BTreeMap;

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
    pub item_informers: &'a mut [ItemInformer],
    pub price_map: &'a BTreeMap<usize, usize>,
}
impl ShopperTickContext<'_> {
    fn path_find(&self, now: GameXy, xy: GameXy) -> Option<Vec<GameXy>> {
        todo!()
    }

    fn ask_for_item(
        &mut self,
        item_informer_id: usize,
        finding_item_id: usize,
        quantity: usize,
        max_price: usize,
    ) -> AskForItemAnswer {
        let price_map = self.price_map;

        let Some(item_informer) = self.item_informer_mut(item_informer_id) else {
            return AskForItemAnswer::Idk;
        };
        item_informer.ask_for_item(finding_item_id, quantity, max_price, price_map)
    }

    fn item_informer(&mut self, informer_id: usize) -> Option<&ItemInformer> {
        self.item_informers
            .iter()
            .find(|informer| informer.id == informer_id)
    }

    fn item_informer_mut(&mut self, informer_id: usize) -> Option<&mut ItemInformer> {
        self.item_informers
            .iter_mut()
            .find(|informer| informer.id == informer_id)
    }
}

#[derive(Debug)]
pub struct ItemInformer {
    id: usize,
    xy: GameXy,
    kind: ItemInformerKind,
}
impl ItemInformer {
    fn ask_for_item(
        &mut self,
        finding_item_id: usize,
        quantity: usize,
        max_price: usize,
        price_map: &BTreeMap<usize, usize>,
    ) -> AskForItemAnswer {
        match &mut self.kind {
            ItemInformerKind::Staff => {
                /*
                    아이템 구매자가 찾아와 아이템이 어딨는지 물어볼 때, 아이템이 어디에 있는지 아는 스태프, 모르는 스태프 개념을 둘까?
                    스태프는 정확하게 어디에 가라고 해야할까, 아니면 대략 저쯤 가보세요~ 를 해야할까?
                    스태프가 바쁘면 못가르쳐줄 수도 있을 것 같은데, 무조건 가르쳐주게 해야할까?

                    - 스태프의 아이템 지식은 '숙련도'나 '역할'에 따라 달라지게 하세요. (교육/성장 시스템 연동)
                    - 안내 정확도는 기본적으로 '대략적'으로 하되, 숙련도가 높거나 특정 조건 만족 시 '정확하게' 안내하도록 하세요.
                    - 스태프의 안내 가능 여부는 '바쁨' 상태나 다른 업무 여부에 따라 '조건부'로 결정되게 하세요. (스태프 관리/워크플로우 관리 중요성 부각)
                */

                todo!()
            }
            ItemInformerKind::CheckoutCounter => {
                /*
                    재고 시스템에 검색해서 알려주거나
                    창고에 들어가서 확인하고 알려주거나
                */
                todo!()
            }
            ItemInformerKind::DisplayStand { slots } => {
                let Some(&price) = price_map.get(&finding_item_id) else {
                    return AskForItemAnswer::Idk;
                };

                let mut bought_count = 0;
                for slot in slots {
                    let ItemSlot::Item {
                        item,
                        quantity: slot_item_quantity,
                    } = slot
                    else {
                        continue;
                    };
                    if item.id != finding_item_id {
                        continue;
                    }

                    // NOTE: The reason to check the price here is that DisplayStand might not have that item.
                    // So only when buyer finds the item in DisplayStand, the price is checked.
                    // I know it's duplicated over the whole slots, but it's simple operation.
                    if price > max_price {
                        return AskForItemAnswer::Expensive;
                    }

                    let amount_to_pop = (*slot_item_quantity).min(quantity - bought_count);

                    bought_count += amount_to_pop;
                    *slot_item_quantity -= amount_to_pop;

                    if *slot_item_quantity == 0 {
                        *slot = ItemSlot::Empty;
                    }
                }

                if bought_count == 0 {
                    AskForItemAnswer::Idk
                } else {
                    AskForItemAnswer::GotIt {
                        quantity: bought_count,
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
enum ItemInformerKind {
    Staff,
    CheckoutCounter,
    DisplayStand { slots: Vec<ItemSlot> },
}

#[derive(Debug)]
enum ItemSlot {
    Empty,
    Item { item: Item, quantity: usize },
}

#[derive(Debug)]
struct Item {
    id: usize,
}

#[derive(Debug)]
enum AskForItemAnswer {
    Idk,
    GotIt { quantity: usize },
    IDontHaveButKnowWhoHasIt { informer_id: usize },
    LetMeCheck,
    Expensive,
}
