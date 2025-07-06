use std::time::Duration;

use anyhow::Result;
use reqwest::{Client, Url};
use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug, Deserialize)]
pub struct WaifuAngry {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct WaifuBaka {
    pub url: String,
}

pub struct Waifu {
    http: Client,
    token: String,
    base_url: Url,
}

impl Waifu {
    pub fn new(token: impl Into<String>) -> Result<Self> {
        let http = Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60))
            .build()?;

        Ok(Self {
            http,
            token: token.into(),
            base_url: Url::parse("https://waifu.it/api/v4")?,
        })
    }

    async fn get<T>(&self, path: &str) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let response = self
            .http
            .get(self.base_url.join(path)?)
            .bearer_auth(&self.token)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;

        Ok(response)
    }

    pub async fn angry(&self) -> Result<WaifuAngry> {
        self.get("/angry").await
    }

    pub async fn baka(&self) -> Result<WaifuBaka> {
        self.get("/baka").await
    }
}

