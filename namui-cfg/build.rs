use std::env;

static NAMUI_CFG_ENV_PREFIX: &str = "NAMUI_CFG_";
static DEFAULT_NAMUI_CFG_KEYS: [&str; 3] = [
    "NAMUI_CFG_TARGET_ENV",
    "NAMUI_CFG_TARGET_ARCH",
    "NAMUI_CFG_TARGET_PLATFORM",
];

fn main() {
    for key in DEFAULT_NAMUI_CFG_KEYS {
        println!("cargo:rerun-if-env-changed={}", key);
    }

    for (key, _value) in env::vars() {
        match key.starts_with(NAMUI_CFG_ENV_PREFIX) {
            true => {
                println!("cargo:rerun-if-env-changed={}", key);
            }
            false => continue,
        }
    }
}
