
use serde::{Deserialize, Serialize};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response, Url};

#[derive(Deserialize, Serialize)]
struct AccessToken {
    access_token: String,
    // Additional fields in the access token response
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "2f0d27d3267c44fca254e7e4f74ff44b";
    let client_secret = "6fdb28d5508a4c5da3ca064a6a7ae2ae";

    let client = Client::new();
    let params = [
        ("grant_type", "client_credentials"),
        // Additional parameters
    ];

    let res = client
        .post("https://accounts.spotify.com/api/token")
        .basic_auth(client_id, Some(client_secret))
        .form(&params)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let access_token = res["access_token"].as_str().unwrap();

    let url = Url::parse("https://api.spotify.com/v1/me/player/play")?;
    let mut headers = HeaderMap::new();
    headers.insert(
        reqwest::header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", access_token))?,
    );

    let params = [
        ("uris", "spotify:track:6rqhFgbbKwnb9MLmUQDhG6"),
        // Additional parameters
    ];

    let res: Response = client
        .put(url)
        .headers(headers)
        .json(&params)
        .send()
        .await?;

    let response_text = res.text().await?;
    println!("Response: {:?}", response_text);

    Ok(())
}