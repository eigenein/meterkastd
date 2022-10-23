use std::path::PathBuf;

use crate::enums::{CounterType, EnergyType, FlowDirection};
use crate::persistence::Tree;
use crate::prelude::*;

pub struct Database(sled::Db);

impl Database {
    #[instrument(skip_all, fields(path = ?path))]
    pub fn open(path: &PathBuf) -> Result<Self> {
        sled::open(path)
            .with_context(|| format!("failed to open the database: {:?}", path))
            .map(Self)
    }

    #[instrument(skip_all)]
    pub fn open_sensor_tree(
        &self,
        energy_type: EnergyType,
        flow_direction: FlowDirection,
        counter_type: CounterType,
    ) -> Result<Tree> {
        let name = format!("Youless/{:?}/{:?}/{:?}", energy_type, flow_direction, counter_type);
        self.0
            .open_tree(&name)
            .with_context(|| format!("failed to open the sensor tree: {}", name))
            .map(Tree::new)
    }
}
