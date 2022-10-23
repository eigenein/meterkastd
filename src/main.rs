#![warn(
    clippy::all,
    clippy::missing_const_for_fn,
    clippy::trivially_copy_pass_by_ref,
    clippy::map_unwrap_or,
    clippy::explicit_into_iter_loop,
    clippy::unused_self,
    clippy::needless_pass_by_value
)]

mod args;
mod prelude;
mod tracing;
mod uom;
mod youless;

use clap::Parser;

use self::prelude::*;
use crate::args::Args;
use crate::youless::Client;

#[tokio::main]
async fn main() -> Result {
    tracing::init()?;
    let args = Args::parse();
    info!(%args.youless_base_url, "starting up…");
    let youless_client = Client::new(&args.youless_base_url)?;
    let values = youless_client.get_counters().await?;
    info!(?values);
    Ok(())
}
