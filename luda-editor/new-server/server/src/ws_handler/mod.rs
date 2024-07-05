mod handle;

use crate::*;
use anyhow::anyhow;
use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use futures::{stream::StreamExt, SinkExt};

pub async fn ws_handler(ws: WebSocketUpgrade, State(db): State<Database>) -> Response {
    // TODO: Multiplexing. Don't wait the previous request to finish before starting the next one.
    // But sometime it should be waited. Make a rule for that.
    ws.on_upgrade(|socket| async move {
        use axum::extract::ws;
        println!("New connection");

        let session = Session::new();
        const MAX_CONCURRENT_REQUESTS: usize = 16;
        let (in_msg_tx, mut in_msg_rx) =
            tokio::sync::mpsc::channel::<InMessage>(MAX_CONCURRENT_REQUESTS);
        let (out_msg_tx, mut out_msg_rx) =
            tokio::sync::mpsc::channel::<OutMessage>(MAX_CONCURRENT_REQUESTS);

        let (mut sender, mut receiver) = socket.split();

        let in_msg_recv_task = tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                let Ok(msg) = msg else {
                    // client disconnected
                    return;
                };

                let message = match msg {
                    ws::Message::Binary(buffer) => InMessage::Binary(buffer),
                    ws::Message::Ping(buffer) => InMessage::Ping(buffer),
                    ws::Message::Text(_) | ws::Message::Pong(_) | ws::Message::Close(_) => return,
                };

                if in_msg_tx.send(message).await.is_err() {
                    return;
                };
            }
        });
        let msg_process_task = tokio::spawn(async move {
            let mut join_set = tokio::task::JoinSet::new();
            while let Some(msg) = in_msg_rx.recv().await {
                join_set.spawn(handle_msg(
                    msg,
                    out_msg_tx.clone(),
                    db.clone(),
                    session.clone(),
                ));
                if join_set.len() >= MAX_CONCURRENT_REQUESTS
                    && join_set.join_next().await.unwrap().is_err()
                {
                    return;
                }
            }
        });
        let out_msg_send_task = tokio::spawn(async move {
            loop {
                let mut msgs = Vec::with_capacity(MAX_CONCURRENT_REQUESTS);
                out_msg_rx
                    .recv_many(&mut msgs, MAX_CONCURRENT_REQUESTS)
                    .await;
                if msgs.is_empty() {
                    return;
                }

                for msg in msgs {
                    let ws_message = match msg {
                        OutMessage::Binary(buffer) => ws::Message::Binary(buffer),
                        OutMessage::Pong(buffer) => ws::Message::Pong(buffer),
                    };
                    if sender.feed(ws_message).await.is_err() {
                        return;
                    }
                }
                if sender.flush().await.is_err() {
                    return;
                }
            }
        });

        let _ = tokio::join!(in_msg_recv_task, msg_process_task, out_msg_send_task);

        println!("Connection closed");
    })
}

enum InMessage {
    Binary(Vec<u8>),
    Ping(Vec<u8>),
}
enum OutMessage {
    Binary(Vec<u8>),
    Pong(Vec<u8>),
}

async fn handle_msg(
    msg: InMessage,
    out_msg_tx: tokio::sync::mpsc::Sender<OutMessage>,
    db: Database,
    session: Session,
) -> Result<()> {
    let in_packet = match msg {
        InMessage::Binary(buffer) => buffer,
        InMessage::Ping(buffer) => {
            out_msg_tx.send(OutMessage::Pong(buffer)).await?;
            return Ok(());
        }
    };
    if in_packet.len() < 6 {
        return Err(anyhow!("Invalid packet"));
    }

    let (in_payload, header) = in_packet.split_at(in_packet.len() - 6);

    let packet_id = u32::from_le_bytes(header[0..4].try_into()?);
    let api_index = u16::from_le_bytes(header[4..6].try_into()?);

    // let (mut out_payload, status): (Vec<u8>, Status) =
    let result = handle::handle(api_index, in_payload, &db, session).await?;

    /*
        out packet = [payload][1byte status][4byte packet_id]
        The reason to put metadata at the end is to reduce heap allocation and copying.
    */

    let (mut out_payload, status) = match result {
        handle::HandleResult::Response(x) => (x, 0),
        handle::HandleResult::Error(x) => (x, 1),
    };

    out_payload.extend_from_slice(&(status as u8).to_le_bytes());
    out_payload.extend_from_slice(&packet_id.to_le_bytes());

    out_msg_tx.send(OutMessage::Binary(out_payload)).await?;
    Ok(())
}
