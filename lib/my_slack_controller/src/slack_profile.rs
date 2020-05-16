extern crate serde;
use serde::{Deserialize, Serialize};

extern crate reqwest;
extern crate chrono;

use crate::slack_general;
use reqwest::header;
use slack_general::SlackGeneral;
use chrono::Local;

pub struct SlackProfile {
    user_id: String,
    token: String
}


#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Profile {
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
    profile: Option<Profile>
}

const GET_URI: &str = "https://slack.com/api/users.profile.get";
const SET_URI: &str = "https://slack.com/api/users.profile.set";


impl SlackGeneral for SlackProfile{
    // 
    // Read and return slack access token in config file(json)
    //
    // # Parameters
    // secret_path -> config file path
    //
    fn get_access_token<'a>(secret_path: &'a str) -> slack_general::SlackAccessToken {
        slack_general::get_access_token(secret_path)
    }
}

impl SlackProfile{
    // 
    // Initialize structure
    //
    // # Parameters
    // `secret_path`:  config file path
    // `user_id`: slack's user id
    //
    // # Example
    //
    // let slack_profile = new("./slack_secret.json","xxxxxx");
    //
    pub fn new<'a,'b>(secret_path: &'a str,user_id: &'b str) -> Self {
        SlackProfile{
            token: slack_general::get_access_token(secret_path).token.unwrap(),
            user_id: user_id.to_string()
        }
    }

    //
    // Get and return slack profile
    // profile structure is "profile" in this url example
    // https://api.slack.com/methods/users.profile.get
    //
    pub async fn get_profile(&self) -> Profile {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap());
        
        let response = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build().unwrap()
            .get(GET_URI)
            .query(&[
                ("token", &self.token),
                ("user", &self.user_id)
            ])
            .send().await.unwrap().text().await.unwrap();

        //println!("{}",response);
        get_profile_in_response(&response)
    }

    //
    // Set new slack profile, and return new profile
    // profile structure is "profile" in this url example
    // https://api.slack.com/methods/users.profile.get
    //
    // # Parameters
    //
    // profile: new slack profile
    //
    pub async fn set_profile<'a>(&self, profile: &'a Profile) -> Profile {
        let mut headers = header::HeaderMap::new();
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_str("application/json; charset=utf-8").unwrap());
        headers.insert(header::HeaderName::from_static("x-slack-user"), header::HeaderValue::from_str(&self.user_id).unwrap());
        headers.insert(header::AUTHORIZATION, header::HeaderValue::from_str(&format!("Bearer {}",&self.token)).unwrap());

        let response = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build().unwrap()
            .post(SET_URI)
            .json(&serde_json::json!({"profile": profile}))
            .send().await.unwrap().text().await.unwrap();

        //println!("{}",response);
        get_profile_in_response(&response)
    }

}

impl<'a> Profile{

    //
    // Update own status
    //
    // # Parameters
    //
    // status_info: new status. Set Indefinite period, if this param is None.
    //
    pub fn change_status(&mut self,status_info: &'a SlackStatus){
        self.status_text = status_info.status_text.clone();
        self.status_emoji = status_info.status_emoji.clone();
        self.status_expiration =  match status_info.status_expiration_from_now {
            Some(t) => Local::now().timestamp() + t,
            None => 0
        };
    }
}

//
// Chack slack response.slack_general
// Return profile, if successful. Otherwise raise panic
//
// # Parameters
//
// status_info: slack api response
//
fn get_profile_in_response<'a>(response: &'a str) -> Profile {
    let response_struct: SlackResponse = serde_json::from_str(&response).unwrap();
    if response_struct.ok {
        response_struct.profile.unwrap()
    }else{
        panic!(response_struct.error.unwrap())
    }
}