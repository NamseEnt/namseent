`rust-to-js` is Rust to JavaScript transpiler.

This doesn't check Rust's safety. It assumes that the Rust code already checked by `cargo check`.

This changes rust code to `MIR` first, and then change `MIR` to JavaScript.

`rust-to-js` provides glue code to run Rust code in JavaScript.

This tests with `QuickJS`.
