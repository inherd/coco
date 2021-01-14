extern crate chrono;

use std::time::{Duration, UNIX_EPOCH};

use chrono::prelude::DateTime;
use chrono::Utc;

pub fn format_unix_time(i: u64) -> String {
    if i == 0 {
        return "".to_string();
    }

    let d = UNIX_EPOCH + Duration::from_secs(i);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
    return timestamp_str;
}

#[cfg(test)]
mod test {
    use crate::infrastructure::time_format::format_unix_time;

    #[test]
    fn format_commit_time() {
        let time = format_unix_time(1610509414);
        assert_eq!("2021-01-13 03:43:34", time);
    }

    #[test]
    fn format_commit_zero() {
        let time = format_unix_time(0);
        assert_eq!("", time);
    }
}
