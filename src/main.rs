use scrumbot::api_token;

use slack::RtmClient;


fn main() {
    let api_token = api_token::slack_api_token();
    let client = RtmClient::login(&api_token);
    
    match client {
        Ok(_) => {},
        Err(err) => panic!("{}", err)
    }
}
