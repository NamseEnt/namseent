mod login;
pub mod notification;

use self::notification::NotificationRoot;
use crate::{components::context_menu::ContextMenu, pages::router::Router};
use anyhow::Result;
use namui::*;
use namui_prebuilt::*;

pub struct App;

#[derive(Debug, PartialEq)]
enum LoadingState {
    Loading,
    Loaded,
    Error(String),
}

impl Component for App {
    fn render(self, ctx: &RenderCtx)  {
        let (loading_state, set_loading_state) = ctx.state(|| LoadingState::Loading);

        ctx.effect("Try login", || {
            spawn_local(async move {
                let result: Result<()> = async move {
                    let session_id = login::get_session_id().await?;
                    crate::RPC.set_session_id(session_id);

                    Ok(())
                }
                .await;
                set_loading_state.mutate(|x| {
                    *x = match result {
                        Ok(_) => LoadingState::Loaded,
                        Err(err) => {
                            crate::log!("Login error: {}", err);
                            LoadingState::Error(err.to_string())
                        }
                    }
                });
            });
        });

        let wh = namui::screen::size();
        let wh = Wh::new(wh.width.into_px(), wh.height.into_px());

        ctx.component(ContextMenu);
        ctx.component(NotificationRoot { wh });
        ctx.compose(|ctx| match &*loading_state {
            LoadingState::Loading => {
                ctx.add(typography::body::center(wh, "Logging in...", Color::WHITE));
            }
            LoadingState::Loaded => {
                ctx.add(Router { wh });
            }
            LoadingState::Error(error) => {
                ctx.add(typography::body::center(wh, error, Color::WHITE));
            }
        });
        ctx.component(simple_rect(wh, Color::TRANSPARENT, 0.px(), Color::BLACK));

        
    }
}
