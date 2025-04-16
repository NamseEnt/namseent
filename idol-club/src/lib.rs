mod customer;
mod game_state;
mod ground_grid;
mod moving;
mod shopper;

use customer::*;
use game_state::*;
use moving::*;
use namui::*;
use namui_prebuilt::*;
use std::{collections::HashMap, num::NonZeroUsize};

type GameCoord = usize;
type GameCoordF = f32;
type GameCoordS = isize;
type GameXy = Xy<GameCoord>;
type GameXyF = Xy<GameCoordF>;
type GameXyS = Xy<GameCoordS>;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(App {});
    });
}

struct App {}
impl Component for App {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        let game_state = use_game_state(ctx);

        ctx.interval("game tick", 33.ms(), |dt| {
            mutate_game_state(move |game_state| {
                game_state.on_update_tick(dt);
            });
        });

        ctx.attach_event(|event| {
            if let Event::KeyUp { event } = event {
                if event.code == Code::Space {
                    mutate_game_state(|game_state| {
                        game_state.flush_checkout_counter();
                    });
                }
            }
        });

        ctx.add(game_state.as_ref());

        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK,
        ));

        // # 목표
        // 손님을 스폰해서 카운터에 오게 하는 것.
        /*
        5초마다 손님을 스폰하자.
        손님은 카운터 앞에 줄을 선다.
            다양한 방법으로 줄 세우고 싶은데... 그거는 나중에 생각하자.
        여기까지 하면 돼!

        손님은 카운터가 어디인지 모르고, 줄이 어떻게 세워져있는진 몰라!
        손님은 일단 가게를 오는게 목적이야.
        그리고 난 다음에 줄이 어디인지 탐색해

        주변 사람한테 물어볼 수도 있고, 근데 주변 사람한테 물어보기 부끄러울 수 있어.
        표지판을 보고 찾아올 수도 있고, 근데 표지판을 못볼 수도 있어. 시야 안에 표지판이 확실하게 들어오면 되겠지.

        뭔갈 사려는 목적을 가진 손님이 있어.
        그런 목적이 아닌 손님도 있겠지. 아무튼 손님은 목적이 있고, 목적 중에는 뭔가를 사려는게 목적인 경우가 있어.

        물건을 사려는 목적인 경우
        일단 가게에 와.
        물건이 있는지 찾아보거나 직원에게 물어봐. 근데 직원에게는 잘 물어보지 않을거야.

        거대한 상태기계를 만들 수 있을 것 같아.

        <살게 있는 손님>
        가게를 찾아가는 중 -가게에 도착-> 살려는 물건이 어디에 있는지 찾는 중 -살려는 물건을 찾음 + 기대한 가격 이하임-> 구매하는 중 -구매 완료-> 집으로 돌아감
                                                                              ㄴ아니라면-> 집으로 돌아감
        */
    }
}

#[derive(Debug)]
enum GoodsKind {}

#[derive(Debug)]
struct DisplayStand {
    capacity: usize,
}

#[derive(Debug)]
struct StorageRack {
    capacity: usize,
}

#[derive(Debug)]
struct CheckoutCounter {
    cash_box: CashBox,
    service_spot_customer_side: Xy<usize>,
    service_spot_staff_side: Xy<usize>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CashUnit {
    Coin1,
    Coin5,
    Coin10,
    Coin50,
    Coin100,
    Coin500,
    Bill1000,
    Bill5000,
    Bill10000,
    Bill50000,
}

type CashBox = HashMap<CashUnit, usize>;
