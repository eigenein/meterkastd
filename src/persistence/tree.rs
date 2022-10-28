use crate::persistence::{IntoKey, IntoValue};
use crate::prelude::*;

#[must_use]
#[derive(Clone)]
pub struct Tree(pub sled::Tree);

impl Tree {
    pub fn insert(&self, key: impl IntoKey, value: impl IntoValue) -> Result {
        self.0
            .insert(key.into_key(), value.into_value())
            .context("failed to insert")?;
        Ok(())
    }
}
