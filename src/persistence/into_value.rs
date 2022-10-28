pub trait IntoValue {
    fn into_value(self) -> sled::IVec;
}

impl IntoValue for f64 {
    fn into_value(self) -> sled::IVec {
        sled::IVec::from(&self.to_be_bytes())
    }
}
