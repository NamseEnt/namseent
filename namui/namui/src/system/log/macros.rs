#[macro_export]
macro_rules! info_once {
    ($($arg:tt)*) => {{
        static __NAMUI_LOG_ONCE: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);
        if !__NAMUI_LOG_ONCE.swap(true, ::std::sync::atomic::Ordering::Relaxed) {
            $crate::tracing::info!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! warn_once {
    ($($arg:tt)*) => {{
        static __NAMUI_LOG_ONCE: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);
        if !__NAMUI_LOG_ONCE.swap(true, ::std::sync::atomic::Ordering::Relaxed) {
            $crate::tracing::warn!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! error_once {
    ($($arg:tt)*) => {{
        static __NAMUI_LOG_ONCE: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);
        if !__NAMUI_LOG_ONCE.swap(true, ::std::sync::atomic::Ordering::Relaxed) {
            $crate::tracing::error!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_once {
    ($($arg:tt)*) => {{
        static __NAMUI_LOG_ONCE: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);
        if !__NAMUI_LOG_ONCE.swap(true, ::std::sync::atomic::Ordering::Relaxed) {
            $crate::tracing::debug!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! trace_once {
    ($($arg:tt)*) => {{
        static __NAMUI_LOG_ONCE: ::std::sync::atomic::AtomicBool =
            ::std::sync::atomic::AtomicBool::new(false);
        if !__NAMUI_LOG_ONCE.swap(true, ::std::sync::atomic::Ordering::Relaxed) {
            $crate::tracing::trace!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! info_every_n {
    ($n:expr, $($arg:tt)*) => {{
        static __NAMUI_LOG_COUNTER: ::std::sync::atomic::AtomicU64 =
            ::std::sync::atomic::AtomicU64::new(0);
        let count = __NAMUI_LOG_COUNTER.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        if count % ($n as u64) == 0 {
            $crate::tracing::info!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! warn_every_n {
    ($n:expr, $($arg:tt)*) => {{
        static __NAMUI_LOG_COUNTER: ::std::sync::atomic::AtomicU64 =
            ::std::sync::atomic::AtomicU64::new(0);
        let count = __NAMUI_LOG_COUNTER.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        if count % ($n as u64) == 0 {
            $crate::tracing::warn!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_every_n {
    ($n:expr, $($arg:tt)*) => {{
        static __NAMUI_LOG_COUNTER: ::std::sync::atomic::AtomicU64 =
            ::std::sync::atomic::AtomicU64::new(0);
        let count = __NAMUI_LOG_COUNTER.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        if count % ($n as u64) == 0 {
            $crate::tracing::debug!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! trace_every_n {
    ($n:expr, $($arg:tt)*) => {{
        static __NAMUI_LOG_COUNTER: ::std::sync::atomic::AtomicU64 =
            ::std::sync::atomic::AtomicU64::new(0);
        let count = __NAMUI_LOG_COUNTER.fetch_add(1, ::std::sync::atomic::Ordering::Relaxed);
        if count % ($n as u64) == 0 {
            $crate::tracing::trace!($($arg)*);
        }
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __namui_throttle_should_emit {
    ($duration:expr) => {{
        static __NAMUI_THROTTLE_ORIGIN: ::std::sync::OnceLock<::std::time::Instant> =
            ::std::sync::OnceLock::new();
        static __NAMUI_THROTTLE_LAST: ::std::sync::atomic::AtomicU64 =
            ::std::sync::atomic::AtomicU64::new(u64::MAX);
        let origin = *__NAMUI_THROTTLE_ORIGIN.get_or_init(::std::time::Instant::now);
        let now_nanos = ::std::time::Instant::now()
            .saturating_duration_since(origin)
            .as_nanos()
            .min(u64::MAX as u128) as u64;
        let interval_nanos = ($duration).as_nanos().min(u64::MAX as u128) as u64;
        let last = __NAMUI_THROTTLE_LAST.load(::std::sync::atomic::Ordering::Relaxed);
        if last == u64::MAX || now_nanos.saturating_sub(last) >= interval_nanos {
            __NAMUI_THROTTLE_LAST.store(now_nanos, ::std::sync::atomic::Ordering::Relaxed);
            true
        } else {
            false
        }
    }};
}

#[macro_export]
macro_rules! info_throttled {
    ($duration:expr, $($arg:tt)*) => {{
        if $crate::__namui_throttle_should_emit!($duration) {
            $crate::tracing::info!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! warn_throttled {
    ($duration:expr, $($arg:tt)*) => {{
        if $crate::__namui_throttle_should_emit!($duration) {
            $crate::tracing::warn!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! error_throttled {
    ($duration:expr, $($arg:tt)*) => {{
        if $crate::__namui_throttle_should_emit!($duration) {
            $crate::tracing::error!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! debug_throttled {
    ($duration:expr, $($arg:tt)*) => {{
        if $crate::__namui_throttle_should_emit!($duration) {
            $crate::tracing::debug!($($arg)*);
        }
    }};
}

#[macro_export]
macro_rules! trace_throttled {
    ($duration:expr, $($arg:tt)*) => {{
        if $crate::__namui_throttle_should_emit!($duration) {
            $crate::tracing::trace!($($arg)*);
        }
    }};
}
