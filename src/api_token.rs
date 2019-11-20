use std::env;
use super::SLACK_API_TOKEN;

pub fn slack_api_token() -> String {
    match env::var(SLACK_API_TOKEN){
        Ok(val) => String::from(val),
        Err(_) => String::from("No api token provided"),
    }
}
