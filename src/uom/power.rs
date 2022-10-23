use serde::Deserialize;

#[derive(PartialEq, Debug, Deserialize)]
#[serde(from = "f64")]
pub struct Watt(pub f64);

impl From<f64> for Watt {
    fn from(watt: f64) -> Self {
        Self(watt)
    }
}
