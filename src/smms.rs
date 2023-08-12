use anyhow::Result;
use image::io::Reader as ImageReader;
use log::{debug, info};
use reqwest::multipart::{Form, Part};
use reqwest::{
    header::{self, HeaderValue},
    Client,
};
use serde::{Deserialize, Serialize};
use std::io::Cursor;
use uuid::Uuid;
pub async fn upload_image(token: String, image_url: String) -> Result<String> {
    // 首先下载图片
    let req = reqwest::get(image_url).await?;
    let bytes = req.bytes().await?;
    let image = bytes.to_vec();
    let buffer = process_image(image).await?;
    let result = upload_image_to_smms(buffer, token.as_str()).await?;
    anyhow::Ok(result)
}

async fn upload_image_to_smms(buffer: Vec<u8>, token: &str) -> Result<String> {
    debug!("start upload image");
    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, HeaderValue::from_str(token).unwrap());
    headers.insert(header::ACCEPT, HeaderValue::from_static("application/json"));
    let client = Client::new();
    // 创建一个多部分表单
    let file_name = Uuid::new_v4().to_string() + ".jpeg";
    let form = Form::new()
        // 添加文件部分
        .part("smfile", Part::bytes(buffer).file_name(file_name))
        // 添加其他字段部分
        .text("format", "json");
    // 发送请求
    let result = client
        .post("https://sm.ms/api/v2/upload")
        .headers(headers)
        .multipart(form)
        .send()
        .await?
        .text()
        .await?;
    let upload_result = serde_json::from_str::<UploadResult>(&result);
    match upload_result {
        Ok(a) => Ok(a.data.url),
        Err(_) => {
            let result = serde_json::from_str::<UploadAlreadyHave>(&result);
            match result {
                Ok(a) => Ok(a.images),
                Err(e) => {
                    info!("upload image error: {}", e);
                    Err(e.into())
                }
            }
        }
    }
}

async fn process_image(file: Vec<u8>) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(file);
    let mut reader = std::io::BufReader::new(&mut cursor);
    debug!("start process image");
    let image = ImageReader::new(&mut reader)
        .with_guessed_format()?
        .decode()?;
    let mut buffer = Vec::new();
    image
        .resize(
            image.width(),
            image.height(),
            image::imageops::FilterType::Lanczos3,
        )
        .write_to(
            &mut Cursor::new(&mut buffer),
            image::ImageOutputFormat::Jpeg(80),
        )?;
    debug!("file size: {}", buffer.len());
    Ok(buffer)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadResult {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub data: Data,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "file_id")]
    pub file_id: i64,
    pub width: i64,
    pub height: i64,
    pub filename: String,
    pub storename: String,
    pub size: i64,
    pub path: String,
    pub hash: String,
    pub url: String,
    pub delete: String,
    pub page: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadAlreadyHave {
    pub success: bool,
    pub code: String,
    pub message: String,
    pub images: String,
    #[serde(rename = "RequestId")]
    pub request_id: String,
}

#[cfg(test)]
mod tests {
    use std::env;
    #[tokio::test]
    async fn it_works() {
        dotenv::dotenv().ok();
        let token = env::var("SMMS_TOKEN").unwrap();
        let result = super::upload_image(
            token,
            "https://cdn.discordapp.com/attachments/1139371949500416054/1139542141383749695/alanlang_A_figure_not_startled_by_the_wine_in_a_deep_spring_sle_e9ba4ea6-de72-4f01-a9d8-f979d798939c.png".to_string(),
        )
        .await
        .unwrap();
        // result end with webp
        assert!(result.ends_with(".webp"));
    }
}
