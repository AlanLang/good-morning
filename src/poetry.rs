use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Poetry {
    pub poetry: String,
    pub author: String,
    pub img_url: String,
}

pub fn get_poetry() -> anyhow::Result<Poetry> {
    let file = File::open("./poetry.json").expect("cannot open poetry file");
    let reader = BufReader::new(file);
    let data: Vec<Poetry> = serde_json::from_reader(reader)?;

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..data.len());

    // 获取随机的数组项的引用
    let random_item_ref = data
        .get(random_index)
        .ok_or(anyhow::anyhow!("Index out of range"))?;
    Ok(random_item_ref.clone()) // 返回项的克隆
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        let poetry = super::get_poetry().unwrap();
    }
}
