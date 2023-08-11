mod chatgpt;
mod midjourney;
mod poetry;
mod weather;
mod wechat;
use crate::{
    chatgpt::Chat,
    wechat::{send_message, MessageInfo},
};
use anyhow::Result;
use chrono::{DateTime, FixedOffset, Local, TimeZone};
use cron_tab::AsyncCron;
use log::{debug, info};
use std::env;
use std::sync::Arc;
use weather::get_weather;

// Use Jemalloc only for musl-64 bits platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    env_logger::init();
    debug!("Starting up");
    let gpt_token = env::var("CHATGPT_TOKEN").unwrap();
    let mj_url = env::var("MIDJOURNEY_PROXY_RUL").unwrap();
    let mj_secret = env::var("MIDJOURNEY_PROXY_SECRET").unwrap();
    let wechat_bot_url = env::var("WECHAT_BOT_URL").unwrap();

    let env = TaskEnv {
        gpt_token,
        mj_url,
        mj_secret,
        wechat_bot_url,
        city_code: "101190201".to_string(),
    };

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
}

async fn run(env: TaskEnv) -> Result<()> {
    let weather = get_weather(env.city_code).await?;
    let poetry = poetry::get_poetry().await?;

    let gpt = Chat::new(env.gpt_token)?;
    let prompt = gpt
        .make_midjourney_prompt_by_poetry(poetry.content.to_string())
        .await?;
    let midjourney = midjourney::Midjourney::new(env.mj_url, env.mj_secret);
    let image = midjourney
        .get_first_image(prompt)
        .await
        .unwrap_or_else(|e| {
            log::error!("生成图片出错: {}", e);
            return "https://vip2.loli.io/2023/08/02/FZS59UEMp7BqoTW.webp".to_string();
        });

    let forecast = &weather.data.forecast[0];
    // 取第一位
    let title = format!(
        "{}天气 {} , 温度 {}°C",
        weather.city_info.city, forecast.type_field, weather.data.wendu
    );
    let description = format!(
        "起床啦，喝杯咖啡，背个单词，去上班。\n今日诗句\n{}",
        poetry.content
    );
    let message = MessageInfo::new(title, description, image);
    info!("message is {:?}", message);
    let _ = send_message(env.wechat_bot_url, message).await?;
    Ok(())
}
