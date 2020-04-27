extern crate serde;
extern crate jsonwebtoken;
extern crate chrono;
extern crate reqwest;

use std::fs::File;
use std::io::BufReader;

use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, Header, Algorithm, EncodingKey,};
use chrono::Local;

//second
const TOKEN_PERIOD: i64  = 300;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessTokenResponse {
    pub access_token: String,
    pub expires_in: i64,
    //pub token_type: String
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
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: i64,
    iat: i64
}

pub async fn get_access_token(secret_path: String,service_uri: String) -> AccessTokenResponse {

    let file = File::open(secret_path).unwrap();
    let auth_info: SecretJson = serde_json::from_reader(BufReader::new(file)).unwrap();
    let token_uri = auth_info.token_uri.clone();

    let jrt = generate_jwt(auth_info,service_uri);

    //let response = 
    let response = reqwest::Client::new()
        .post(&token_uri)
        .form(&[
            ("grant_type","urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion",&jrt)
        ]).send().await.unwrap().text().await.unwrap();
    println!("{}",response);
    serde_json::from_str(&response).unwrap()
}

fn generate_jwt(auth_info: SecretJson, service_uri: String) -> String {
    let claims = Claims::new(auth_info.client_email, auth_info.token_uri, service_uri);
    let secret_key = EncodingKey::from_rsa_pem(str::as_bytes(&auth_info.private_key)).unwrap();
    return match encode(&Header::new(Algorithm::RS256), &claims, &secret_key) {
        Ok(t) => t,
        Err(e) => panic!(e.to_string())
    }
}

impl Claims{
    fn new(email: String, token_uri: String, service_uri: String) -> Self{
        let now: i64 = Local::now().timestamp();
        println!("{}",Local::now());
        Claims {
              iss : email,
              scope : service_uri,
              aud : token_uri,
              iat : now,
              exp : now + TOKEN_PERIOD 
        }
    }
}
