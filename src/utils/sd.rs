//! # Stable Diffusion WebUI Client
//!
//! This module provides a lightweight Rust client for interacting with the
//! [Stable Diffusion WebUI](https://github.com/AUTOMATIC1111/stable-diffusion-webui)
//! REST API.
//!
//! ## Example
//! ```no_run
//! # use anyhow::Result;
//! # use crate::utils::sd::*;
//! #
//! # #[tokio::main]
//! # async fn main() -> Result<()> {
//! let client = Client::new("http://localhost:7860")?;
//!
//! let req = GenerateBody {
//!     prompt: "A futuristic city skyline at dusk, ultra‑wide, 8k".into(),
//!     negative_prompt: "blurry, lowres".into(),
//!     width: 1024,
//!     height: 512,
//!     cfg_scale: 7.5,
//!     seed: -1,
//!     steps: 30,
//! };
//!
//! let res = client.generate(req).await?;
//! std::fs::write("output.png", &res.images[0])?;
//! # Ok(())
//! # }
//! ```
//!
//! ---

use std::time::Duration;

use anyhow::Result;
use base64::{Engine, engine::general_purpose};
use reqwest::Url;
use serde::{Deserialize, Serialize, de::DeserializeOwned};

/// JSON body accepted by `/sdapi/v1/txt2img`.
///
/// See the upstream documentation for a complete list of available parameters
/// and their effect on the diffusion process.
#[derive(Debug, Serialize)]
pub struct GenerateBody {
    /// Positive prompt guiding what **should** appear in the generated image.
    pub prompt: String,
    /// Negative prompt guiding what **should _not_** appear in the image.
    pub negative_prompt: String,
    /// Output width (pixels). Must be a multiple of 8.
    pub width: u16,
    /// Output height (pixels). Must be a multiple of 8.
    pub height: u16,
    /// Classifier‑Free Guidance scale (CFG). Higher values follow the prompt
    /// more strictly but may reduce diversity.
    pub cfg_scale: f32,
    /// RNG seed. Use `-1` for a random seed chosen by the server.
    pub seed: i64,
    /// Number of denoising steps.
    pub steps: u8,
}

#[derive(Debug, Deserialize)]
struct Txt2ImgResult {
    pub images: Vec<String>,
    pub info: String,
}

#[derive(Debug)]
pub struct GenerateResult {
    /// Raw PNG/JPEG bytes for each generated image.
    pub images: Vec<Vec<u8>>,
    /// Parsed metadata describing the generation parameters.
    pub info: GenerateInfo,
}

/// Metadata string returned by the Stable Diffusion WebUI, parsed into a struct
/// for ergonomic access.
#[derive(Debug, Deserialize)]
pub struct GenerateInfo {
    pub prompt: String,
    pub negative_prompt: String,
    pub seed: i64,
    pub width: u16,
    pub height: u16,
    pub sampler_name: String,
    pub cfg_scale: f32,
    pub steps: u8,
    pub sd_model_name: String,
    pub sd_model_hash: String,
    pub version: String,
}

pub struct Client {
    base_url: Url,
    http: reqwest::Client,
}

impl Client {
    pub fn new(base_url: impl AsRef<str>) -> Result<Self> {
        let http = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(60 * 3))
            .build()?;
        Ok(Self {
            base_url: Url::parse(base_url.as_ref())?,
            http,
        })
    }

    /// Issues a `POST` request with a JSON body and deserializes the JSON
    /// response into `T`.
    async fn post<T, U>(&self, path: &str, body: U) -> Result<T>
    where
        T: DeserializeOwned,
        U: Serialize + Sized,
    {
        let url = self.base_url.join(path)?;
        let req = self
            .http
            .post(url)
            .json(&body)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;

        Ok(req)
    }

    /// Generates one or more images from the given [`GenerateBody`] payload.
    ///
    pub async fn generate(&self, payload: GenerateBody) -> Result<GenerateResult> {
        let res: Txt2ImgResult = self.post("sdapi/v1/txt2img", payload).await?;
        let mut images: Vec<Vec<u8>> = Vec::with_capacity(res.images.len());

        for image in res.images {
            images.push(general_purpose::STANDARD.decode(image)?);
        }

        let info = serde_json::from_str(&res.info)?;

        Ok(GenerateResult { images, info })
    }
}
