use crate::persistence::Tree;
use crate::prelude::*;

#[must_use]
#[derive(Clone)]
pub struct Db(pub sled::Db);

impl Db {
    pub fn open_tree(&self, name: &str) -> Result<Tree> {
        self.0
            .open_tree(name)
            .with_context(|| format!("failed to open tree `{}`", name))
            .map(Tree)
    }
}
