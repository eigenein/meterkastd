use sled::IVec;

use crate::persistence::timestamp::TimestampKey;
use crate::prelude::*;

pub struct SensorLog(sled::Tree);

impl SensorLog {
    pub const fn new(tree: sled::Tree) -> Self {
        Self(tree)
    }

    pub fn insert(&self, timestamp: DateTime, value: f64) -> Result {
        self.0
            .insert(Self::convert_key(timestamp), &value.to_be_bytes())
            .context("failed to insert the value")?;
        Ok(())
    }

    #[inline]
    fn convert_key<K: Into<TimestampKey>>(key: K) -> [u8; 8] {
        key.into().into()
    }

    #[inline]
    fn ivec_to_f64(ivec: &IVec) -> Result<f64> {
        Ok(f64::from_be_bytes(ivec.as_ref().try_into()?))
    }
}