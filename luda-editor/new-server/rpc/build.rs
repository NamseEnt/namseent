mod build_helper;
#[path = "src/lib.rs"]
#[allow(unused_imports, dead_code)]
mod rpc;

use macro_common_lib::*;
use rpc::Rpc;

fn main() {
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=build_helper");

    let rpc = rpc::get_rpc();
    build_helper::server::generate_code(&rpc);
    build_helper::client::generate_code(&rpc);
}
