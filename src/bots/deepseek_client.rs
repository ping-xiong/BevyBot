use anyhow::Result;
use deepseek_api::{DeepSeekClient, DeepSeekClientBuilder};

use crate::bots::REQUEST_TIME_OUT_SEC;

pub fn build_deepseek_client() -> Result<DeepSeekClient> {
    let deepseek_api_key = std::env::var("DEEPSEEK_API_KEY")?;
    let deepseek_client = DeepSeekClientBuilder::new(deepseek_api_key).timeout(REQUEST_TIME_OUT_SEC).build()?;

    Ok(deepseek_client)
}
