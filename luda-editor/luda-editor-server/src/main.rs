use futures::{
    join,
    stream::{self, StreamExt},
    SinkExt,
};
use luda_editor_rpc::{self, async_trait::async_trait, response_waiter::ResponseWaiter};
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

    let cors = warp::cors().allow_any_origin().allow_methods(vec!["GET", "OPTIONS"]);
    let log = warp::log("luda_editor_rpc");

    let routes = resource_images_route.or(web_socket_route).with(cors).with(log);

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

    let _ =
        join!(loop_sending, luda_editor_rpc::loop_receiving(tx2, stream, handler, response_waiter));
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
        println!("{:?}", resource_image_path);

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
                Ok(luda_editor_rpc::get_camera_shot_urls::Response {
                    camera_shot_urls,
                })
            }
            Err(error) => {
                let error_message = format!("get_camera_shot_urls error: {}", error);
                println!("{}", error_message);
                Err(error_message)
            }
        }
    }
}
