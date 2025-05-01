mod game_state;

use game_state::*;
use namui::{rand::seq::SliceRandom, *};
use namui_prebuilt::*;
use std::collections::BTreeMap;

pub fn main() {
    namui::start(|ctx| {
        let (game_state, set_game_state) = ctx.init_atom(&GAME_STATE_ATOM, GameState::new);

        ctx.on_raw_event(|event| {
            let event = event.clone();
            set_game_state.mutate(move |game_state| {
                game_state.on_namui_event(event);
            });
        });

        ctx.interval("tick", (1. / 60.).sec(), |_dt| {
            set_game_state.mutate(|game_state| {
                game_state.tick();
            });
        });

        ctx.add(game_state.as_ref());

        ctx.add(simple_rect(
            namui::screen::size().map(|v| v.into_px()),
            Color::TRANSPARENT,
            0.px(),
            Color::BLACK,
        ));
    });
}
