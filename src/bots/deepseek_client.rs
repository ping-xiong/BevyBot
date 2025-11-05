use anyhow::Result;
use deepseek_api::{DeepSeekClient, DeepSeekClientBuilder};

pub fn build_deepseek_clienty() -> Result<DeepSeekClient> {
    let deepseek_api_key = std::env::var("DEEPSEEK_API_KEY")?;
    let deepseek_client = DeepSeekClientBuilder::new(deepseek_api_key).build()?;

    Ok(deepseek_client)
}
