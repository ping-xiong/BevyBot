use anyhow::Result;
use octocrab::{Page, models::Milestone};

use crate::{bots::github_client::build_github_client, tasks::github_task::{BEYV_OWNER, BEVY_REPO}};


pub async fn get_milestone_list() -> Result<Page<Milestone>> {
    let spider = build_github_client()?;

    let milestones: Page<Milestone> = spider
        .get(
            format!("/repos/{}/{}/milestones", BEYV_OWNER, BEVY_REPO),
            None::<&()>
        )
        .await?;

    Ok(milestones)
}



#[cfg(test)]
mod tests {
    use crate::tasks::github_task::watch_milestones::get_milestone_list;

    #[tokio::test]
    async fn test_get_milestone_list() {
        dotenvy::dotenv().ok();
        let list = get_milestone_list().await.unwrap();
        // println!("{:?}", list);
        for item in list {
            println!("版本: {}, id: {}", item.title, item.id)
        }
    }
}
