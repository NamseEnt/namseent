use crate::*;

#[derive(Debug)]
enum CustomerState {
    JustSpawned,
    GoingToStore,
    GoingToCheckoutCounter,
    BuyingItem,
    GoingHome,
}

pub struct CustomerTickContext<'a> {
    pub dt: Duration,
    pub map_blocks: &'a [Vec<bool>],
    pub on_event: &'a mut dyn FnMut(GameEvent),
}

#[derive(Debug)]
pub struct Customer {
    pub id: usize,
    moving: Moving,
    state: CustomerState,
    goods_to_buy: HashMap<GoodsKind, NonZeroUsize>,
}
// TODO: if path blocked, wait seconds and retry or find another path or be angry.
impl Customer {
    pub fn tick(&mut self, ctx: CustomerTickContext) {
        match &mut self.state {
            CustomerState::JustSpawned => {
                let store_entrance_xy = GameXy::new(5, 5);
                let Some(path) =
                    namui::algorithm::bfs(ctx.map_blocks, self.moving.now(), store_entrance_xy)
                else {
                    // TODO: wait seconds and retry.
                    return;
                };
                self.state = CustomerState::GoingToStore;
                self.moving = Moving::new(path);
            }
            CustomerState::GoingToStore => {
                self.moving.tick(ctx.dt);
                if !self.moving.done() {
                    return;
                }
                let checkout_counter_xy = GameXy::new(3, 5);
                let Some(path) =
                    namui::algorithm::bfs(ctx.map_blocks, self.moving.now(), checkout_counter_xy)
                else {
                    // TODO: wait seconds and retry.
                    return;
                };
                self.state = CustomerState::GoingToCheckoutCounter;
                self.moving = Moving::new(path);
            }
            CustomerState::GoingToCheckoutCounter => {
                self.moving.tick(ctx.dt);
                if !self.moving.done() {
                    return;
                }
                self.state = CustomerState::BuyingItem;
            }
            CustomerState::BuyingItem => {}
            CustomerState::GoingHome => {
                // TODO: Find path to home.
                self.moving.tick(ctx.dt);
                if self.moving.done() {
                    (ctx.on_event)(GameEvent::CustomerGoneHome {
                        customer_id: self.id,
                    });
                }
            }
        }
    }

    pub fn go_home_if_on_checkout_counter(&mut self) {
        if let CustomerState::BuyingItem = self.state {
            self.state = CustomerState::GoingHome;
        }
    }
}

impl Component for &Customer {
    fn render(self, ctx: &RenderCtx) {
        let tile_size = 48.px();
        ctx.translate(self.moving.nowf().map(|v| tile_size * v))
            .add(simple_rect(
                Wh::single(tile_size),
                Color::WHITE,
                1.px(),
                Color::GREEN,
            ));
    }
}

#[derive(Debug)]
pub struct CustomerSpawner {
    interval: Duration,
    last_spawn_at: Instant,
    spawn_xy: GameXy,
}

impl CustomerSpawner {
    pub fn tick(&mut self, customers: &mut Vec<Customer>) {
        if time::now() < self.last_spawn_at + self.interval {
            return;
        }

        self.last_spawn_at = time::now();
        customers.push(Customer {
            id: {
                use std::sync::atomic::*;
                static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
                NEXT_ID.fetch_add(1, Ordering::Relaxed)
            },
            moving: Moving::new(vec![self.spawn_xy]),
            state: CustomerState::JustSpawned,
            goods_to_buy: HashMap::new(),
        });
    }

    pub fn new(spawn_xy: GameXy, interval: Duration) -> Self {
        Self {
            interval,
            last_spawn_at: time::now(),
            spawn_xy,
        }
    }
}
