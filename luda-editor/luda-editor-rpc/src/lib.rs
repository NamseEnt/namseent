mod types;
pub use nrpc::*;
pub use types::*;

def_rpc! {
    get_character_image_urls({}) -> {
        character_image_urls: Vec<String>,
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
    get_sequences({})-> {
        title_sequence_json_tuples: Vec<(String, String)>,
    },
    put_sequences({
        title_sequence_json_tuples: Vec<(String, String)>,
    })-> {},
}
