mod components;
mod drummer;
mod music;
mod music_play_page;
mod music_select_page;
mod note;
mod play_state;
mod setting_overlay;
mod theme;

use self::{
    music::{load_music_metadata, load_music_speed_map, MusicSpeedMap},
    music_play_page::MusicPlayPage,
    music_select_page::MusicSelectPage,
    play_state::{PlayState, PLAY_STATE_ATOM},
    setting_overlay::{SettingOverlay, SETTING_OVERLAY_OPEN_ATOM},
};
use namui::prelude::*;
use namui_prebuilt::simple_rect;

pub static MUSIC_SPEED_MAP_ATOM: Atom<Option<MusicSpeedMap>> = Atom::uninitialized_new();

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) -> RenderDone {
        let (musics, set_musics) = ctx.state(Vec::new);
        let wh = screen::size().into_type::<Px>();

        let _ = ctx.atom_init(&SETTING_OVERLAY_OPEN_ATOM, || false);
        let (play_state, _) = ctx.atom_init(&PLAY_STATE_ATOM, PlayState::default);
        let (music_speed_map, set_music_speed_map) = ctx.atom_init(&MUSIC_SPEED_MAP_ATOM, || None);

        ctx.effect("load musics", || {
            namui::spawn(async move {
                let musics = load_music_metadata().await;
                set_musics.set(musics);
            });
        });
        ctx.effect("load music speed map", || {
            namui::spawn(async move {
                let music_speed_map = load_music_speed_map().await;
                set_music_speed_map.set(Some(music_speed_map));
            });
        });

        ctx.component(SettingOverlay { wh });

        let play_state_is_idle = matches!(*play_state, PlayState::Idle);
        ctx.compose(|ctx| {
            if !play_state_is_idle {
                return;
            }
            ctx.add(MusicSelectPage {
                wh,
                musics: &musics,
                music_speed_map: (*music_speed_map).as_ref(),
            });
        });

        ctx.compose(|ctx| {
            if play_state_is_idle {
                return;
            }
            ctx.add(MusicPlayPage { wh });
        });

        ctx.component(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::WHITE));

        ctx.done()
    }
}
