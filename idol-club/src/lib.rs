mod game_state;

use std::{collections::HashMap, num::NonZeroUsize};

use namui::*;
use namui_prebuilt::*;

pub fn main() {
    namui::start(|ctx| {
        ctx.add(App {});
    });
}

struct App {}
impl Component for App {
    fn render(self, ctx: &RenderCtx) {
        let screen_wh = screen::size().into_type::<Px>();
        eprintln!("screen size: {:?}", screen_wh);
    }
}

#[derive(Debug)]
struct GameState {
    customer_spawner: CustomerSpawner,
    customers: Vec<Customer>,
}

#[derive(Debug)]
struct CustomerSpawner {}

#[derive(Debug)]
struct Customer {
    destination: Xy<usize>,
    goods_to_buy: HashMap<GoodsKind, NonZeroUsize>,
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
