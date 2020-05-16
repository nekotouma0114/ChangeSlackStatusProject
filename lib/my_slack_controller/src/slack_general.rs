extern crate serde;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize)]
pub struct SlackAccessToken {
    pub token: Option<String>,
    pub bot_token: Option<String>
}

pub trait SlackGeneral {
    // Force implementation of setting of authentication function information.
    // Call `slack_general::get_access_token(secret_path)`
    fn get_access_token<'a>(secret_path: &'a str) -> SlackAccessToken;
}

// Retrun access token in setting file(json)
//
// # Parameters
// secret_path -> config file path
//
//TODO: Support for multiple error types
pub fn get_access_token<'a>(secret_path: &'a str) -> SlackAccessToken {
    let file = File::open(secret_path).unwrap();
    serde_json::from_reader(BufReader::new(file)).unwrap()
}
