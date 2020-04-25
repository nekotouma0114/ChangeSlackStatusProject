use tokio;
use lambda_runtime::{error::HandlerError, lambda, Context};
use simple_error::bail;
use serde_derive::{Serialize, Deserialize};

extern crate my_google_controller;
use my_google_controller::google_calendar;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    first_name: String,
    last_name: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() {
    lambda!(my_handler);
}


fn my_handler(e: CustomEvent, ctx: Context) -> Result<CustomOutput, HandlerError> {
    let http_client = async{
        let str = google_calendar::get_request("https://google.co.jp".to_string());
        println!("{}",str.await);
    };

    tokio::runtime::Runtime::new().unwrap().block_on(http_client);

    Ok(CustomOutput{
        message: format!("Hello, {}!", e.first_name),
    })
}