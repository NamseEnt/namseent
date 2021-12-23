use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::Notify;

#[derive(Clone)]
pub struct ResponseWaiter {
    notification_map: Arc<DashMap<u64, Arc<Notify>>>,
    response_data_map: Arc<DashMap<u64, Vec<u8>>>,
}
impl ResponseWaiter {
    pub fn new() -> Self {
        Self {
            notification_map: Arc::new(DashMap::new()),
            response_data_map: Arc::new(DashMap::new()),
        }
    }
}

impl ResponseWaiter {
    pub fn ready_to_wait(&self, id: u64) -> Arc<Notify> {
        let notify = Arc::new(Notify::new());
        self.notification_map.insert(id, notify.clone());
        notify
    }
    pub async fn wait(&self, id: u64, notify: Arc<Notify>) -> Result<Vec<u8>, String> {
        notify.notified().await;

        let (_, data) = self
            .response_data_map
            .remove(&id)
            .ok_or_else(|| format!("no response for id {}", id))?;

        Ok(data)
    }
    pub fn on_response(&self, id: u64, response: Vec<u8>) {
        self.response_data_map.insert(id, response);

        let (_, notify) = self.notification_map.remove(&id).unwrap();

        notify.notify_waiters();
    }
}
