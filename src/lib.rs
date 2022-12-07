use anyhow::{anyhow, Result};
use chrono::{DateTime, Datelike, Duration, Utc};
use icalendar::{Calendar, Component, Event, EventLike};
mod foxford;
mod utils;

async fn handler(token: &str, user: &str, date: &DateTime<Utc>) -> Result<String> {
    let fcal = foxford::fetch_calendar(token, user, date).await?;
    let events = fcal.course_lessons.iter().chain(fcal.coach_lessons.iter()).map(|lesson| {
        Event::new()
            .summary(&lesson.discipline_name)
            .starts(lesson.starts_at)
            .ends(lesson.starts_at + lesson.duration)
            .done()
    });
    let ical: Calendar = events.collect();
    Ok(ical.to_string())
}

fn start_date() -> DateTime<Utc> {
    let now = Utc::now();
    now - Duration::days((now.weekday().num_days_from_monday() + 7) as i64)
}

#[worker::event(fetch)]
pub async fn main(
    req: worker::Request,
    env: worker::Env,
    _ctx: worker::Context,
) -> Result<worker::Response> {
    utils::log_request(&req);
    worker::Router::new()
        .get_async("/:token/:user", |_req, ctx| async move {
            match (ctx.param("token"), ctx.param("user")) {
                (Some(token), Some(user)) => match handler(token, user, &start_date()).await {
                    Ok(ical) => {
                        let response = worker::Response::ok(ical)?;
                        let mut headers = worker::Headers::new();
                        headers.set("content-type", "text/calendar")?;
                        Ok(response.with_headers(headers))
                    }
                    Err(e) => worker::Response::error(e.to_string(), 500),
                },
                _ => worker::Response::error("Bad request", 400),
            }
        })
        .run(req, env)
        .await
        .map_err(|e| anyhow!("{}", e))
}
