const PORT: u16 = 8080;

use crate::build::{
    build_server::{start_build, StartBuildOption},
    namui_config::get_namui_config,
    web_server::{StartServerOption, WebServer},
};
use namui::build::types::ErrorMessage;
use std::sync::{Arc, RwLock};

pub async fn build<'a>(watch: bool) {
    assert!(
        watch,
        "for now, only watch mode is supported. please use --watch option."
    );

    let bundle = Arc::new(RwLock::new(Vec::<u8>::new()));

    let namui_config = get_namui_config();

    let web_server = Arc::new(
        WebServer::start(StartServerOption {
            port: PORT,
            resource_path: namui_config.resources,
            bundle: bundle.clone(),
            on_connected: || {},
        })
        .await,
    );

    let _ = webbrowser::open(format!("http://localhost:{}", PORT).as_str());
    print_server_address(PORT);

    start_build(StartBuildOption {
        callback: |option| {
            print_build_result(&option.error_messages);
            print_server_address(PORT);
        },
        root_dir: namui_config.root_directory_path,
        bundle: bundle.clone(),
        web_server: web_server.clone(),
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
    print!("{}[2J", 27 as char);
}

fn print_server_address(port: u16) {
    println!("server is running on http://localhost:{}", port);
}
