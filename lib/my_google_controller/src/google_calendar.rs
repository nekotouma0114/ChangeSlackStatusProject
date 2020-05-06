extern crate reqwest;
extern crate chrono;
extern crate serde;

use crate::google_auth;
use google_auth::AccessTokenResponse;

use reqwest::header;
use chrono::{Utc, Date, FixedOffset};
use serde::{Deserialize, Serialize};

const READONLY_CALENDER_URI: &str = "https://www.googleapis.com/auth/calendar.readonly";
const JST_OFFSET: i32 = 9 * 3600;

////{} => calendar id
//const CALENDAR_EVENT_LIST_URL: &str = "https://www.googleapis.com/calendar/v3/calendars/{}/events";

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEvent{
    pub items: Vec<EventItem>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventItem{
    pub summary: String,
    #[serde(alias = "originalStartTime")]
    pub original_starttime: Option<OriginalStartTime>,
    pub start: EventItemPeriod,
    pub end: EventItemPeriod
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventItemPeriod {
    //unused when all day schedule
    #[serde(alias = "dateTime")]
    pub date_time: Option<String>,
    //unused when not all day schedule
    pub date: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalStartTime{
    #[serde(alias = "dateTime")]
    pub date_time: String
}

//TODO: Support for multiple error types
pub async fn get_oneday_schedule(email: &str, oneday: Date<FixedOffset>) -> Result<CalendarEvent,reqwest::Error> {
    let token:AccessTokenResponse = google_auth::get_access_token("./google_secret.json",READONLY_CALENDER_URI).await?;

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("OAuth {}",token.access_token)).unwrap());
    
    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?
        .get(&format!("https://www.googleapis.com/calendar/v3/calendars/{}/events",email))
        .query(&[
            ("singleEvents","True"),
            ("timeZone","JST"),
            ("timeMin",&oneday.and_hms(0,0,0).to_rfc3339()),
            ("timeMax",&oneday.and_hms(21,59,59).to_rfc3339())
        ])
        .send().await?.text().await?;
    //println!("{}",response);
    Ok(serde_json::from_str(&response).unwrap())
}

pub async fn get_today_schedule(email: &str) -> Result<CalendarEvent,reqwest::Error> {
    //NOTE: Not recommended use `Date<Local>`, if you use other than `UTC`. timezone in AWS Lambda is `UTC`(and environment variable `TZ` is a reserved variable)
    get_oneday_schedule(email, Utc::today().with_timezone(&FixedOffset::east(JST_OFFSET))).await
}