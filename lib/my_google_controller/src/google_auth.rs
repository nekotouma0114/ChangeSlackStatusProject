extern crate serde;
extern crate jsonwebtoken;
extern crate chrono;
extern crate reqwest;

use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey};
use chrono::Local;

//second
const TOKEN_PERIOD: i64  = 300;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    //pub token_type: &str
}

#[derive(Debug, Serialize, Deserialize)]
struct SecretJson{
  //type: String,
  //project_id: String,
  //private_key_id: String,
  private_key: String,
  client_email: String,
  client_id: String,
  //auth_uri: String,
  token_uri: String,
  //auth_provider_x509_cert_url: String,
  //client_x509_cert_url: String
}


#[derive(Debug, Serialize, Deserialize)]
struct Claims<'a> {
    iss: &'a str,
    scope: &'a str,
    aud: &'a str,
    exp: i64,
    iat: i64
}

//TODO: Support for multiple error types
pub async fn get_access_token<'a>(secret_path: &'a str,service_uri: &'a str) -> Result<AccessTokenResponse,reqwest::Error> {

    let file = File::open(secret_path).unwrap();
    let auth_info: SecretJson = serde_json::from_reader(BufReader::new(file)).unwrap();

    let jrt = generate_jwt(&auth_info,service_uri);

    let response = reqwest::Client::new()
        .post(&auth_info.token_uri)
        .form(&[
            ("grant_type","urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion",&jrt)
        ]).send().await?.text().await?;
    
    Ok(serde_json::from_str(&response).unwrap())
}

fn generate_jwt<'a>(auth_info: &'a SecretJson, service_uri: &'a str) -> String {
    let claims = Claims::new(&auth_info.client_email, &auth_info.token_uri, &service_uri);
    let secret_key = EncodingKey::from_rsa_pem(str::as_bytes(&auth_info.private_key)).unwrap();
    
    match encode(&Header::new(Algorithm::RS256), &claims, &secret_key) {
        Ok(t) => t,
        Err(e) => panic!(e)
    }
}

impl<'a> Claims<'a>{
    fn new(email: &'a str, token_uri: &'a str, service_uri: &'a str) -> Self{
        let now: i64 = Local::now().timestamp();

        Claims {
              iss : email,
              scope : service_uri,
              aud : token_uri,
              iat : now,
              exp : now + TOKEN_PERIOD 
        }
    }
}
