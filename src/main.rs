mod chatgpt;
mod midjourney;
mod poetry;
mod smms;
mod weather;
mod wechat;
use crate::{
    chatgpt::Chat,
    smms::upload_image,
    wechat::{send_message, MessageInfo},
};
use anyhow::Result;
use chrono::{DateTime, Datelike, FixedOffset, Local, TimeZone, Weekday};
use cron_tab::AsyncCron;
use log::{debug, info};
use std::env;
use std::sync::Arc;
use weather::get_weather;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    debug!("Starting up");
    let gpt_token = env::var("CHATGPT_TOKEN")
        .map_err(|e| format!("无法获取 CHATGPT_TOKEN 环境变量: {}", e))
        .unwrap();
    let mj_url = env::var("MIDJOURNEY_PROXY_RUL")
        .map_err(|e| format!("无法获取 MIDJOURNEY_PROXY_RUL 环境变量: {}", e.to_string()))
        .unwrap();
    let mj_secret = env::var("MIDJOURNEY_PROXY_SECRET")
        .map_err(|e| {
            format!(
                "无法获取 MIDJOURNEY_PROXY_SECRET 环境变量: {}",
                e.to_string()
            )
        })
        .unwrap();
    let wechat_bot_url = env::var("WECHAT_BOT_URL")
        .map_err(|e| format!("无法获取 WECHAT_BOT_URL 环境变量: {}", e.to_string()))
        .unwrap();
    let smms_token = env::var("SMMS_TOKEN").ok();

    let env = TaskEnv {
        gpt_token,
        mj_url,
        mj_secret,
        wechat_bot_url,
        city_code: "101190201".to_string(),
        smms_token,
    };

    run(env.clone()).await.unwrap();

    let local = Local::from_offset(&FixedOffset::east(8));
    let mut cron = AsyncCron::new(local);
    let current_datetime: DateTime<Local> = local.timestamp(Local::now().timestamp(), 0);

    debug!("当前时间: {}", current_datetime.format("%Y-%m-%d %H:%M:%S"));

    cron.start().await;
    let env = Arc::new(env);

    let _ = cron
        .add_fn("0 0 9 * * ?", move || {
            info!("开始执行任务");
            let env = env.clone();
            async move {
                run(env.as_ref().clone()).await.unwrap();
            }
        })
        .await;

    std::thread::sleep(std::time::Duration::from_secs(u64::MAX));
    // stop cron
    cron.stop();
}

#[derive(Default, Debug, Clone, PartialEq)]
struct TaskEnv {
    gpt_token: String,
    mj_url: String,
    mj_secret: String,
    wechat_bot_url: String,
    city_code: String,
    smms_token: Option<String>,
}

async fn run(env: TaskEnv) -> Result<()> {
    let weather = get_weather(env.city_code).await?;
    let poetry = poetry::get_poetry().await?;

    let gpt = Chat::new(env.gpt_token)?;
    let prompt = gpt
        .make_midjourney_prompt_by_poetry(poetry.content.to_string())
        .await?;
    let midjourney = midjourney::Midjourney::new(env.mj_url, env.mj_secret);
    let mut image = midjourney
        .get_first_image(prompt)
        .await
        .unwrap_or_else(|e| {
            log::error!("生成图片出错: {}", e);
            return "https://vip2.loli.io/2023/08/02/FZS59UEMp7BqoTW.webp".to_string();
        });

    if let Some(token) = env.smms_token {
        image = upload_image(token, image).await?;
    }

    let forecast = &weather.data.forecast[0];
    let today = Local::today();
    let weekday = today.weekday();
    let chinese_weekday = match weekday {
        Weekday::Mon => "周一",
        Weekday::Tue => "周二",
        Weekday::Wed => "周三",
        Weekday::Thu => "周四",
        Weekday::Fri => "周五",
        Weekday::Sat => "周六",
        Weekday::Sun => "周日",
    };
    // 取第一位
    let title = format!(
        "{} 天气 {} , 温度 {}°C",
        chinese_weekday, forecast.type_field, weather.data.wendu
    );
    let today_note = gpt.get_today_note(title.clone()).await?;

    let description = format!(
        "{}\n\n今日诗句\n{}\n---{}",
        today_note, poetry.content, poetry.author
    );
    let message = MessageInfo::new(title, description, image);
    info!("message is {:?}", message);
    let _ = send_message(env.wechat_bot_url, message).await?;
    Ok(())
}
