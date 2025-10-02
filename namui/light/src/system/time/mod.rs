use crate::*;
use namui_type::*;

static START_AT: OnceLock<Instant> = OnceLock::new();

pub(crate) fn init() -> Result<()> {
    START_AT.set(Instant::now());

    Ok(())
}

/// It's time since the program started.
pub fn since_start() -> Duration {
    START_AT.get().unwrap().elapsed()
}

pub fn system_time_now() -> SystemTime {
    SystemTime::now()
}

/// It's just monotonic time. If you want to get the clock's date or time, use `system_time_now`.
pub fn now() -> Instant {
    Instant::now()
}

pub fn stop_watch(key: impl AsRef<str>) -> StopWatch {
    StopWatch::new(key.as_ref().to_string())
}

/// You can await on this.
///
/// ```ignore
/// sleep(Duration::from_secs(1)).await;
/// ```
///
/// Sleep 0 duration if passed duration is less than 0.
pub fn sleep(duration: Duration) -> tokio::time::Sleep {
    tokio::time::sleep(duration.to_std().unwrap_or_default())
}
