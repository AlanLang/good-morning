use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};

pub async fn send_message(url: String, info: MessageInfo) -> Result<()> {
    let _ = reqwest::Client::new()
        .post(url.clone())
        .json(&info)
        .send()
        .await?
        .text()
        .await?;
    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MessageInfo {
    pub msgtype: String,
    pub news: News,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub articles: Vec<Article>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub title: String,
    pub description: String,
    pub url: String,
    pub picurl: String,
}

impl MessageInfo {
    pub fn new(title: String, description: String, picurl: String) -> Self {
        let article = Article {
            title: title,
            description: description,
            url: "https://www.yuque.com/u68186/owc2wh/uxiqkm".to_string(),
            picurl: picurl,
        };
        let mut news = News::default();
        news.articles.push(article);
        Self {
            msgtype: "news".to_string(),
            news: news,
        }
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn it_works() {
        let title = format!("{}天气 {} , 温度 {}°C", "无锡", "多云", "27");
        let description = format!(
            "起床啦，喝杯咖啡，背个单词，去上班。\n今日诗句\n{}",
            "我是诗句"
        );
        let message = super::MessageInfo::new(
            title,
            description,
            "https://vip2.loli.io/2023/08/02/FZS59UEMp7BqoTW.webp".to_string(),
        );
        let _ = super::send_message("https://qyapi.weixin.qq.com/cgi-bin/webhook/send?key=bf7b051b-a8e9-4af6-a836-e623a1988b81".to_string(), message).await.unwrap();
    }
}
