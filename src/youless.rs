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
    electricity_consumption_high_tree: Tree,
    electricity_production_low_tree: Tree,
    electricity_production_high_tree: Tree,
    gas_consumption_tree: Tree,
}

impl Youless {
    pub fn new(db: &Db, client: Client) -> Result<Self> {
        Ok(Self {
            client,
            electricity_power_tree: db.open_tree("youless:electricity:power")?,
            electricity_consumption_low_tree: db
                .open_tree("youless:electricity:consumption:low")?,
            electricity_consumption_high_tree: db
                .open_tree("youless:electricity:consumption:high")?,
            electricity_production_low_tree: db.open_tree("youless:electricity:production:low")?,
            electricity_production_high_tree: db
                .open_tree("youless:electricity:production:high")?,
            gas_consumption_tree: db.open_tree("youless:gas:consumption")?,
        })
    }

    pub async fn run(self) -> Result {
        info!("Youless started");
        loop {
            match self.client.get_counters().await {
                Ok(counters) => {
                    for counters in counters {
                        debug!(?counters.timestamp, "fetched counters");
                        let key = counters.timestamp.into_key();
                        self.electricity_power_tree
                            .insert(key, counters.actual_power_watt)?;
                        self.electricity_consumption_low_tree
                            .insert(key, counters.electricity_consumption_low_kwh)?;
                        self.electricity_consumption_high_tree
                            .insert(key, counters.electricity_consumption_high_kwh)?;
                        self.electricity_production_low_tree
                            .insert(key, counters.electricity_production_low_kwh)?;
                        self.electricity_production_high_tree
                            .insert(key, counters.electricity_production_high_kwh)?;
                        self.gas_consumption_tree
                            .insert(key, counters.gas_consumption_m3)?;
                    }
                }
                Err(error) => {
                    error!("failed to fetch the counters: {:#}", error);
                }
            };

            sleep(time::Duration::from_secs(1)).await;
        }
    }
}
