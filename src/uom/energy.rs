use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(from = "f64")]
pub struct KilowattHour(pub f64);

impl From<f64> for KilowattHour {
    fn from(kwh: f64) -> Self {
        Self(kwh)
    }
}
