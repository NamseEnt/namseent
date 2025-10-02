use crate::*;
use anyhow::{Result, anyhow};
use namui_skia::*;
use namui_type::*;
use std::sync::*;

static DRAW_COMMAND_TX: OnceLock<std::sync::mpsc::Sender<DrawingCommand>> = OnceLock::new();

#[derive(Debug)]
enum DrawingCommand {
    Draw { rendering_tree: RenderingTree },
    Resize { wh: Wh<IntPx> },
}

pub async fn load_typeface(typeface_name: String, bytes: Vec<u8>) -> Result<()> {
    NativeCalculate::load_typeface(typeface_name.to_string(), bytes.to_vec()).await??;

    Ok(())
}

pub async fn load_image_from_resource_location(
    resource_location: impl AsRef<ResourceLocation>,
) -> Result<Image> {
    match resource_location.as_ref() {
        ResourceLocation::Bundle(path) => {
            let bytes = crate::file::bundle::read(path).await?;
            load_image_from_bytes(bytes.as_ref()).await
        }
        ResourceLocation::KvStore(key) => {
            let bytes = crate::file::kv_store::get(key)?.ok_or_else(|| anyhow!("Not found"))?;
            load_image_from_bytes(bytes.as_ref()).await
        }
        ResourceLocation::Network(url) => {
            use crate::system::network::http;
            let bytes = http::Request::get(url.to_string())
                .body(())?
                .send()
                .await?
                .ensure_status_code()?
                .bytes()
                .await?;
            load_image_from_bytes(bytes.as_ref()).await
        }
    }
}

async fn load_image_from_bytes(bytes: &[u8]) -> Result<Image> {
    todo!()
    // let image = NativeCalculate::load_image_from_encoded(bytes.as_ref()).await?;

    // Ok(image)
}

pub async fn load_image_from_raw(image_info: ImageInfo, bytes: &[u8]) -> Result<Image> {
    todo!()
    // let image = NativeCalculate::load_image_from_raw(image_info, bytes).await?;

    // Ok(image)
}

pub(crate) fn on_window_resize(wh: Wh<IntPx>) {
    send_command(DrawingCommand::Resize { wh });
}

pub(crate) fn request_draw_rendering_tree(rendering_tree: RenderingTree) {
    send_command(DrawingCommand::Draw { rendering_tree });
}

fn send_command(command: DrawingCommand) {
    let Some(tx) = DRAW_COMMAND_TX.get() else {
        return;
    };
    let _ = tx.send(command);
}

// pub(crate) fn on_skia_drawing_thread() -> Result<()> {
//     let (tx, rx) = std::sync::mpsc::channel();
//     DRAW_COMMAND_TX.set(tx).map_err(|_| unreachable!()).unwrap();

//     let mut last_rendering_tree = None;
//     let mut rendering_tree_changed = false;
//     let mut resized = false;
//     let mut last_mouse_xy = system::mouse::position();

//     while let Ok(command) = rx.recv() {
//         let mut on_command = |command| match command {
//             DrawingCommand::Draw { rendering_tree } => {
//                 if last_rendering_tree.as_ref() != Some(&rendering_tree) {
//                     last_rendering_tree = Some(rendering_tree.clone());
//                     rendering_tree_changed = true;
//                 }
//             }
//             DrawingCommand::Resize { wh } => {
//                 resized = true;
//                 skia.on_resize(wh);
//             }
//         };

//         on_command(command);
//         while let Ok(next_command) = rx.try_recv() {
//             on_command(next_command);
//         }

//         let mouse_xy = system::mouse::position();
//         if (resized || rendering_tree_changed || mouse_xy != last_mouse_xy)
//             && let Some(rendering_tree) = last_rendering_tree.clone()
//         {
//             namui_drawer::draw(
//                 &mut skia,
//                 rendering_tree,
//                 mouse_xy,
//                 mouse::standard_cursor_sprite_set(),
//             );
//             rendering_tree_changed = false;
//             resized = false;
//             last_mouse_xy = mouse_xy;
//         }
//     }

//     Ok(())
// }
