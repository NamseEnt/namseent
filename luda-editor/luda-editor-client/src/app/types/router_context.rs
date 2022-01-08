use luda_editor_rpc::Socket;
use namui::Wh;

pub struct RouterContext {
    pub screen_size: Wh<f32>,
    pub socket: Socket,
}
