use anyhow::{Ok, Result};
use log::debug;
use serde::{Deserialize, Serialize};

pub async fn get_poetry() -> Result<PoetryResult> {
    debug!("Getting poetry");
    let url = format!("https://v1.jinrishici.com/shanshui");
    let body = reqwest::get(url).await?.json::<PoetryResult>().await?;
    Ok(body)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoetryResult {
    pub content: String,
    pub origin: String,
    pub author: String,
    pub category: String,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        let poetry = super::get_poetry().await.unwrap();
        assert!(poetry.content.len() > 0);
    }
}
