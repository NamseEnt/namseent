use luda_editor_rpc::Socket;
use namui::Wh;

pub struct AppContext {
    pub screen_size: Wh<f32>,
    pub socket: Socket,
}
