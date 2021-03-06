extern crate serde;
use serde::{Deserialize, Serialize};
extern crate reqwest;
extern crate chrono;

use crate::slack_general;
use reqwest::header;
use slack_general::SlackAccessToken;
use chrono::Local;

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackProfile {
    pub email: String,
    pub status_text: String,
    pub status_emoji: String,
    pub status_expiration: i64
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct SlackStatus{
    pub status_text: String,
    pub status_emoji: String,
    pub status_expiration_from_now: Option<i64>
}

#[derive(Debug, Serialize, Deserialize)]
struct SlackResponse {
    ok: bool,
    error: Option<String>,
    profile: Option<SlackProfile>

}

const GET_URI: &str = "https://slack.com/api/users.profile.get";
const SET_URI: &str = "https://slack.com/api/users.profile.set";


pub async fn get_profile<'a>(token: &'a SlackAccessToken, user_id: &'a str) -> SlackProfile {

    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
    
    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build().unwrap()
        .get(GET_URI)
        .query(&[
            ("token", token.token.as_ref().unwrap()),
            ("user", &user_id.to_string())
        ])
        .send().await.unwrap().text().await.unwrap();

    get_profile_in_response(&response)
}


pub async fn set_profile<'a>(tokens: &'a SlackAccessToken, profile: SlackProfile,user_id: &'a str) -> SlackProfile {

    let mut headers = header::HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_str("application/json; charset=utf-8").unwrap());
    headers.insert(header::HeaderName::from_static("x-slack-user"), header::HeaderValue::from_str(user_id).unwrap());
    headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}",tokens.token.as_ref().unwrap())).unwrap());

    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build().unwrap()
        .post(SET_URI)
        .json(&serde_json::json!({"profile": profile}))
        .send().await.unwrap().text().await.unwrap();


    //println!("{}",response);
    get_profile_in_response(&response)
}

fn get_profile_in_response<'a>(response: &'a str) -> SlackProfile {
    let response_struct: SlackResponse = serde_json::from_str(&response).unwrap();
    if response_struct.ok {
        response_struct.profile.unwrap()
    }else{
        panic!(response_struct.error.unwrap())
    }
}

impl<'a> SlackProfile{
    pub fn change_status(&mut self,status_info: &'a SlackStatus){
        self.status_text = status_info.status_text.clone();
        self.status_emoji = status_info.status_emoji.clone();
        self.status_expiration =  match status_info.status_expiration_from_now {
            Some(t) => Local::now().timestamp() + t,
            None => 0
        };
    }

    pub async fn get_access_token(secret_path: &'a str) -> SlackAccessToken {
        slack_general::get_access_token(secret_path).await
    }
}