extern crate chrono;
use chrono::prelude::DateTime;
use chrono::Utc;

use std::time::{Duration, UNIX_EPOCH};

pub mod cloud_native_identify;
pub mod file_scanner;
pub mod git;

pub fn format_unix_time(i: u64) -> String {
    let d = UNIX_EPOCH + Duration::from_secs(i);
    let datetime = DateTime::<Utc>::from(d);
    let timestamp_str = datetime.format("%Y-%m-%d %H:%M:%S.%f").to_string();
    return timestamp_str;
}

#[cfg(test)]
mod test {
    use crate::infrastructure::format_unix_time;

    #[test]
    fn format_commit_time() {
        let time = format_unix_time(1610509414);
        assert_eq!("2021-01-13 03:43:34.000000000", time);
    }
}
