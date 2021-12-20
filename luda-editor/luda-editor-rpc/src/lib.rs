pub use nrpc::*;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct DirectoryEntry {}

def_rpc! {
    ls({ path: String, }) -> {
        directory_entries: Vec<super::DirectoryEntry>,
    },
}
