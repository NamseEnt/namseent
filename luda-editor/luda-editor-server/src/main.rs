use futures::{
    join,
    stream::{self, StreamExt},
    SinkExt,
};
use luda_editor_rpc::{
    self, async_trait::async_trait, response_waiter::ResponseWaiter, Dirent, DirentFileType,
};
use path_clean::PathClean;
use tokio::sync::mpsc::unbounded_channel;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

const RESOURCE_ROOT: &str = "../resources";
fn resource_path() -> std::path::PathBuf {
    std::env::current_dir().unwrap().join(RESOURCE_ROOT).clean()
}
#[tokio::main]
async fn main() {
    let resource_images_route = warp::path("resources").and(warp::fs::dir(resource_path()));

    let web_socket_route = warp::path::end()
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| ws.on_upgrade(move |web_socket| on_connected(web_socket)));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "OPTIONS"]);
    let log = warp::log("luda_editor_rpc");

    let routes = resource_images_route
        .or(web_socket_route)
        .with(cors)
        .with(log);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn on_connected(web_socket: WebSocket) {
    let response_waiter = ResponseWaiter::new();
    let (sink, stream) = web_socket.split();
    let mut sink =
        sink.with_flat_map(|packet: Vec<u8>| stream::iter(vec![Ok(Message::binary(packet))]));

    let (tx, mut rx) = unbounded_channel();
    let tx2 = tx.clone();

    // let mut socket = Socket::new(tx, response_waiter.clone());
    // let socket2 = socket.clone();

    let handler = RpcHandler {};
    let stream = stream.map(|message| {
        message
            .map(|message| message.as_bytes().to_vec())
            .map_err(|e| format!("websocket error: {}", e))
    });

    let loop_sending = async {
        while let Some(data) = rx.recv().await {
            sink.send(data).await.unwrap();
        }
    };

    let _ = join!(
        loop_sending,
        luda_editor_rpc::loop_receiving(tx2, stream, handler, response_waiter)
    );
}

#[derive(Clone)]
pub struct RpcHandler {}

#[async_trait]
impl luda_editor_rpc::RpcHandle for RpcHandler {
    async fn get_camera_shot_urls(
        &mut self,
        _: luda_editor_rpc::get_camera_shot_urls::Request,
    ) -> Result<luda_editor_rpc::get_camera_shot_urls::Response, String> {
        let resource_image_path = resource_path().join("images");
        println!("[get_camera_shot_urls] {:?}", resource_image_path);

        match std::fs::read_dir(resource_image_path) {
            Ok(entries) => {
                let mut camera_shot_urls = Vec::new();

                for entry in entries {
                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name().into_string().unwrap();
                            camera_shot_urls
                                .push(format!("http://localhost:3030/resources/images/{}", name));
                        }
                        Err(e) => {
                            println!("{}", e);
                        }
                    }
                }
                Ok(luda_editor_rpc::get_camera_shot_urls::Response { camera_shot_urls })
            }
            Err(error) => {
                let error_message = format!("get_camera_shot_urls error: {}", error);
                println!("{}", error_message);
                Err(error_message)
            }
        }
    }
    async fn read_file(
        &mut self,
        request: luda_editor_rpc::read_file::Request,
    ) -> Result<luda_editor_rpc::read_file::Response, String> {
        let dest_path = resource_path().join(request.dest_path);
        println!("[read_file] dest_path::{:?}", &dest_path);
        match std::fs::read(dest_path) {
            Ok(file) => Ok(luda_editor_rpc::read_file::Response { file }),
            Err(error) => {
                let error_message = format!("read_file error: {}", error);
                println!("{}", error_message);
                Err(error_message)
            }
        }
    }
    async fn read_dir(
        &mut self,
        request: luda_editor_rpc::read_dir::Request,
    ) -> Result<luda_editor_rpc::read_dir::Response, String> {
        let dest_path = resource_path().join(request.dest_path);
        println!("[read_dir] dest_path::{:?}", &dest_path);
        match std::fs::read_dir(dest_path) {
            Ok(read_dir) => {
                let directory_entries: Vec<Dirent> = read_dir
                    .filter_map(|dirent| match dirent {
                        Ok(dirent) => match dirent.file_type() {
                            Ok(file_type) => Some(Dirent {
                                name: dirent.file_name().to_string_lossy().to_string(),
                                file_type: match file_type.is_dir() {
                                    true => DirentFileType::Directory,
                                    false => DirentFileType::File,
                                },
                            }),
                            Err(error) => {
                                println!("{}", format!("read_dir error: {}", error));
                                None
                            }
                        },
                        Err(error) => {
                            println!("{}", format!("read_dir error: {}", error));
                            None
                        }
                    })
                    .collect();
                Ok(luda_editor_rpc::read_dir::Response { directory_entries })
            }
            Err(error) => {
                let error_message = format!("read_dir error: {}", error);
                println!("{}", error_message);
                Err(error_message)
            }
        }
    }
    async fn write_file(
        &mut self,
        request: luda_editor_rpc::write_file::Request,
    ) -> Result<luda_editor_rpc::write_file::Response, String> {
        let dest_path = resource_path().join(request.dest_path);
        println!("[write_file] dest_path::{:?}", &dest_path);
        match std::fs::write(dest_path, request.file) {
            Ok(_) => Ok(luda_editor_rpc::write_file::Response {}),
            Err(error) => {
                let error_message = format!("write_file error: {}", error);
                println!("{}", error_message);
                Err(error_message)
            }
        }
    }
    async fn get_sequences(
        &mut self,
        _: luda_editor_rpc::get_sequences::Request,
    ) -> Result<luda_editor_rpc::get_sequences::Response, String> {
        let dir_path = resource_path().join("sequence");

        let read_dir = std::fs::read_dir(dir_path);

        if read_dir.is_err() {
            return Err(format!("get_sequences error: {}", read_dir.err().unwrap()));
        };

        let read_dir = read_dir.unwrap();

        let mut title_sequence_json_tuples = Vec::new();

        for dirent in read_dir {
            if dirent.is_err() {
                return Err(format!("get_sequences error: {}", dirent.err().unwrap()));
            };

            let dirent = dirent.unwrap();

            let file_path = dirent.path();
            let file_name_without_extension = file_path.file_stem().unwrap();

            let file = std::fs::read(&file_path);
            if file.is_err() {
                return Err(format!("get_sequences error: {}", file.err().unwrap()));
            };
            let file = file.unwrap();

            let json_string = std::str::from_utf8(&file);
            if json_string.is_err() {
                return Err(format!(
                    "get_sequences error: {}",
                    json_string.err().unwrap()
                ));
            }
            let json_string = json_string.unwrap();
            let title = file_name_without_extension.to_string_lossy().to_string();
            title_sequence_json_tuples.push((title, json_string.to_string()));
        }

        Ok(luda_editor_rpc::get_sequences::Response {
            title_sequence_json_tuples,
        })
    }
    async fn put_sequences(
        &mut self,
        request: luda_editor_rpc::put_sequences::Request,
    ) -> Result<luda_editor_rpc::put_sequences::Response, String> {
        let dir_path = resource_path().join("sequence");

        for (title, sequence_json) in request.title_sequence_json_tuples {
            let path = dir_path.join(format!("{}.json", title));

            let result = std::fs::write(path, sequence_json.as_bytes());
            if result.is_err() {
                return Err(format!("put_sequences error: {}", result.err().unwrap()));
            };
        }
        Ok(luda_editor_rpc::put_sequences::Response {})
    }
}
