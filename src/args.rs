use std::path::PathBuf;

use clap::Parser;
use reqwest::Url;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// [YouLess](https://www.youless.nl) meter endpoint.
    #[arg(
        long = "youless",
        value_name = "ENDPOINT",
        default_value = "http://youless.local",
        env = "METERKASTD_YOULESS_BASE_URL"
    )]
    pub youless_base_url: Url,

    /// Sensor database path – a [sled](https://sled.rs/) embedded database.
    #[arg(
        long = "database",
        value_name = "PATH",
        default_value = "meterkastd.sled",
        env = "METERKASTD_DATABASE_PATH"
    )]
    pub database_path: PathBuf,

    /// Web user interface bind endpoint.
    #[arg(
        long = "bind",
        value_name = "ENDPOINT",
        default_value = "localhost:8082",
        env = "METERKASTD_BIND_ENDPOINT"
    )]
    pub bind_endpoint: PathBuf,
}
