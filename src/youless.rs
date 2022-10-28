mod client;
mod models;

use std::time;

use tokio::time::sleep;

pub use self::client::*;
pub use self::models::*;
use crate::persistence::{Db, IntoKey, Tree};
use crate::prelude::*;

pub struct Youless {
    client: Client,
    electricity_power_tree: Tree,
    electricity_consumption_low_tree: Tree,
}

impl Youless {
    const TREE_ELECTRICITY_CONSUMPTION_LOW: &'static str = "youless:electricity:consumption:low";
    const TREE_ELECTRICITY_POWER: &'static str = "youless:electricity:power";

    pub fn new(db: &Db, client: Client) -> Result<Self> {
        Ok(Self {
            client,
            electricity_power_tree: db.open_tree(Self::TREE_ELECTRICITY_POWER)?,
            electricity_consumption_low_tree: db
                .open_tree(Self::TREE_ELECTRICITY_CONSUMPTION_LOW)?,
        })
    }

    pub async fn run(self) -> Result {
        loop {
            for counters in self.client.get_counters().await? {
                info!(counters.actual_power_watt);
                let key = counters.timestamp.into_key();
                self.electricity_power_tree
                    .insert(key, counters.actual_power_watt)?;
                self.electricity_consumption_low_tree
                    .insert(key, counters.electricity_consumption_low_kwh)?;
            }
            sleep(time::Duration::from_secs(1)).await;
        }
    }
}
