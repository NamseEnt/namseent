use crate::Duration;

pub trait DurationExt {
    fn ms(self) -> Duration;
    fn sec(self) -> Duration;
    fn minute(self) -> Duration;
    fn hour(self) -> Duration;
    fn day(self) -> Duration;
    fn week(self) -> Duration;
}

impl DurationExt for f64 {
    fn ms(self) -> Duration {
        Duration::from_secs_f64(self / 1000.0)
    }

    fn sec(self) -> Duration {
        Duration::from_secs_f64(self)
    }

    fn minute(self) -> Duration {
        f64::sec(self * 60.0)
    }

    fn hour(self) -> Duration {
        f64::minute(self * 60.0)
    }

    fn day(self) -> Duration {
        f64::hour(self * 24.0)
    }

    fn week(self) -> Duration {
        f64::day(self * 7.0)
    }
}

impl DurationExt for i32 {
    fn ms(self) -> Duration {
        Duration::from_millis(self as i64)
    }

    fn sec(self) -> Duration {
        Duration::from_secs(self as i64)
    }

    fn minute(self) -> Duration {
        Duration::from_secs(self as i64 * 60)
    }

    fn hour(self) -> Duration {
        Duration::from_secs(self as i64 * 60 * 60)
    }

    fn day(self) -> Duration {
        Duration::from_secs(self as i64 * 60 * 60 * 24)
    }

    fn week(self) -> Duration {
        Duration::from_secs(self as i64 * 60 * 60 * 24 * 7)
    }
}
