#[cfg(feature = "cli_debug")]
#[macro_export]
macro_rules! debug_println {
    ($( $args:expr ),*) => { println!( $( $args ),* ); }
}

// Non-debug version
#[cfg(not(feature = "cli_debug"))]
#[macro_export]
macro_rules! debug_println {
    ($( $args:expr ),*) => {};
}
