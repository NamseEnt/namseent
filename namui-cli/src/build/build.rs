const PORT: u16 = 8080;

use crate::build::{
    build_server::{start_build, StartBuildOption},
    bundle::Bundle,
    web_server::{StartServerOption, WebServer},
};
use cargo_metadata::MetadataCommand;
use namui::build::types::ErrorMessage;
use std::{path::PathBuf, sync::Arc};
use tokio::sync::RwLock;

pub async fn build(target_dir: String, watch: bool) {
    assert!(watch, "for now, only watch mode is supported. please use --watch option.");
    let root_dir = get_root_dir(&target_dir);
    let bundle = Arc::new(RwLock::new(Bundle::new()));

    let web_server = Arc::new(
        WebServer::start(StartServerOption {
            port: PORT,
            bundle: bundle.clone(),
            on_connected: || {},
        })
        .await,
    );

    let _ = webbrowser::open(format!("http://localhost:{}", PORT).as_str());
    print_server_address(PORT);

    let watch_dir = PathBuf::from(&root_dir)
        .join("./src")
        .to_string_lossy()
        .to_string();
    let manifest_path = PathBuf::from(&root_dir)
        .join("./Cargo.toml")
        .to_string_lossy()
        .to_string();
    start_build(StartBuildOption {
        callback: |option| {
            print_build_result(&option.error_messages);
            print_server_address(PORT);
        },
        watch_dir,
        bundle: bundle.clone(),
        web_server: web_server.clone(),
        manifest_path,
    })
    .await;
}

fn print_build_result(error_messages: &Vec<ErrorMessage>) {
    clear_console();
    if error_messages.is_empty() {
        println!("No errors");
        return;
    }
    println!("Errors {}", error_messages.len());
    for error_message in error_messages {
        println!(
            "{}\n\t--> {}:{}:{}\n",
            error_message.text,
            error_message.absolute_file,
            error_message.line,
            error_message.column
        );
    }
}

fn clear_console() {
    // print!("{}[2J", 27 as char);
}

fn print_server_address(port: u16) {
    println!("server is running on http://localhost:{}", port);
}

fn get_root_dir(crate_root: &str) -> PathBuf {
    let mut manifest_path = PathBuf::from(
        &(MetadataCommand::new()
            .current_dir(crate_root)
            .exec()
            .unwrap()
            .root_package()
            .expect(format!("Could not found root crate from {}", crate_root).as_str())
            .manifest_path),
    );
    manifest_path.pop();
    manifest_path
}
