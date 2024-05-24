mod components;
mod drummer;
mod music;
mod music_play_page;
mod music_select_page;
mod note;
mod play_state;
mod setting_overlay;
mod theme;

use crate::app::theme::THEME;

use self::{
    music::{
        load_music_best_score_map, load_music_metadata, load_music_speed_map, MusicBestScoreMap,
        MusicSpeedMap,
    },
    music_play_page::MusicPlayPage,
    music_select_page::MusicSelectPage,
    play_state::{PlayState, PLAY_STATE_ATOM},
    setting_overlay::{SettingOverlay, SETTING_OVERLAY_OPEN_ATOM},
};
use namui::*;
use namui_prebuilt::simple_rect;

pub static MUSIC_SPEED_MAP_ATOM: Atom<Option<MusicSpeedMap>> = Atom::uninitialized();
pub static MUSIC_BEST_SCORE_MAP_ATOM: Atom<Option<MusicBestScoreMap>> = Atom::uninitialized();

#[namui::component]
pub struct App {}
impl namui::Component for App {
    fn render(self, ctx: &RenderCtx) {
        let (musics, set_musics) = ctx.state(Vec::new);
        let wh = screen::size().into_type::<Px>();

        let _ = ctx.init_atom(&SETTING_OVERLAY_OPEN_ATOM, || false);
        let (play_state, _) = ctx.init_atom(&PLAY_STATE_ATOM, PlayState::default);
        let (music_speed_map, set_music_speed_map) = ctx.init_atom(&MUSIC_SPEED_MAP_ATOM, || None);
        let (_music_best_score_map, set_music_best_score_map) =
            ctx.init_atom(&MUSIC_BEST_SCORE_MAP_ATOM, || None);

        ctx.effect("load musics", || {
            let set_musics = set_musics.cloned();
            namui::spawn(async move {
                let musics = load_music_metadata().await;
                set_musics.set(musics);
            });
        });
        ctx.effect("load music speed map", || {
            let set_music_speed_map = set_music_speed_map.cloned();
            namui::spawn(async move {
                let music_speed_map = load_music_speed_map().await;
                set_music_speed_map.set(Some(music_speed_map));
            });
        });
        ctx.effect("load music best score map", || {
            let set_music_best_score_map = set_music_best_score_map.cloned();
            namui::spawn(async move {
                let music_best_score_map = load_music_best_score_map().await;
                set_music_best_score_map.set(Some(music_best_score_map));
            });
        });
        ctx.effect("load fonts", || {
            namui::spawn(async move {
                try_join!(async {
                    namui::typeface::register_typeface(
                        THEME.font_name.to_string(),
                        namui::file::bundle::read(
                            "bundle:font/Demo-Hemi Head/Demo_Fonts/Fontspring-Demo-hemi_head_rg.otf",
                        )
                        .await
                        .unwrap(),
                    ).await
                }, async {
                    namui::typeface::register_typeface(
                        THEME.icon_font_name.to_string(),
                        namui::file::bundle::read(
                            "bundle:font/fontawesome-free-5.15.4-desktop/otfs/Font Awesome 5 Free-Solid-900.otf",
                        )
                        .await
                        .unwrap(),
                    ).await
                }).unwrap();
            });
        });

        ctx.add(SettingOverlay { wh });

        let play_state_is_loaded = matches!(*play_state, PlayState::Loaded { .. });
        ctx.compose(|ctx| {
            if play_state_is_loaded {
                return;
            }
            ctx.add(MusicSelectPage {
                wh,
                musics: &musics,
                music_speed_map: (*music_speed_map).as_ref(),
            });
        });

        ctx.compose(|ctx| {
            if !play_state_is_loaded {
                return;
            }
            ctx.add(MusicPlayPage {
                wh,
                music_speed_map: (*music_speed_map).as_ref(),
            });
        });

        ctx.add(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::WHITE));
    }
}
