pub struct Socket {
    sender: UnboundedSender<Vec<u8>>,
    response_waiter: ResponseWaiter,
}
impl Clone for Socket {
    fn clone(&self) -> Self {
        Self {
            sender: self
                .sender
                .clone(),
            response_waiter: self
                .response_waiter
                .clone(),
        }
    }
}
