use crate::cli::Target;
use std::error::Error;

pub fn test(target: &Target) -> Result<(), Box<dyn Error>> {
    if cfg!(target_os = "linux") {
        use super::linux;
        match target {
            Target::WasmUnknownWeb => linux::wasm_unknown_web::test(),
        }
    } else {
        Result::Err(format!("{} is unsupported os", std::env::consts::OS).into())
    }
}
