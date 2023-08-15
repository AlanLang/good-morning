use std::collections::HashMap;

use anyhow::{Ok, Result};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

pub async fn get_holiday_info(today: &DateTime<Local>) -> Option<HolidayInfo> {
    let holiday = get_holiday_info_from_api(today).await.unwrap();
    let today_str = today.format("%m-%d").to_string();
    let holiday_info = holiday.holiday.get(&today_str);
    match holiday_info {
        Some(info) => Some(info.clone()),
        None => None,
    }
}

async fn get_holiday_info_from_api(today: &DateTime<Local>) -> Result<HolidayResult> {
    let url = format!(
        "https://timor.tech/api/holiday/year/{}",
        today.format("%Y-%m").to_string()
    );
    print!("url: {}", url);
    let body = reqwest::get(url).await?.json::<HolidayResult>().await?;
    Ok(body)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    #[tokio::test]
    async fn get_holiday_info_from_api() {
        // 2023年10月1日
        let today = chrono::Local
            .with_ymd_and_hms(2023, 10, 1, 0, 0, 0)
            .unwrap();
        let holiday = super::get_holiday_info_from_api(&today).await.unwrap();
        assert!(holiday.code == 0);
    }

    #[tokio::test]
    async fn get_holiday_info() {
        // 2023年10月1日
        let today = chrono::Local
            .with_ymd_and_hms(2023, 10, 1, 0, 0, 0)
            .unwrap();
        let holiday = super::get_holiday_info(&today).await.unwrap();
        assert!(holiday.holiday == true);
    }

    #[tokio::test]
    async fn is_not_holiday() {
        // 2023年10月1日
        let today = chrono::Local
            .with_ymd_and_hms(2023, 08, 15, 0, 0, 0)
            .unwrap();
        let holiday = super::get_holiday_info(&today).await;
        assert!(holiday == None)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolidayResult {
    pub code: i64,
    pub holiday: HashMap<String, HolidayInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HolidayInfo {
    pub holiday: bool,
    pub name: String,
    pub wage: i64,
    pub date: String,
    pub rest: i64,
}
