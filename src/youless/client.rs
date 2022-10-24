use reqwest::Url;

use crate::prelude::*;
use crate::youless::Counters;

/// YouLess client.
pub struct Client {
    client: reqwest::Client,

    /// Uploaded values URL.
    /// See: https://wiki.td-er.nl/index.php?title=YouLess#Enelogic_.28default.29_firmware.
    export_url: Url,
}

impl Client {
    #[instrument(skip_all, fields(base_url = ?base_url))]
    pub fn new(base_url: &Url) -> Result<Self> {
        let export_url = base_url
            .join("e")
            .context("failed to build the YouLess API URL")?;
        let client = reqwest::ClientBuilder::new()
            .build()
            .context("failed to build the client")?;
        Ok(Self { export_url, client })
    }

    #[instrument(skip_all)]
    pub async fn get_counters(&self) -> Result<Vec<Counters>> {
        self.client
            .get(self.export_url.clone())
            .send()
            .await
            .context("failed to send the request to YouLess")?
            .error_for_status()
            .context("YouLess HTTP request has failed")?
            .json()
            .await
            .context("failed to deserialize the YouLess response")
    }
}
