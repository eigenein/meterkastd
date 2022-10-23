use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(from = "f64")]
pub struct CubicMeter(pub f64);

impl From<f64> for CubicMeter {
    fn from(m3: f64) -> Self {
        Self(m3)
    }
}
