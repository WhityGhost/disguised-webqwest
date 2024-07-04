use reqwest_impersonate as reqwest;
use std::error::Error;

use futures_util::{SinkExt, StreamExt, TryStreamExt};
use reqwest::{impersonate::Impersonate, Client, Message};

// tcp url: https://answerthepublic.com
// websocket url: wss://answerthepublic.com/

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Build a client to mimic Chrome123
    let client = reqwest::Client::builder()
        .impersonate(Impersonate::Chrome126)
        .enable_ech_grease()
        .permute_extensions()
        .build()?;

    // Use the API you're already familiar with
    let resp = client.get("https://answerthepublic.com").send().await?;
    println!("Status Code: {}", resp.status().as_str());

    let websocket = Client::builder()
        .impersonate_websocket(Impersonate::Chrome126)
        .build()?
        .get("wss://echo.websocket.org")
        .upgrade()
        .send()
        .await?
        .into_websocket()
        .await?;

    let (mut tx, mut rx) = websocket.split();

    tokio::spawn(async move {
        for i in 1..6 {
            tx.send(Message::Text(format!("Hello, World! #{i}")))
                .await
                .unwrap();
        }
    });

    while let Some(message) = rx.try_next().await? {
        match message {
            Message::Text(text) => println!("received: {text}"),
            _ => {}
        }
    }

    Ok(())
}