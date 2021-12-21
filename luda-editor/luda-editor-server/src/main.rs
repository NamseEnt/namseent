use futures::{
    join,
    stream::{self, StreamExt},
    SinkExt,
};
use luda_editor_rpc::{self, async_trait::async_trait, response_waiter::ResponseWaiter};
use tokio::sync::mpsc::unbounded_channel;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};
#[tokio::main]
async fn main() {
    let routes = warp::path::end()
        // The `ws()` filter will prepare the Websocket handshake.
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            // And then our closure will be called when it completes...
            ws.on_upgrade(move |web_socket| on_connected(web_socket))
        });

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
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
            .map(|message| {
                message
                    .as_bytes()
                    .to_vec()
            })
            .map_err(|e| format!("websocket error: {}", e))
    });

    let loop_sending = async {
        while let Some(data) = rx
            .recv()
            .await
        {
            println!("sending: {:?}", data);
            sink.send(data)
                .await
                .unwrap();
        }
    };

    let _ =
        join!(loop_sending, luda_editor_rpc::loop_receiving(tx2, stream, handler, response_waiter));
}

#[derive(Clone)]
pub struct RpcHandler {}

#[async_trait]
impl luda_editor_rpc::RpcHandle for RpcHandler {
    async fn ls(
        &mut self,
        request: luda_editor_rpc::ls::Request,
    ) -> Result<luda_editor_rpc::ls::Response, String> {
        println!("ls: {}", request.path);
        Ok(luda_editor_rpc::ls::Response {
            directory_entries: vec![],
        })
    }
}
