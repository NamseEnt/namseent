#![feature(rustc_private)]

use rust_to_js::run;

fn main() {
    let output = run("src/tests/_print_variables/main.rs");
    println!("{}", output);
}
