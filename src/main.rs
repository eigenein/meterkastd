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
mod web;
mod youless;

use clap::Parser;
use tokio::{spawn, try_join};

use self::prelude::*;
use crate::args::Args;
use crate::youless::{Client, Youless};

#[tokio::main]
async fn main() -> Result {
    tracing::init()?;
    let args = Args::parse();

    info!("starting up…");
    let db = sled::open(&args.database_path).context("failed to open the database")?;
    let youless_client = Client::new(&args.youless_base_url)?;
    let youless = Youless::new(&db, youless_client)?;

    info!("running…");
    let (web_result, youless_result) =
        try_join!(spawn(web::run(args.bind_endpoint, db)), spawn(youless.run()))?;
    web_result.and(youless_result).context("fatal error")
}
