use crate::transport_layer::TransportLayer;
use dashmap::DashMap;
use std::{
    rc::Rc,
    sync::{atomic::AtomicU64, Arc},
};
use tokio::sync::Notify;

pub struct RecallLayer {
    transport_layer: Box<dyn TransportLayer>,
    id: AtomicU64,
    notification_map: Arc<DashMap<u64, Arc<Notify>>>,
    response_data_map: Arc<DashMap<u64, Vec<u8>>>,
}

pub struct RecallLayerReceiver {
    notification_map: Arc<DashMap<u64, Arc<Notify>>>,
    response_data_map: Arc<DashMap<u64, Vec<u8>>>,
}

impl RecallLayerReceiver {
    pub fn on_received(&self, packet: Vec<u8>) {
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
    pub fn new(mut transport_layer: impl TransportLayer + 'static) -> Self {
        let notification_map = Arc::new(DashMap::new());
        let response_data_map = Arc::new(DashMap::new());

        let receiver = RecallLayerReceiver {
            notification_map: notification_map.clone(),
            response_data_map: response_data_map.clone(),
        };

        // transport_layer.on_received(Box::new(move |packet| {
        //     receiver.on_received(packet);
        // }));
        transport_layer.set_recall_layer_receiver(receiver);

        let recall_layer = Self {
            transport_layer: Box::new(transport_layer),
            id: AtomicU64::new(0),
            notification_map: notification_map.clone(),
            response_data_map: response_data_map.clone(),
        };

        recall_layer
    }

    fn encode_packet(&self, id: u64, data: Vec<u8>) -> Vec<u8> {
        let mut packet = Vec::new();
        packet.extend_from_slice(&id.to_be_bytes());
        packet.extend_from_slice(&data);
        packet
    }
    pub async fn send(&mut self, data: Vec<u8>) -> Result<Vec<u8>, String> {
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
            .send(packet)
            .await;

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
    use std::{sync::Arc, thread};

    use async_trait::async_trait;
    use tokio::runtime::Runtime;

    use super::RecallLayer;
    use crate::{recall_layer::RecallLayerReceiver, transport_layer::TransportLayer};

    #[tokio::test]
    async fn test() {
        struct TestTransportLayer {
            recall_layer_receiver: Option<RecallLayerReceiver>,
        }
        #[async_trait]
        impl TransportLayer for TestTransportLayer {
            async fn send(&mut self, packet: Vec<u8>) -> Result<(), String> {
                assert_eq!(packet, vec![0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3]);
                self.recall_layer_receiver
                    .as_ref()
                    .unwrap()
                    .on_received(vec![0, 0, 0, 0, 0, 0, 0, 0, 3, 2, 1]);
                Ok(())
            }

            fn set_recall_layer_receiver(
                &mut self,
                recall_layer_receiver: super::RecallLayerReceiver,
            ) {
                self.recall_layer_receiver = Some(recall_layer_receiver);
            }
        }

        let transport_layer = TestTransportLayer {
            recall_layer_receiver: None,
        };

        let mut recall_layer = RecallLayer::new(transport_layer);
        let result = recall_layer
            .send(vec![1, 2, 3])
            .await;

        assert_eq!(result.unwrap(), vec![3, 2, 1]);
    }
}
