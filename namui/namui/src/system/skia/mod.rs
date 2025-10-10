#[cfg(target_os = "wasi")]
mod wasi;
#[cfg(not(target_os = "wasi"))]
mod winit;

#[cfg(target_os = "wasi")]
use wasi as inner;
#[cfg(not(target_os = "wasi"))]
use winit as inner;

use super::InitResult;
use crate::*;
use anyhow::Result;
use namui_skia::*;
use namui_type::*;
use std::sync::*;

static DRAW_COMMAND_TX: OnceLock<std::sync::mpsc::Sender<DrawingCommand>> = OnceLock::new();

#[derive(Debug)]
enum DrawingCommand {
    Draw { rendering_tree: RenderingTree },
    Resize { wh: Wh<IntPx> },
}

pub(super) fn init() -> InitResult {
    inner::init()?;

    Ok(())
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

//     let mut skia = inner::init_skia()?;
//     println!("skia init done!");

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
//             inner::after_draw();
//             rendering_tree_changed = false;
//             resized = false;
//             last_mouse_xy = mouse_xy;
//         }
//     }

//     Ok(())
// }
