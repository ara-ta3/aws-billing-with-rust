use serde_json::Value;
use std::env;
use std::result::Result;

#[tokio::main]
async fn main() {
    let webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    let payload: Value = serde_json::json!({
        "text": "Hello, World from Rust!"
    });
    let response: Result<reqwest::Response, reqwest::Error> = reqwest::Client::new()
        .post(&webhook_url)
        .json(&payload)
        .send()
        .await;

    match response {
        Ok(response) => {
            if response.status().is_success() {
                println!("success!");
            } else {
                eprintln!("failed. slack api status: {}", response.status());
            }
        }
        Err(e) => {
            eprintln!("failed. error: {}", e);
        }
    }
}
