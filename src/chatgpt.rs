extern crate dotenv;
use anyhow::{Ok, Result};
use chatgpt::{prelude::*, types::CompletionResponse};

pub struct Chat {
    client: ChatGPT,
}

impl Chat {
    pub fn new(token: String) -> Result<Self> {
        let client = ChatGPT::new_with_config(
            token,
            ModelConfigurationBuilder::default()
                .engine(ChatGPTEngine::Gpt4)
                .temperature(0.8)
                .build()
                .unwrap(),
        )?;
        Ok(Self { client })
    }

    pub async fn get_today_note(&self, weather: &str) -> Result<String> {
        let prompt = format!("我告诉你今天的星期和天气，你根据生成一句关心我的话，内容可以稍微多一点，但是文字总数不能超多100字，工作日可以让我好好工作，周末了可以让我好好享受周末时光，如果周五了会很开心因为快放假了，语气要温柔可爱，语言中不需要再出现天气的内容，也不要叫我亲爱的。今天是{}", weather);

        let response: CompletionResponse = self.client.send_message(prompt).await?;
        let result = response.message().content.clone();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    #[tokio::test]
    async fn get_today_note_test() {
        dotenv::dotenv().ok();
        let token = env::var("CHATGPT_TOKEN").unwrap();
        let chat = super::Chat::new(token).unwrap();
        let result = chat
            .get_today_note("周一，天气 晴，温度 25 摄氏度")
            .await
            .unwrap();
        println!("-------");
        println!("{}", result);
        println!("-------");
        assert!(result.len() > 0);
    }
}
