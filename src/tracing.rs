use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, Layer};

use crate::prelude::*;

pub fn init() -> Result {
    let format_filter =
        EnvFilter::try_from_env("METERKASTD_LOG").or_else(|_| EnvFilter::try_new("info"))?;
    let format_layer = layer().without_time().with_filter(format_filter);

    tracing_subscriber::Registry::default()
        .with(format_layer)
        .init();

    Ok(())
}
