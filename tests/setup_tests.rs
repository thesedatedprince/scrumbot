use scrumbot;
use std::env;


#[test]
fn test_api_token_returns_env_if_set() {
    env::set_var(scrumbot::SLACK_API_TOKEN, "TEST_STRING");
    assert_eq!("TEST_STRING", &scrumbot::api_token::slack_api_token());
}

#[test]
fn test_api_token_returns_error_message_if_unset() {
    env::remove_var(scrumbot::SLACK_API_TOKEN);
    assert_eq!("No api token provided", &scrumbot::api_token::slack_api_token());
}
