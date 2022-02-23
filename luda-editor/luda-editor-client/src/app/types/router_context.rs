use super::MetaContainer;
use luda_editor_rpc::Socket;
use std::sync::Arc;

pub struct AppContext {
    pub socket: Socket,
    pub meta_container: Arc<MetaContainer>,
}
