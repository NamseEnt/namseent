pub use nrpc::*;

def_rpc! {
    get_camera_shot_urls({}) -> {
        camera_shot_urls: Vec<String>,
    },
}
