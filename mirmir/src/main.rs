#![feature(rustc_private)]

use rust_to_js::run;

fn main() {
    run("../tower-defense/src/lib.rs");
}
