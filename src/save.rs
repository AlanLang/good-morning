use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedResult {
    pub poetry: String,
    pub author: String,
    pub img_url: String,
}

pub fn save(poetry: &str, author: &str, img_url: &str) -> Result<()> {
    let saved_file: &str = "./saved.json";
    let mut saved: Vec<SavedResult> = match std::fs::read_to_string(saved_file) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => vec![],
    };
    saved.push(SavedResult {
        poetry: poetry.to_string(),
        author: author.to_string(),
        img_url: img_url.to_string(),
    });
    std::fs::write(saved_file, serde_json::to_string(&saved)?)?;
    Ok(())
}

pub async fn download_image(url: &str) -> Result<()> {
    let req = reqwest::get(url).await?;
    let bytes = req.bytes().await?;
    let image = bytes.to_vec();
    // get file name from url
    let file_name = url.split('/').last().unwrap_or("image.png");
    // 如果没有 images 目录则创建
    let dir = std::path::Path::new("./images");
    if !dir.exists() {
        std::fs::create_dir(dir)?;
    }
    // 下载到 images 目录
    let file_name = format!("./images/{}", file_name);
    std::fs::write(file_name, image)?;
    Ok(())
}

#[test]
fn test_save() -> Result<(), Box<dyn std::error::Error>> {
    let poetry = "Roses are red, violets are blue";
    let img_url = "https://example.com/image.png";
    let author = "author";
    let expected_saved = vec![SavedResult {
        poetry: poetry.to_string(),
        author: author.to_string(),
        img_url: img_url.to_string(),
    }];

    // Call the save function
    save(poetry, author, img_url)?;

    // Read the saved file and deserialize its contents
    let saved_file = "./saved.json";
    let saved_content = std::fs::read_to_string(saved_file)?;
    let saved: Vec<SavedResult> = serde_json::from_str(&saved_content)?;

    // Check that the saved content matches the expected content
    assert_eq!(saved, expected_saved);
    std::fs::remove_file(saved_file)?;
    Ok(())
}
