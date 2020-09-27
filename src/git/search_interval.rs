use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{Duration, NaiveDate, NaiveDateTime};
use git2::Time;

#[derive(Clone, Copy)]
pub struct SearchInterval {
    pub from: NaiveDateTime,
    pub to: NaiveDateTime,
}

impl SearchInterval {
    pub fn starting(at: NaiveDateTime, duration: Duration) -> SearchInterval {
        let two_weeks = at - duration;
        SearchInterval {
            from: two_weeks,
            to: at,
        }
    }

    pub fn start_now(duration: Duration) -> SearchInterval {
        let now = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(now) => NaiveDateTime::from_timestamp(now.as_secs() as i64, 0),
            Err(error) => {
                crate::log::error(format!("SystemTime error {}", error));
                NaiveDate::from_ymd(2020, 4, 28).and_hms(22, 51, 28)
            }
        };
        SearchInterval::starting(now, duration)
    }

    pub fn get_git_time(&self) -> (Time, Time) {
        (Time::new(self.from.timestamp(), 0), Time::new(self.to.timestamp(), 0))
    }
}
