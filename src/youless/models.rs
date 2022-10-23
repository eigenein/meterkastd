use serde::Deserialize;

use crate::prelude::*;

#[serde_with::serde_as]
#[derive(Deserialize, Debug)]
pub struct Counters {
    #[serde(rename = "tm")]
    #[serde_as(as = "serde_with::TimestampSeconds<i64>")]
    pub timestamp: DateTime,

    #[serde(rename = "pwr")]
    pub actual_power_watt: f64,

    #[serde(rename = "p1")]
    pub electricity_consumption_low_kwh: f64,

    #[serde(rename = "p2")]
    pub electricity_consumption_high_kwh: f64,

    #[serde(rename = "n1")]
    pub electricity_production_low_kwh: f64,

    #[serde(rename = "n2")]
    pub electricity_production_high_kwh: f64,

    #[serde(rename = "net")]
    pub electricity_consumption_net_kwh: f64,

    #[serde(rename = "gas")]
    pub gas_consumption_m3: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_ok() -> Result {
        // language=json
        let counters: Counters = serde_json::from_str(
            r#"{"tm":1666532025,"net": 11993.485,"pwr": 512,"ts0":1663603259,"cs0": 0.000,"ps0": 0,"p1": 9334.267,"p2": 8179.826,"n1": 1713.316,"n2": 3807.292,"gas": 7148.355,"gts":2210231530}"#,
        )?;
        assert_eq!(counters.actual_power_watt, 512.0);
        assert_eq!(counters.electricity_consumption_low_kwh, 9334.267);
        Ok(())
    }
}
