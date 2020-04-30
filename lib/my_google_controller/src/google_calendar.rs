extern crate reqwest;
extern crate chrono;
extern crate serde;

use crate::google_auth;
use google_auth::AccessTokenResponse;

use reqwest::header;
use chrono::{Local, Date};
use serde::{Deserialize, Serialize};

const READONLY_CALENDER_URI: &str = "https://www.googleapis.com/auth/calendar.readonly";

////{} => calendar id
//const CALENDAR_EVENT_LIST_URL: &str = "https://www.googleapis.com/calendar/v3/calendars/{}/events";

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarEvent{
    pub items: Vec<EventItem>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventItem{
    pub summary: String,
    pub originalStartTime: Option<OriginalStartTime>,
    pub start: EventItemPeriod,
    pub end: EventItemPeriod
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventItemPeriod {
    //unused when all day schedule
    pub dateTime: Option<String>,
    //unused when not all day schedule
    pub date: Option<String>
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OriginalStartTime{
    pub dateTime: String
}

//TODO: Support for multiple error types
pub async fn get_oneday_schedule(email: &str, oneday: Date<Local>) -> Result<CalendarEvent,reqwest::Error> {
    let token:AccessTokenResponse = google_auth::get_access_token("./secret.json",READONLY_CALENDER_URI).await?;

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("OAuth {}",token.access_token)).unwrap());
    
    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()?
        .get(&format!("https://www.googleapis.com/calendar/v3/calendars/{}/events",email))
        .query(&[
            ("timeZone","JST"),
            ("timeMin",&oneday.and_hms(0,0,0).to_rfc3339()),
            ("timeMax",&oneday.and_hms(21,59,59).to_rfc3339())
        ])
        .send().await?.text().await?;
    //println!("{}",response);
    Ok(serde_json::from_str(&response).unwrap())
}

pub async fn get_today_schedule(email: &str) -> Result<CalendarEvent,reqwest::Error> {
    get_oneday_schedule(email, Local::today()).await
}