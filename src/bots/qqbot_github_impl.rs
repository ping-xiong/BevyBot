use std::env;

use crate::bots::qqbot_client::QQBotClient;
use anyhow::Result;
use chrono::Local;
use log::info;
use serde::Serialize;
use serde_json::json;


impl QQBotClient {
    pub async fn send_any_thread(&self, data: impl Serialize, sub_channel_id: &str) -> Result<serde_json::Value> {
        let res: serde_json::Value = self.put(
            format!("/channels/{}/threads", sub_channel_id),
            data
        ).await?;

        info!("{:?}", res);

        Ok(res)
    }

    pub async fn send_thread(&self, title: &str, text: &String, sub_channel_id: &str) -> Result<()> {
        let res = self.send_any_thread(json!({
            "title": title,
            "content": text,
            "format": 3
        }), sub_channel_id)
        .await?;

        info!("{:?}", res);

        Ok(())
    }

    pub async fn send_issue_summary(&self, title: &str, text: &String) -> Result<()> {
        let title = format!("每日 {} 总结：{}", title, Local::now().format("%Y-%m-%d"));
        self.send_thread(&title, text, &env::var("ISSUE_CHANNEL_ID").unwrap_or_default())
            .await?;
        Ok(())
    }

    pub async fn send_commit_summary(&self, title: &str, text: &String) -> Result<()> {
        let title = format!("每日 {} 总结：{}", title, Local::now().format("%Y-%m-%d"));
        self.send_thread(&title, text, &env::var("COMMINT_CHANNEL_ID").unwrap_or_default())
            .await?;

        Ok(())
    }

    pub async fn send_pr_summary(&self, title: &str, text: &String) -> Result<()> {
        let title = format!("每日 {} 总结：{}", title, Local::now().format("%Y-%m-%d"));

        self.send_thread(&title, text, &env::var("PR_CHANNEL_ID").unwrap_or_default())
            .await?;

        Ok(())
    }

}
