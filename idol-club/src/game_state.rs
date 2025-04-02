use crate::*;

#[derive(Debug)]
pub struct GameState {}

pub static GAME_STATE_ATOM: Atom<GameState> = Atom::uninitialized();

pub fn use_game_state<'a>(ctx: &'a RenderCtx) -> Sig<'a, GameState> {
    ctx.init_atom(&GAME_STATE_ATOM, || GameState {}).0
}

pub fn mutate_game_state(f: impl FnOnce(&mut GameState) + Send + Sync + 'static) {
    GAME_STATE_ATOM.mutate(f);
}
