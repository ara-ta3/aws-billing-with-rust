use reqwest;
use serde_json::json;
use std::env;

#[tokio::main]
async fn main() {
    let webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    let payload = json!({
        "text": "Hello, World from Rust!"
    });

    match reqwest::Client::new()
        .post(&webhook_url)
        .json(&payload)
        .send()
        .await
    {
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
