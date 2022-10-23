use crate::prelude::*;

pub struct TimestampKey(DateTime);

impl From<DateTime> for TimestampKey {
    fn from(timestamp: DateTime) -> Self {
        Self(timestamp)
    }
}

impl From<TimestampKey> for [u8; 8] {
    fn from(timestamp: TimestampKey) -> Self {
        timestamp.0.timestamp_millis().to_be_bytes()
    }
}

impl From<TimestampKey> for DateTime {
    fn from(key: TimestampKey) -> Self {
        key.0
    }
}
