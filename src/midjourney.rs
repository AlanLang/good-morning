use std::time::Duration;

use anyhow::{Ok, Result};
use log::{debug, info};
use reqwest::header::{self, HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::time::{self, Instant};
pub struct Midjourney {
    proxy_url: String,
    secret: String,
}
impl Midjourney {
    pub fn new(proxy_url: String, secret: String) -> Self {
        Self { proxy_url, secret }
    }

    pub async fn get_first_image(&self, prompt: String) -> Result<String> {
        debug!("Getting first image: {}", prompt);
        let result = self.submit_image(prompt).await?;
        let id = result.result;
        let start = Instant::now() + Duration::from_secs(60);
        let mut interval = time::interval_at(start, time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            debug!("等待图片生成完成: {}", id);
            let jog = self.get_job(id.clone()).await?;
            if jog.progress == "100%" {
                break;
            }
        }
        let change_result = self
            .submit_change(ImageChangeParams::new(id.clone()))
            .await?;
        loop {
            debug!("等待图片选取完成: {}", id);
            interval.tick().await;
            let jog = self.get_job(change_result.result.clone()).await?;
            if jog.progress == "100%" {
                return Ok(jog.imageUrl);
            }
        }
    }

    fn get_header(&self) -> Result<HeaderMap> {
        let mut headers = header::HeaderMap::new();

        let token_header_value = HeaderValue::from_str(self.secret.as_str())?;
        headers.insert("mj-api-secret", token_header_value);
        let json_header_value = HeaderValue::from_static("application/json");
        headers.insert(header::CONTENT_TYPE, json_header_value);
        Ok(headers)
    }

    pub async fn submit_image(&self, prompt: String) -> Result<SubmitImageResult> {
        debug!("Submitting image: {}", prompt);
        let url = format!("{}/submit/imagine", self.proxy_url);
        let body = json!({ "prompt": prompt });
        debug!("body: {}", body);

        let result = reqwest::Client::new()
            .post(url.clone())
            .headers(self.get_header()?)
            .json(&body)
            .send()
            .await?
            .json::<SubmitImageResult>()
            .await?;

        Ok(result)
    }

    pub async fn submit_change(&self, params: ImageChangeParams) -> Result<SubmitImageResult> {
        debug!("Submitting change: {:?}", params);
        let url = format!("{}/submit/change", self.proxy_url);
        let result = reqwest::Client::new()
            .post(url.clone())
            .headers(self.get_header()?)
            .json(&params)
            .send()
            .await?
            .json::<SubmitImageResult>()
            .await?;

        Ok(result)
    }

    pub async fn get_job(&self, job_id: String) -> Result<JobStatus> {
        debug!("Getting job: {}", job_id);
        let url = format!("{}//task/{}/fetch", self.proxy_url, job_id);
        let result = reqwest::Client::new()
            .get(url.clone())
            .headers(self.get_header()?)
            .send()
            .await?
            .json::<JobStatus>()
            .await?;

        Ok(result)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitImageResult {
    pub code: i16,
    pub description: String,
    pub result: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobStatus {
    action: String,
    id: String,
    status: String,
    progress: String,
    imageUrl: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImageChangeParams {
    pub action: String,
    pub index: i64,
    pub notify_hook: String,
    pub state: String,
    pub task_id: String,
}

impl ImageChangeParams {
    pub fn new(task_id: String) -> Self {
        Self {
            action: "UPSCALE".to_string(),
            index: 1,
            notify_hook: "".to_string(),
            state: "".to_string(),
            task_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::midjourney::SubmitImageResult;
    #[tokio::test]
    async fn submit_image_test() {
        dotenv::dotenv().ok();
        let url = env::var("MIDJOURNEY_PROXY_RUL").unwrap();
        let secret = env::var("MIDJOURNEY_PROXY_SECRET").unwrap();

        let midjourney = super::Midjourney::new(url, secret);
        let result = midjourney
            .submit_image("一只快乐的小兔子".to_string())
            .await
            .unwrap();
        assert!(result.code == 1);
        assert!(result.description == "提交成功");
    }

    #[tokio::test]
    async fn get_job_test() {
        dotenv::dotenv().ok();
        let url = env::var("MIDJOURNEY_PROXY_RUL").unwrap();
        let secret = env::var("MIDJOURNEY_PROXY_SECRET").unwrap();

        let midjourney = super::Midjourney::new(url, secret);
        let result = midjourney
            .get_job("1691733511960857".to_string())
            .await
            .unwrap();
        assert!(result.action == "UPSCALE");
        assert!(result.id == "1691733511960857");
        assert!(result.status == "SUCCESS");
        assert!(result.progress == "100%");
    }

    #[tokio::test]
    async fn submit_change_test() {
        dotenv::dotenv().ok();
        let url = env::var("MIDJOURNEY_PROXY_RUL").unwrap();
        let secret = env::var("MIDJOURNEY_PROXY_SECRET").unwrap();

        let midjourney = super::Midjourney::new(url, secret);
        let result = midjourney
            .submit_change(super::ImageChangeParams::new(
                "1691741750445654".to_string(),
            ))
            .await
            .unwrap();
        assert!(result.code == 1);
        assert!(result.description == "提交成功");
    }

    #[tokio::test]
    async fn get_first_image() {
        dotenv::dotenv().ok();
        let url = env::var("MIDJOURNEY_PROXY_RUL").unwrap();
        let secret = env::var("MIDJOURNEY_PROXY_SECRET").unwrap();

        let midjourney = super::Midjourney::new(url, secret);
        let image = midjourney
            .get_first_image("黑色的小狗".to_string())
            .await
            .unwrap();
        assert!(image.len() > 0)
    }

    #[tokio::test]
    async fn it_works() {
        let result =
            r#"{"code":1,"description":"提交成功","result":"1691741750445654","properties":{}}"#;
        let result = serde_json::from_str::<SubmitImageResult>(result).unwrap();
        assert!(result.code == 1);
        assert!(result.description == "提交成功");
    }
}
