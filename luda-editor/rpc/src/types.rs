pub struct RpcFuture<RpcResult: 'static> {
    pub(crate) future: Box<dyn std::future::Future<Output = RpcResult> + Unpin + 'static>,
}

#[cfg(feature = "client")]
impl<RpcResult: 'static> RpcFuture<RpcResult> {
    pub fn callback(self, callback: impl FnOnce(RpcResult) + 'static) {
        let future = self.future;
        namui::spawn_local(async move {
            let result = future.await;
            callback(result);
        });
    }
}
impl<RpcResult: 'static> std::future::Future for RpcFuture<RpcResult> {
    type Output = RpcResult;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        let this = self.get_mut();
        match std::pin::Pin::new(&mut this.future).poll(cx) {
            std::task::Poll::Ready(result) => std::task::Poll::Ready(result),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
