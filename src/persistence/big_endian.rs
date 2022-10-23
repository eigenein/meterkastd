use chrono::TimeZone;

use crate::prelude::*;

pub trait BigEndian<const N: usize> {
    fn from_be_bytes(bytes: [u8; N]) -> Self;

    fn to_be_bytes(self) -> [u8; N];
}

impl BigEndian<8> for f64 {
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Self::from_be_bytes(bytes)
    }

    fn to_be_bytes(self) -> [u8; 8] {
        f64::to_be_bytes(self)
    }
}

impl BigEndian<8> for DateTime {
    fn from_be_bytes(bytes: [u8; 8]) -> Self {
        Utc.timestamp_millis(i64::from_be_bytes(bytes))
    }

    fn to_be_bytes(self) -> [u8; 8] {
        self.timestamp_millis().to_be_bytes()
    }
}
