mod login;

use crate::pages::router::Router;
use anyhow::Result;
use namui::prelude::*;
use namui_prebuilt::*;

#[namui::component]
pub struct App;

#[derive(Debug, PartialEq)]
enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

impl Component for App {
    fn render<'a>(&'a self, ctx: RenderCtx<'a>) -> RenderDone {
        let (loading_state, set_loading_state) = ctx.use_state(|| LoadingState::Loading);

        ctx.use_effect("Try login", || {
            namui::log!("before spawn_local");
            spawn_local(async move {
                let result: Result<()> = async move {
                    namui::log!("before login::get_session_id().await?;");
                    let session_id = login::get_session_id().await?;
                    namui::log!("after login::get_session_id().await?;");
                    crate::RPC.set_session_id(session_id);

                    Ok(())
                }
                .await;
                set_loading_state.mutate(|x| {
                    *x = match result {
                        Ok(_) => LoadingState::Loaded,
                        Err(err) => LoadingState::Error(err.to_string()),
                    }
                });
            });
        });

        let wh = namui::screen::size();

        ctx.add(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK));
        match &*loading_state {
            LoadingState::Loading => {
                ctx.add(typography::body::center(wh, "Logging in...", Color::WHITE))
            }
            LoadingState::Loaded => ctx.add(Router { wh }),
            LoadingState::Error(error) => {
                ctx.add(typography::body::center(wh, &error, Color::WHITE))
            }
        };

        ctx.done()
    }
}
