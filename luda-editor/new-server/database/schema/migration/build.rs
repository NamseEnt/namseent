use std::path::Path;

#[path = "src/latest_version.rs"]
mod latest_version;

fn main() {
    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = std::fs::read_dir(Path::new(&root).parent().unwrap()).unwrap();

    let mut biggest: Option<usize> = None;
    for entry in dir {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap();
            let Ok(value) = str::parse::<usize>(name) else {
                continue;
            };
            if biggest.is_none() || value > biggest.unwrap() {
                biggest = Some(value);
            }
        }
    }

    let Some(biggest) = biggest else {
        panic!("no version found");
    };

    if latest_version::LATEST_VERSION != biggest {
        panic!(
            "expected latest version to be {}, but found {}",
            biggest,
            latest_version::LATEST_VERSION
        );
    }
}
