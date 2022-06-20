use eyre::eyre;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {
    id: String,
    source: String,
    title: String,
    author: String,
    chapters: i64,
    words: i64,
    description: String,
    created: String,
    updated: String,
}

pub async fn meta(url: &str) -> eyre::Result<Option<Meta>> {
    // todo: sane error handling
    let client = reqwest::Client::builder()
        .user_agent("fic.ai")
        .build()
        .map_err(|e| eyre!("failed to connect to build fichub client: {:?}", e))?;
    Ok(Some(
        client
            .get("https://fichub.net/api/v0/meta")
            .query(&[("q", url)])
            .send()
            .await
            .map_err(|e| eyre!("failed to connect to send request: {:?}", e))?
            .json()
            .await
            .map_err(|e| eyre!("failed to connect to fetch body: {:?}", e))?,
    ))
}
