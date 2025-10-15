use crate::game_state::mutate_game_state;
use crate::*;

#[derive(Clone, Copy, Default, State)]
struct KeyboardNav {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

pub struct CameraController;

impl Component for CameraController {
    fn render(self, ctx: &RenderCtx) {
        let (keyboard_nav, set_keyboard_nav) = ctx.state(KeyboardNav::default);

        ctx.attach_event(move |event| match event {
            Event::KeyDown { event } => match event.code {
                Code::KeyW => set_keyboard_nav.mutate(|nav| nav.up = true),
                Code::KeyS => set_keyboard_nav.mutate(|nav| nav.down = true),
                Code::KeyA => set_keyboard_nav.mutate(|nav| nav.left = true),
                Code::KeyD => set_keyboard_nav.mutate(|nav| nav.right = true),
                _ => {}
            },
            Event::KeyUp { event } => match event.code {
                Code::KeyW => set_keyboard_nav.mutate(|nav| nav.up = false),
                Code::KeyS => set_keyboard_nav.mutate(|nav| nav.down = false),
                Code::KeyA => set_keyboard_nav.mutate(|nav| nav.left = false),
                Code::KeyD => set_keyboard_nav.mutate(|nav| nav.right = false),
                _ => {}
            },
            _ => {}
        });

        ctx.interval("camera move", Duration::from_millis(16), move |real_dt| {
            let nav = *keyboard_nav;
            if nav.up || nav.down || nav.left || nav.right {
                // px/s
                let speed_px_per_sec: f32 = 1024.0;
                // 실제 틱 시간(ms)
                let dt_secs = real_dt.as_millis() as f32 / 1000.0;
                let step = (speed_px_per_sec * dt_secs).max(0.0);
                let mut dir_x: f32 = 0.0;
                let mut dir_y: f32 = 0.0;
                if nav.left {
                    dir_x -= 1.0;
                }
                if nav.right {
                    dir_x += 1.0;
                }
                if nav.up {
                    dir_y -= 1.0;
                }
                if nav.down {
                    dir_y += 1.0;
                }
                let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
                if len > 0.0 {
                    let nx = dir_x / len;
                    let ny = dir_y / len;
                    let dx = (nx * step).px();
                    let dy = (ny * step).px();
                    mutate_game_state(move |game_state| {
                        game_state.camera.move_by(Xy::new(dx, dy));
                    });
                }
            }
        });
    }
}
