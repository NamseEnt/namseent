mod data_fetch;
mod home;
mod new_team_page;
mod router;
mod simple_button;
mod toast;

use data_fetch::*;
use namui::*;
use namui_prebuilt::{table::*, *};
use simple_button::*;

pub async fn main() {
    namui::start(|ctx| {
        if !try_login(ctx) {
            return;
        }

        let screen_wh = namui::screen::size().map(|x| x.into_px());

        ctx.add(toast::Toast);
        ctx.add(router::Router);
        ctx.add(simple_rect(
            screen_wh,
            Color::TRANSPARENT,
            0.px(),
            Color::grayscale_f01(0.8),
        ));
    });
}

fn try_login(ctx: &RenderCtx) -> bool {
    ctx.effect("Insert gsi html api", || {
        let js_handle = namui::wasi::insert_js(include_str!("login.js"), Some(|data: &[u8]| {}));

        move || {
            drop(js_handle);
        }
    });

    todo!()
}
