use namui_type::TimeExt;

pub async fn retry_on_error<FuncFuture, FuncOk, FuncErr>(
    func: impl Fn() -> FuncFuture,
    max_retry_count: usize,
) -> Result<FuncOk, FuncErr>
where
    FuncFuture: std::future::Future<Output = Result<FuncOk, FuncErr>>,
{
    let mut retry_count = 0;
    let mut delay = 100.ms();
    loop {
        match func().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                if retry_count < max_retry_count {
                    retry_count += 1;

                    #[cfg(feature = "client")]
                    namui::time::delay(delay).await;

                    #[cfg(feature = "server")]
                    tokio::time::sleep(
                        tokio::time::Duration::from_millis(delay.as_millis() as u64),
                    )
                    .await;

                    delay = {
                        #[cfg(feature = "client")]
                        let random_u8 = namui::random(1)[0];
                        #[cfg(feature = "server")]
                        let random_u8 = rand::random::<u8>();
                        #[cfg(all(not(feature = "server"), not(feature = "client")))]
                        let random_u8 = 0;

                        let collision_avoidance = ((random_u8 % 10) as f32).ms();
                        let next_delay = delay * 2 + collision_avoidance;
                        let max_delay = 4000.ms();
                        if next_delay > max_delay {
                            max_delay
                        } else {
                            next_delay
                        }
                    };
                    continue;
                } else {
                    return Err(error);
                }
            }
        }
    }
}
