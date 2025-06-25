#![allow(dead_code)]

mod app;
mod component;
mod ecs;

pub fn main() {
    namui::start(|ctx: &RenderCtx| {
        ctx.add(app::App::new());
    })
}
