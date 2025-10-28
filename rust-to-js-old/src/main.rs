#![feature(rustc_private)]

use std::io::Write;

use rust_to_js::run;

fn main() {
    let rx = run("src/tests/_iter/src/main.rs");
    // let rx = run("../tower-defense/src/lib.rs");

    let prev = std::fs::read_to_string("test.js").unwrap();

    let mut file = std::fs::File::create("test.js").unwrap();
    for line in prev.lines() {
        file.write_all(line.as_bytes()).unwrap();
        file.write_all("\n".as_bytes()).unwrap();

        if line.starts_with("// ====") {
            break;
        }
    }
    while let Ok(line) = rx.recv() {
        file.write_all(line.as_bytes()).unwrap();
    }
}
