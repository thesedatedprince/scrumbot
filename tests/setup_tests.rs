use scrumbot;
use std::env;


#[test]
fn test_api_token_returns_env_if_set() {
    env::set_var("SLACK_API_TOKEN", scrumbot::SLACK_API_TOKEN);
    assert_eq!(scrumbot::SLACK_API_TOKEN, &scrumbot::api_token::slack_api_token());
}
