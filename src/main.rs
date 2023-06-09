use rspotify::client::Spotify;
use rspotify::oauth2::{CredentialsBuilder, OAuthBuilder};

#[tokio::main]
async fn main() {
    let creds = CredentialsBuilder::default()
        .client_id("2f0d27d3267c44fca254e7e4f74ff44b")
        .client_secret("6fdb28d5508a4c5da3ca064a6a7ae2ae")
        .build()
        .unwrap();

    let oauth = OAuthBuilder::default()
        .scope("user-read-private")
        .build()
        .unwrap();

    let mut spotify = Spotify::default()
        .credentials(creds)
        .oauth(oauth)
        .build()
        .unwrap();

    spotify.prompt_for_user_token().await.unwrap();

    let me = spotify.me().await.unwrap();
    println!("Hello, {}!", me.display_name.unwrap());
}
