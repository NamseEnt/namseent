use super::*;
use namui_skia::*;

static DRAW_COMMAND_TX: OnceLock<std::sync::mpsc::Sender<DrawingCommand>> = OnceLock::new();

pub(crate) fn init() -> Result<()> {
    while DRAW_COMMAND_TX.get().is_none() {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }

    Ok(())
}

extern "C" {
    fn take_bitmap(width: i32, height: i32);
}
pub(crate) fn take_main_thread() -> Result<()> {
    let (tx, rx) = std::sync::mpsc::channel();
    DRAW_COMMAND_TX.set(tx).map_err(|_| unreachable!()).unwrap();

    let screen_size = crate::system::screen::size();
    println!("screen size: {:?}", screen_size);
    let mut skia = namui_skia::init_skia(crate::system::screen::size())?;
    println!("skia init done!");

    let mut last_rendering_tree = None;
    let mut rendering_tree_changed = false;
    let mut should_redraw = false;

    while let Ok(command) = rx.recv() {
        let mut on_command = |command| match command {
            DrawingCommand::Draw { rendering_tree } => {
                if last_rendering_tree.as_ref() != Some(&rendering_tree) {
                    last_rendering_tree = Some(rendering_tree.clone());
                    rendering_tree_changed = true;
                }
            }
            DrawingCommand::Resize { wh } => {
                should_redraw = true;
                skia.on_resize(wh);
            }
        };

        on_command(command);
        while let Ok(next_command) = rx.try_recv() {
            on_command(next_command);
        }

        if should_redraw || rendering_tree_changed {
            if let Some(rendering_tree) = last_rendering_tree.clone() {
                namui_drawer::draw(&mut skia, rendering_tree);
                unsafe { take_bitmap(screen_size.width.as_i32(), screen_size.height.as_i32()) };
                rendering_tree_changed = false;
                should_redraw = false;
            }
        }
    }

    Ok(())
}

pub(super) fn send_command(command: DrawingCommand) {
    let tx = DRAW_COMMAND_TX.get().unwrap();
    let _ = tx.send(command);
}
