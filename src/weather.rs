use anyhow::{Ok, Result};
use log::debug;
use serde::{Deserialize, Serialize};

pub async fn get_weather(city_code: String) -> Result<WeatherResult> {
    debug!("Getting weather for {}", city_code);
    let url = format!("http://t.weather.sojson.com/api/weather/city/{}", city_code);
    let body = reqwest::get(url).await?.json::<WeatherResult>().await?;
    Ok(body)
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResult {
    pub message: String,
    pub status: i64,
    pub date: String,
    pub time: String,
    pub city_info: CityInfo,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CityInfo {
    pub city: String,
    pub citykey: String,
    pub parent: String,
    pub update_time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub shidu: String,
    pub pm25: f64,
    pub pm10: f64,
    pub quality: String,
    pub wendu: String,
    pub ganmao: String,
    pub forecast: Vec<Forecast>,
    pub yesterday: Yesterday,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub date: String,
    pub high: String,
    pub low: String,
    pub ymd: String,
    pub week: String,
    pub sunrise: String,
    pub sunset: String,
    pub aqi: i64,
    pub fx: String,
    pub fl: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub notice: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Yesterday {
    pub date: String,
    pub high: String,
    pub low: String,
    pub ymd: String,
    pub week: String,
    pub sunrise: String,
    pub sunset: String,
    pub aqi: i64,
    pub fx: String,
    pub fl: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub notice: String,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn it_works() {
        let weather = super::get_weather("101190201".to_string()).await.unwrap();
        // 验证 state = 200
        assert_eq!(weather.status, 200);
        assert_eq!(weather.city_info.city, "无锡市");
        assert_eq!(weather.city_info.parent, "江苏");
    }
}
