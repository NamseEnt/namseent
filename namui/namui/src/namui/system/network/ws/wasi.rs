use anyhow::Result;
use dashmap::DashMap;
use std::sync::OnceLock;

/// For easy implementation, I chose to use a single thread for all WebSocket connections.
struct WsThread {
    job_tx: std::sync::mpsc::Sender<WsThreadJob>,
}
static WS_THREAD: OnceLock<WsThread> = OnceLock::new();
impl WsThread {
    fn new() -> Self {
        let (job_tx, job_rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            while let Ok(job) = job_rx.recv() {
                match job {
                    WsThreadJob::NewWebSocket { url, result } => {
                        let id = unsafe { new_web_socket(url.as_ptr(), url.len()) };
                        let _ = result.send(id);
                    }
                    WsThreadJob::Send { id, data } => {
                        unsafe { web_socket_send(id, data.as_ptr(), data.len()) };
                    }
                }
            }
        });
        Self { job_tx }
    }

    async fn new_web_socket(&self, url: impl AsRef<str>) -> u32 {
        let (result_tx, result_rx) = tokio::sync::oneshot::channel();
        self.job_tx
            .send(WsThreadJob::NewWebSocket {
                url: url.as_ref().to_string(),
                result: result_tx,
            })
            .unwrap();
        result_rx.await.unwrap()
    }
}
enum WsThreadJob {
    NewWebSocket {
        url: String,
        result: tokio::sync::oneshot::Sender<u32>,
    },
    Send {
        id: u32,
        data: Box<[u8]>,
    },
}

pub async fn connect(url: impl AsRef<str>) -> Result<(WsSender, WsReceiver)> {
    let ws_thread = WS_THREAD.get_or_init(WsThread::new);
    let id = ws_thread.new_web_socket(url).await;

    let (event_tx, mut event_rx) = tokio::sync::mpsc::unbounded_channel();

    assert!(WS_EVENT_TX
        .get_or_init(Default::default)
        .insert(id, event_tx)
        .is_none());

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

pub struct WsSender {
    id: u32,
}
impl WsSender {
    pub fn send(&self, data: &[u8]) {
        let data = data.to_vec().into_boxed_slice();
        WS_THREAD
            .get()
            .unwrap()
            .job_tx
            .send(WsThreadJob::Send { id: self.id, data })
            .unwrap();
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

extern "C" {
    fn new_web_socket(url_ptr: *const u8, url_len: usize) -> u32;
    fn web_socket_send(id: u32, data_ptr: *const u8, data_len: usize);
}

#[no_mangle]
pub extern "C" fn on_web_socket_open(id: u32) {
    let _ = WS_EVENT_TX
        .get()
        .unwrap()
        .get(&id)
        .unwrap()
        .send(WsEvent::Open);
}

#[no_mangle]
pub extern "C" fn on_web_socket_close(id: u32) {
    let _ = WS_EVENT_TX
        .get()
        .unwrap()
        .get(&id)
        .unwrap()
        .send(WsEvent::Close);
}

static MESSAGE_HEAP: OnceLock<DashMap<MessagePtr, Box<[u8]>>> = OnceLock::new();

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct MessagePtr {
    ptr: *mut u8,
}
unsafe impl Send for MessagePtr {}
unsafe impl Sync for MessagePtr {}

#[no_mangle]
pub extern "C" fn web_socket_message_alloc(data_len: usize) -> *mut u8 {
    let data = vec![0; data_len].into_boxed_slice();
    let ptr = data.as_ptr() as *mut u8;
    assert!(MESSAGE_HEAP
        .get_or_init(Default::default)
        .insert(MessagePtr { ptr }, data)
        .is_none());
    ptr
}

#[no_mangle]
pub extern "C" fn on_web_socket_message(id: u32, data_ptr: *mut u8) {
    let (_, data) = MESSAGE_HEAP
        .get_or_init(Default::default)
        .remove(&MessagePtr { ptr: data_ptr })
        .unwrap();

    if let Some(tx) = WS_EVENT_TX.get().unwrap().get(&id) {
        let _ = tx.send(WsEvent::Message(data));
    }
}
