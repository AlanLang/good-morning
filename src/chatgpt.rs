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

    pub async fn make_midjourney_prompt_by_poetry(&self, poetry: &str) -> Result<String> {
        let prompt = format!("我给你一些汉语古诗词或其他古文的句子，请你仔细在汉语语境下理解该句子，然后将其转换为Midjourney网站绘图所能识别的英文prompt，要求prompt尽可能详细，且其中要体现中国古典山水画的特色。我给你的句子是:{}", poetry);

        let response: CompletionResponse = self.client.send_message(prompt).await?;
        let result = response.message().content.clone();
        Ok(result)
    }

    pub async fn get_today_note(&self, weather: &str) -> Result<String> {
        let prompt = format!("我告诉你今天的星期和天气，你根据生成一句关心我的话，内容可以稍微多一点，工作日可以让我好好工作，周末了可以让我好好享受周末时光，如果周五了会很开心因为快放假了，语气要温柔可爱，语言中不需要再出现天气的内容，也不要叫我亲爱的。今天是::{}", weather);

        let response: CompletionResponse = self.client.send_message(prompt).await?;
        let result = response.message().content.clone();
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    #[tokio::test]
    async fn it_works() {
        dotenv::dotenv().ok();
        let token = env::var("CHATGPT_TOKEN").unwrap();
        let chat = super::Chat::new(token).unwrap();
        let result = chat
            .make_midjourney_prompt_by_poetry("两岸猿声啼不住，轻舟已过万重山")
            .await
            .unwrap();
        println!("-------");
        println!("{}", result);
        println!("-------");
        assert!(result.len() > 0);
    }

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
