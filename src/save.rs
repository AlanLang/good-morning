use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedResult {
    pub poetry: String,
    pub img_url: String,
}

pub fn save(poetry: &str, img_url: &str) -> Result<()> {
    let saved_file: &str = "./saved.json";
    let mut saved: Vec<SavedResult> = match std::fs::read_to_string(saved_file) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => vec![],
    };
    saved.push(SavedResult {
        poetry: poetry.to_string(),
        img_url: img_url.to_string(),
    });
    std::fs::write(saved_file, serde_json::to_string(&saved)?)?;
    Ok(())
}

#[test]
fn test_save() -> Result<(), Box<dyn std::error::Error>> {
    let poetry = "Roses are red, violets are blue";
    let img_url = "https://example.com/image.png";
    let expected_saved = vec![SavedResult {
        poetry: poetry.to_string(),
        img_url: img_url.to_string(),
    }];

    // Call the save function
    save(poetry, img_url)?;

    // Read the saved file and deserialize its contents
    let saved_file = "./saved.json";
    let saved_content = std::fs::read_to_string(saved_file)?;
    let saved: Vec<SavedResult> = serde_json::from_str(&saved_content)?;

    // Check that the saved content matches the expected content
    assert_eq!(saved, expected_saved);
    std::fs::remove_file(saved_file)?;
    Ok(())
}
