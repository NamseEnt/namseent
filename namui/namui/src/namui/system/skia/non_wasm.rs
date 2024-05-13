use anyhow::Result;
use namui_skia::*;
use namui_type::*;
use std::{
    ops::DerefMut,
    sync::{Arc, RwLock},
};

static SKIA: OnceLock<Arc<RwLock<dyn SkSkia + Send + Sync>>> = OnceLock::new();

pub(super) async fn init_skia() -> Result<Arc<RwLock<impl SkSkia + Send + Sync>>> {
    namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )
}

pub(crate) fn on_window_resize(wh: Wh<IntPx>) {
    let mut skia = SKIA.get().unwrap().write().unwrap();
    skia.on_resize(wh);
}

pub(crate) fn render(draw_input: DrawInput) {
    let mut skia = SKIA.get().unwrap().write().unwrap();

    namui_drawer_sys::draw(skia.deref_mut(), draw_input, &|image_source| {
        let image_source = image_source.clone();
        tokio::spawn(async move {
            crate::system::image::load_image(&image_source)
                .await
                .unwrap();

            crate::system::skia::redraw();
        });
    });
}
//! Non-wasm drawer run in same process unlike wasm drawer.

use crate::{image::ImageBitmap, system::InitResult, *};
use std::sync::OnceLock;

static DRAW_COMMAND_TX: OnceLock<tokio::sync::mpsc::UnboundedSender<DrawingCommand>> =
    OnceLock::new();

#[derive(Debug)]
enum DrawingCommand {
    Draw { rendering_tree: RenderingTree },
    Redraw,
}

pub(crate) async fn init() -> InitResult {
    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    DRAW_COMMAND_TX.set(tx).unwrap();

    spawn_drawing_task(rx);

    Ok(())
}

fn spawn_drawing_task(mut rx: tokio::sync::mpsc::UnboundedReceiver<DrawingCommand>) {
    tokio::spawn(async move {
        let mut last_rendering_tree = None;
        let mut rendering_tree_changed = false;
        let mut redraw_requested = false;

        while let Some(command) = rx.recv().await {
            let mut on_command = |command| match command {
                DrawingCommand::Draw { rendering_tree } => {
                    if last_rendering_tree.as_ref() != Some(&rendering_tree) {
                        last_rendering_tree = Some(rendering_tree.clone());
                        rendering_tree_changed = true;
                    }
                }
                DrawingCommand::Redraw => {
                    redraw_requested = true;
                }
            };

            on_command(command);
            while let Ok(next_command) = rx.try_recv() {
                on_command(next_command);
            }

            if redraw_requested || rendering_tree_changed {
                if let Some(rendering_tree) = last_rendering_tree.clone() {
                    system::skia::render(DrawInput { rendering_tree });
                    rendering_tree_changed = false;
                    redraw_requested = false;
                }
            }
        }
    });
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    DRAW_COMMAND_TX
        .get()
        .unwrap()
        .send(DrawingCommand::Draw { rendering_tree })
        .unwrap();
}

pub(crate) fn load_typeface(_typeface_name: &str, _bytes: &[u8]) {
    // nothing
}

pub(crate) async fn load_image(
    _image_source: &ImageSource,
    _image_bitmap: ImageBitmap,
) -> ImageInfo {
    // todo!()
}

/// This function redraw forcibly.
pub(crate) fn redraw() {
    DRAW_COMMAND_TX
        .get()
        .unwrap()
        .send(DrawingCommand::Redraw)
        .unwrap();
}
