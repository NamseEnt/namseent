use crate::RingBuffer;
use anyhow::Result;
use dashmap::DashMap;
use std::sync::{Arc, OnceLock, atomic::AtomicBool};

pub async fn connect(url: impl ToString) -> Result<(WsSender, WsReceiver)> {
    let ws_thread = WS_THREAD.get_or_init(WsThread::new);
    ws_thread.wait_for_init().await;

    let id = {
        let url = url.to_string();
        tokio::task::spawn_blocking(move || unsafe { _new_web_socket(url.as_ptr(), url.len()) })
            .await
            .unwrap()
    };

    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

    assert!(
        WS_EVENT_TX
            .get_or_init(Default::default)
            .insert(id, event_tx)
            .is_none()
    );

    let event = event_rx.recv().await.unwrap();
    match event {
        WsEvent::Open => {
            // Good!
        }
        WsEvent::Close => {
            WS_EVENT_TX.get().unwrap().remove(&id).unwrap();
            return Err(anyhow::anyhow!("Connection closed"));
        }
        WsEvent::Message(_) => unreachable!(),
    }

    let ws_sender = WsSender { id };

    let (ws_receiver_tx, ws_receiver) = tokio::sync::mpsc::unbounded_channel();

    tokio::spawn(async move {
        while let Some(event) = event_rx.recv().await {
            match event {
                WsEvent::Open => unreachable!(),
                WsEvent::Close => {
                    break;
                }
                WsEvent::Message(data) => {
                    if ws_receiver_tx.send(data).is_err() {
                        break;
                    }
                }
            }
        }
        WS_EVENT_TX.get().unwrap().remove(&id).unwrap();
    });

    Ok((ws_sender, ws_receiver))
}

#[derive(Debug, bincode::Decode, bincode::Encode, Clone, Copy)]
pub struct WsSender {
    id: u32,
}
impl WsSender {
    pub fn send(&self, data: impl AsRef<[u8]>) {
        let data = data.as_ref().to_vec().into_boxed_slice();
        unsafe { _web_socket_send(self.id, data.as_ptr(), data.len()) };
    }
}
pub type WsReceiver = tokio::sync::mpsc::UnboundedReceiver<Box<[u8]>>;

enum WsEvent {
    Open,
    Close,
    Message(Box<[u8]>),
}

static WS_EVENT_TX: OnceLock<DashMap<u32, tokio::sync::mpsc::UnboundedSender<WsEvent>>> =
    OnceLock::new();

unsafe extern "C" {
    fn _init_web_socket_thread(event_buffer_ptr: *const u8, event_buffer_len: usize);
    fn _web_socket_event_poll() -> usize;
    fn _web_socket_event_commit(byte_length: usize);
    fn _new_web_socket(url_ptr: *const u8, url_len: usize) -> u32;
    fn _web_socket_send(id: u32, data_ptr: *const u8, data_len: usize);
}

/// For easy implementation, I chose to use a single thread for all WebSocket connections.
///
/// ```text
/// # eventBuffer protocol
/// [ws id: u32][message type: u8][message data: ...]
/// - 0x01: on open
/// - 0x02: on close
/// - 0x03: on small message (<= 64KB)
///     - u16: byte length
///     - u8[data length]: data
/// - 0x04: on big message start (< 4GB)
///     - u32: total byte length
///     - u16: chunk count
/// - 0x05: on big message chunk
///     - u16: chunk byte length
///     - u8[data length]: data
/// ```
struct WsThread {
    initialized: Arc<AtomicBool>,
}
static WS_THREAD: OnceLock<WsThread> = OnceLock::new();
impl WsThread {
    fn new() -> Self {
        let initialized = Arc::new(AtomicBool::new(false));
        std::thread::spawn({
            let initialized = initialized.clone();
            move || {
                let mut event_buffer = RingBuffer::new(4 * 1024 * 1024);

                unsafe {
                    _init_web_socket_thread(event_buffer.ptr(), event_buffer.size());
                }

                initialized.store(true, std::sync::atomic::Ordering::Relaxed);

                let send_event = |id, event: WsEvent| {
                    let Some(tx) = WS_EVENT_TX.get().unwrap().get(&id) else {
                        return;
                    };
                    let _ = tx.send(event);
                };

                loop {
                    assert_ne!(unsafe { _web_socket_event_poll() }, 0);

                    let id = event_buffer.read_u32();
                    let message_type = event_buffer.read_u8();
                    match message_type {
                        0x01 => {
                            send_event(id, WsEvent::Open);
                            unsafe { _web_socket_event_commit(event_buffer.take_read_count()) }
                        }
                        0x02 => {
                            send_event(id, WsEvent::Close);
                            unsafe { _web_socket_event_commit(event_buffer.take_read_count()) }
                        }
                        0x03 => {
                            let data_length = event_buffer.read_u16() as usize;
                            let data = event_buffer
                                .read_bytes(data_length)
                                .into_owned()
                                .into_boxed_slice();
                            send_event(id, WsEvent::Message(data));
                            unsafe { _web_socket_event_commit(event_buffer.take_read_count()) }
                        }
                        0x04 => {
                            let total_byte_length = event_buffer.read_u32() as usize;
                            let chunk_count = event_buffer.read_u16() as usize;

                            unsafe { _web_socket_event_commit(event_buffer.take_read_count()) }

                            let mut data = Vec::with_capacity(total_byte_length);
                            for _ in 0..chunk_count {
                                let chunk_byte_length = event_buffer.read_u16() as usize;
                                let chunk_data =
                                    event_buffer.read_bytes(chunk_byte_length).into_owned();

                                data.extend_from_slice(&chunk_data);

                                unsafe { _web_socket_event_commit(event_buffer.take_read_count()) }
                            }

                            send_event(id, WsEvent::Message(data.into_boxed_slice()));
                        }
                        _ => unreachable!(),
                    }
                }
            }
        });
        Self { initialized }
    }

    async fn wait_for_init(&self) {
        while !self.initialized.load(std::sync::atomic::Ordering::Relaxed) {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }
}
