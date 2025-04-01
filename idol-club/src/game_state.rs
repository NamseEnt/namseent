use crate::*;

pub static GAME_STATE_ATOM: Atom<GameState> = Atom::uninitialized();

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, || GameState {
        customer_spawner: CustomerSpawner::new(GameXy::new(10, 10), 5.sec()),
        customers: vec![],
        map_blocks: vec![vec![false; 20]; 20],
    })
    .0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}

#[derive(Debug)]
pub struct GameState {
    customer_spawner: CustomerSpawner,
    customers: Vec<Customer>,
    map_blocks: Vec<Vec<bool>>,
}

impl GameState {
    pub fn on_update_tick(&mut self, dt: Duration) {
        let mut game_events = vec![];

        self.customer_spawner.tick(&mut self.customers);

        self.customers.iter_mut().for_each(|customer| {
            customer.tick(CustomerTickContext {
                dt,
                map_blocks: &self.map_blocks,
                on_event: &mut |event| game_events.push(event),
            })
        });

        for event in game_events {
            match event {
                GameEvent::CustomerGoneHome { customer_id } => {
                    let Some(index) = self.customers.iter().position(|c| c.id == customer_id)
                    else {
                        continue;
                    };
                    self.customers.swap_remove(index);
                }
            }
        }
    }

    pub fn flush_checkout_counter(&mut self) {
        self.customers.iter_mut().for_each(|customer| {
            customer.go_home_if_on_checkout_counter();
        });
    }
}

impl Component for &GameState {
    fn render(self, ctx: &RenderCtx) {
        let tile_size = 48.px();

        self.customers.iter().for_each(|customer| {
            ctx.add_with_key(customer.id, customer);
        });
    }
}

pub enum GameEvent {
    CustomerGoneHome { customer_id: usize },
}
