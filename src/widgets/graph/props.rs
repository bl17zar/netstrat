use chrono::{Date, DateTime, Duration, NaiveTime, Timelike, Utc};

use crate::sources::binance::Interval;

#[derive(Debug, Clone, Copy)]
pub struct Props {
    pub date_start: Date<Utc>,
    pub date_end: Date<Utc>,
    pub time_start: NaiveTime,
    pub time_end: NaiveTime,
    pub interval: Interval,
    pub limit: usize,
}

impl Props {
    pub fn start_time(&self) -> DateTime<Utc> {
        self.date_start.and_hms(
            self.time_start.hour(),
            self.time_start.minute(),
            self.time_start.second(),
        )
    }

    pub fn end_time(&self) -> DateTime<Utc> {
        self.date_end.and_hms(
            self.time_end.hour(),
            self.time_end.minute(),
            self.time_end.second(),
        )
    }

    pub fn is_valid(&self) -> bool {
        self.start_time() < self.end_time()
    }
}

impl Default for Props {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            date_start: now.date() - Duration::days(1),
            date_end: now.date(),
            time_start: NaiveTime::from_hms(0, 0, 0),
            time_end: NaiveTime::from_hms(now.hour(), now.minute(), now.second()),
            interval: Interval::Minute,
            limit: 1000,
        }
    }
}
