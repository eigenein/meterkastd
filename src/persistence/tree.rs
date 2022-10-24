use crate::persistence::BigEndian;
use crate::prelude::*;

#[must_use]
pub struct Tree(sled::Tree);

impl Tree {
    pub const fn new(tree: sled::Tree) -> Self {
        Self(tree)
    }

    pub fn insert<const N: usize, const M: usize>(
        &self,
        key: impl BigEndian<N>,
        value: impl BigEndian<M>,
    ) -> Result {
        self.0
            .insert(key.to_be_bytes(), value.to_be_bytes().as_slice())
            .context("failed to insert the value")?;
        Ok(())
    }
}
