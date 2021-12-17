use crate::transport_layer::TransportLayer;
use dashmap::DashMap;
use std::{
    rc::Rc,
    sync::{atomic::AtomicU64, Arc},
};
use tokio::sync::Notify;

pub(crate) struct RecallLayer {
    transport_layer: Box<dyn TransportLayer>,
    id: AtomicU64,
    notification_map: Arc<DashMap<u64, Arc<Notify>>>,
    response_data_map: Arc<DashMap<u64, Vec<u8>>>,
}

struct RecallLayerReceiver {
    notification_map: Arc<DashMap<u64, Arc<Notify>>>,
    response_data_map: Arc<DashMap<u64, Vec<u8>>>,
}

impl RecallLayerReceiver {
    fn on_received(&self, packet: Vec<u8>) {
        let (id, data) = self.decode_packet(packet);

        self.response_data_map
            .insert(id, data.to_vec());

        self.notification_map
            .get(&id)
            .unwrap()
            .notify();

        self.notification_map
            .remove(&id);
    }
    fn decode_packet(&self, packet: Vec<u8>) -> (u64, Vec<u8>) {
        let id = u64::from_be_bytes(
            packet[0..8]
                .try_into()
                .unwrap(),
        );
        let data = packet[8..].to_vec();
        (id, data)
    }
}

impl RecallLayer {
    pub fn new(mut transport_layer: impl TransportLayer + 'static) -> Rc<Self> {
        let notification_map = Arc::new(DashMap::new());
        let response_data_map = Arc::new(DashMap::new());

        let receiver = RecallLayerReceiver {
            notification_map: notification_map.clone(),
            response_data_map: response_data_map.clone(),
        };

        transport_layer.on_received(Box::new(move |packet| {
            receiver.on_received(packet);
        }));

        let recall_layer = Rc::new(Self {
            transport_layer: Box::new(transport_layer),
            id: AtomicU64::new(0),
            notification_map: notification_map.clone(),
            response_data_map: response_data_map.clone(),
        });

        recall_layer
    }

    fn encode_packet(&self, id: u64, data: Vec<u8>) -> Vec<u8> {
        let mut packet = Vec::new();
        packet.extend_from_slice(&id.to_be_bytes());
        packet.extend_from_slice(&data);
        packet
    }
    pub async fn send(&self, data: Vec<u8>) -> Result<Vec<u8>, String> {
        let id = self
            .id
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let notify = Arc::new(Notify::new());
        let notify2 = notify.clone();

        self.notification_map
            .insert(id, notify);

        let packet = self.encode_packet(id, data);
        let sent = self
            .transport_layer
            .send(packet);

        match sent {
            Ok(_) => {
                notify2
                    .notified()
                    .await;

                let response_data = self
                    .response_data_map
                    .get(&id)
                    .unwrap()
                    .to_vec();

                self.response_data_map
                    .remove(&id);

                Ok(response_data)
            }
            Err(err) => {
                self.notification_map
                    .remove(&id)
                    .unwrap();
                Err(err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RecallLayer;
    use crate::transport_layer::{self, TransportLayer};

    #[tokio::test]
    async fn test() {
        struct TestTransportLayer {
            pub callback: Box<dyn Fn(Vec<u8>)>,
        }
        impl TransportLayer for TestTransportLayer {
            fn send(&self, packet: Vec<u8>) -> Result<(), String> {
                assert_eq!(packet, vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]);
                (self.callback)(vec![0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 1]);
                Ok(())
            }

            fn on_received(&mut self, callback: Box<dyn Fn(Vec<u8>)>) {
                self.callback = callback;
            }
        }

        let transport_layer = TestTransportLayer {
            callback: Box::new(|_| panic!()),
        };
        let recall_layer = RecallLayer::new(transport_layer);
        let result = recall_layer
            .send(vec![1, 2, 3])
            .await;

        assert_eq!(result.unwrap(), vec![3, 2, 1]);
    }
}
