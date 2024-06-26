mod build_helper;
#[path = "src/lib.rs"]
#[allow(unused_imports)]
mod rpc;

use rpc::Rpc;

fn main() {
    let rpc = rpc::get_rpc();
    build_helper::server::generate_code(&rpc);
    build_helper::client::generate_code(&rpc);
}
