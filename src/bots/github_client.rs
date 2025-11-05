use anyhow::Result;
use octocrab::Octocrab;


pub fn build_github_client() -> Result<Octocrab> {
    let token = std::env::var("GITHUB_PERSON_TOKEN")?;
    let spider = octocrab::Octocrab::builder()
        .personal_token(token)
        .build()?;

    Ok(spider)
}
