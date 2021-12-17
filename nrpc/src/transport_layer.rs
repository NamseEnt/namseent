pub trait TransportLayer {
    fn send(&self, packet: Vec<u8>) -> Result<(), String>;
    fn on_received(&mut self, callback: Box<dyn Fn(Vec<u8>)>);
}
