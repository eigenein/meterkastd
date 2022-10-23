mod client;
mod models;

use std::sync::Arc;

pub use self::client::*;
pub use self::models::*;
use crate::prelude::*;
use crate::Database;

pub async fn run(db: Arc<Database>, client: Client) -> Result {
    Ok(())
}
