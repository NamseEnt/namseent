//! Non-wasm drawer run in same process unlike wasm drawer.

use super::sk_calculate;
use crate::system::InitResult;
use anyhow::{anyhow, Result};
use namui_skia::*;
use namui_type::*;
use std::sync::OnceLock;
use std::{
    ops::DerefMut,
    sync::{Arc, RwLock},
};

static SKIA: OnceLock<Arc<RwLock<NativeSkia>>> = OnceLock::new();

pub(crate) async fn init() -> InitResult {
    let skia = namui_skia::init_skia(
        crate::system::screen::window_id(),
        crate::system::screen::size(),
    )?;
    SKIA.set(Arc::new(RwLock::new(skia)))
        .map_err(|_| anyhow!("Skia is already initialized"))?;

    let (tx, rx) = tokio::sync::mpsc::unbounded_channel();
    DRAW_COMMAND_TX.set(tx).unwrap();

    spawn_drawing_task(rx);

    Ok(())
}

pub(crate) fn on_window_resize(wh: Wh<IntPx>) {
    let mut skia = SKIA.get().unwrap().write().unwrap();
    skia.on_resize(wh);
}

pub(crate) fn render(rendering_tree: RenderingTree) {
    let mut skia = SKIA.get().unwrap().write().unwrap();

    namui_drawer_sys::draw(skia.deref_mut(), rendering_tree);
}

static DRAW_COMMAND_TX: OnceLock<tokio::sync::mpsc::UnboundedSender<DrawingCommand>> =
    OnceLock::new();

#[derive(Debug)]
enum DrawingCommand {
    Draw { rendering_tree: RenderingTree },
    Redraw,
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
                    render(rendering_tree);
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

pub(crate) async fn load_typeface(typeface_name: String, bytes: Vec<u8>) -> Result<()> {
    sk_calculate()
        .load_typeface(typeface_name.to_string(), bytes.to_vec())
        .await??;

    Ok(())
}

pub(crate) async fn load_image_from_url(url: impl AsRef<str>) -> Result<Image> {
    let bytes = crate::system::network::http::get_bytes(url.as_ref()).await?;
    let image = sk_calculate()
        .load_image_from_encoded(bytes.as_ref())
        .await?;

    Ok(image)
}

pub(crate) async fn load_image_from_raw(image_info: ImageInfo, bytes: &[u8]) -> Result<Image> {
    let image = sk_calculate()
        .load_image_from_raw(image_info, bytes)
        .await?;

    Ok(image)
}

/// This function redraw forcibly.
pub(crate) fn redraw() {
    DRAW_COMMAND_TX
        .get()
        .unwrap()
        .send(DrawingCommand::Redraw)
        .unwrap();
}
