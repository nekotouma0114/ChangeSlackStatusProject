use tokio;
use std::fs::File;
use std::io::BufReader;

use lambda_runtime::{error::HandlerError, lambda, Context};
use serde::{Serialize, Deserialize};

extern crate my_google_controller;
extern crate my_slack_controller;
use my_google_controller::google_calendar;
use my_slack_controller::{slack_profile, slack_general};


#[derive(Deserialize, Clone)]
struct CustomEvent {
    event_type: String
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

#[derive(Deserialize, Clone)]
struct MyConfig {
    slack_user_id : String,
    event_var: EventVariable
}

#[derive(Deserialize, Clone)]
struct EventVariable{
    morning: slack_profile::SlackStatus,
    evening: slack_profile::SlackStatus,
    holiday: slack_profile::SlackStatus
}

fn main() {
    lambda!(my_handler);
}


fn my_handler(e: CustomEvent, ctx: Context) -> Result<CustomOutput, HandlerError> {
    let http_client = async{
        let config: MyConfig = serde_json::from_reader(BufReader::new(File::open("./config.json").unwrap())).unwrap();

        let slack_token = slack_general::get_access_token("./slack_secret.json").await;
        let mut profile = slack_profile::get_profile(&slack_token,&config.slack_user_id).await;
        

        let change_status: Option<slack_profile::SlackStatus> = match &*e.event_type.clone() {
            "morning" => {
                let my_event_list = google_calendar::get_today_schedule(&profile.email).await.unwrap();
                Some(if is_today_holiday(&my_event_list).await { config.event_var.holiday } else { config.event_var.morning })
            },
            "evening" => {
                let my_event_list = google_calendar::get_today_schedule(&profile.email).await.unwrap();
                if is_today_holiday(&my_event_list).await {
                    Some (config.event_var.evening)
                } else {
                    None
                }
            },
            "holiday" => {
                Some (config.event_var.holiday)
            },
            unknown => panic!(r#"Unknown EventType: EventType is "{}" "#,unknown)
        };

        match change_status {
            Some(status) => {
                profile.change_status(&status);
                slack_profile::set_profile(&slack_token,profile,&config.slack_user_id).await;
            },
            None => println!("No change status")
        }
    };

    tokio::runtime::Runtime::new().unwrap().block_on(http_client);
    Ok(CustomOutput{
        message: String::from("Success!!"),
    })
}


//Check special or public holidays
async fn is_today_holiday<'a>(personal_event:&'a google_calendar::CalendarEvent )-> bool {
    let japanese_holiday_schedule = google_calendar::get_today_schedule(google_calendar::JAPANESE_HOLIDAY_CALENDAR_ID);
    if !japanese_holiday_schedule.await.unwrap().items.is_empty() { return true; }

    for item in &personal_event.items {
        if item.summary.contains("休"){
            return true;
        }
    }
    false
}