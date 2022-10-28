use crate::prelude::*;

pub trait IntoKey {
    type Output: AsRef<[u8]>;

    fn into_key(self) -> Self::Output;
}

impl<const N: usize> IntoKey for [u8; N] {
    type Output = Self;

    fn into_key(self) -> Self::Output {
        self
    }
}

impl IntoKey for DateTime {
    type Output = [u8; 8];

    fn into_key(self) -> Self::Output {
        self.timestamp_millis().to_be_bytes()
    }
}
