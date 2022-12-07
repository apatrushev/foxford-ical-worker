use anyhow::Result;
use chrono::{DateTime, Duration, Utc};
use serde::Deserialize;
use std::str::FromStr;

fn deserialize_duration_minutes<'de, D>(deserializer: D) -> Result<chrono::Duration, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let minutes = u64::deserialize(deserializer)?;
    Ok(chrono::Duration::minutes(minutes as i64))
}

#[derive(Deserialize)]
pub(crate) struct Course {
    pub starts_at: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_duration_minutes")]
    pub duration: chrono::Duration,
    pub discipline_name: String,
}

#[derive(Deserialize)]
pub(crate) struct Calendar {
    pub course_lessons: Vec<Course>,
    pub coach_lessons: Vec<Course>,
}

pub(crate) async fn fetch_calendar(
    token: &str,
    user: &str,
    date: &DateTime<Utc>,
) -> Result<Calendar> {
    let date_from = date.format("%Y-%m-%d");
    let date_to = (*date + Duration::days(20)).format("%Y-%m-%d");
    let mut url = reqwest::Url::from_str("https://foxford.ru/api/calendar")?;
    let mut form = url::form_urlencoded::Serializer::new(String::new());
    let query = form
        .extend_pairs(vec![
            ("date_from", date_from.to_string()),
            ("date_to", date_to.to_string()),
            ("token", token.to_string()),
            ("user_id", user.to_string()),
        ])
        .finish();
    url.set_query(Some(&query));
    let response = reqwest::Client::new().get(url).send().await?;
    Ok(serde_json::from_str(&response.text().await?)?)
}
