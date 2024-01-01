//! Non-wasm drawer run in same process unlike wasm drawer.

use crate::{image::ImageBitmap, system::InitResult, *};
use std::sync::{Mutex, OnceLock};

static RENDERING_TREE_TX: OnceLock<std::sync::mpsc::Sender<RenderingTree>> = OnceLock::new();

pub(crate) async fn init() -> InitResult {
    let (tx, rx) = std::sync::mpsc::channel();
    RENDERING_TREE_TX.set(tx).unwrap();

    spawn_drawing_thread(rx);

    Ok(())
}

fn spawn_drawing_thread(rx: std::sync::mpsc::Receiver<RenderingTree>) {
    std::thread::spawn(move || {
        while let Ok(mut rendering_tree) = rx.recv() {
            rx.try_iter().for_each(|next_rendering_tree| {
                rendering_tree = next_rendering_tree;
            });

            system::skia::render(DrawInput { rendering_tree });
        }
    });
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: namui_type::RenderingTree) {
    {
        static LAST_RENDERING_TREE_TX: Mutex<Option<RenderingTree>> = Mutex::new(None);
        let mut last_rendering_tree = LAST_RENDERING_TREE_TX.lock().unwrap();
        if last_rendering_tree.as_ref() == Some(&rendering_tree) {
            return;
        }
        *last_rendering_tree = Some(rendering_tree.clone());
    }

    RENDERING_TREE_TX
        .get()
        .unwrap()
        .send(rendering_tree)
        .unwrap();
}

pub(crate) fn load_typeface(_typeface_name: &str, _bytes: &[u8]) {
    // nothing
}

pub(crate) fn load_image(_image_source: &ImageSource, _image_bitmap: ImageBitmap) {
    // nothing. already loaded
}

pub(crate) fn redraw() {
    // NOTE: We may not need this because we redraw by polling on winit event loop.
}
