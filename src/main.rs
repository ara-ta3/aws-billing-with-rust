use serde_json::Value;
use std::env;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let webhook_url = env::var("SLACK_WEBHOOK_URL").expect("SLACK_WEBHOOK_URL is not set");

    let payload: Value = serde_json::json!({
        "text": "Hello, World from Rust!"
    });
    let response = reqwest::Client::new()
        .post(&webhook_url)
        .json(&payload)
        .send()
        .await?;

    if response.status().is_success() {
        println!("success!");
    } else {
        eprintln!("failed. slack api status: {}", response.status());
    }
    Ok(())
}
