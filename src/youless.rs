mod client;
mod models;

use std::sync::Arc;
use std::time;

use tokio::time::sleep;

pub use self::client::*;
pub use self::models::*;
use crate::enums::{CounterType, EnergyType, FlowDirection};
use crate::prelude::*;
use crate::Database;

pub async fn run(db: Arc<Database>, client: Client) -> Result {
    loop {
        for counters in client.get_counters().await? {
            info!(counters.actual_power_watt);
            db.open_sensor_tree(
                EnergyType::Electricity,
                FlowDirection::Consumption,
                CounterType::Current,
            )?
            .insert(counters.timestamp, counters.actual_power_watt)?;
        }
        sleep(time::Duration::from_secs(1)).await;
    }
}
