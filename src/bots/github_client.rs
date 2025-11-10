use std::time::Duration;

use anyhow::Result;
use octocrab::Octocrab;

use crate::bots::REQUEST_TIME_OUT_SEC;


pub fn build_github_client() -> Result<Octocrab> {
    let token = std::env::var("GITHUB_PERSON_TOKEN")?;
    let spider = octocrab::Octocrab::builder()
        .personal_token(token)
        .set_connect_timeout(Some(Duration::from_secs(REQUEST_TIME_OUT_SEC)))
        .set_read_timeout(Some(Duration::from_secs(REQUEST_TIME_OUT_SEC)))
        .set_write_timeout(Some(Duration::from_secs(REQUEST_TIME_OUT_SEC)))
        .build()?;

    Ok(spider)
}
