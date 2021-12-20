use crate::recall_layer::RecallLayerReceiver;
use async_trait::async_trait;

#[async_trait]
pub trait TransportLayer {
    fn set_recall_layer_receiver(&mut self, recall_layer_receiver: RecallLayerReceiver);
    async fn send(&mut self, packet: Vec<u8>) -> Result<(), String>;
    // fn on_received(&mut self, callback: Box<dyn Fn(Vec<u8>)>);
}
