mod types;
pub use nrpc::*;
pub use types::*;

def_rpc! {
    get_camera_shot_urls({}) -> {
        camera_shot_urls: Vec<String>,
    },
    read_file({
        dest_path: String,
    }) -> {
        file: Vec<u8>,
    },
    read_dir({
        dest_path: String,
    }) -> {
        directory_entries: Vec<crate::Dirent>
    },
    write_file({
        dest_path: String,
        file: Vec<u8>,
    }) -> {},
}
