use oauth2::{Client, Token, StandardToken, State, Url};
use oauth2_examples::{config_from_args, listen_for_code};
use reqwest::header::{HeaderName,HeaderValue,ACCEPT,CONTENT_TYPE};


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config_from_args("Spotify Example")?;

    let reqwest_client = reqwest::Client::new();

    let auth_url = Url::parse("https://accounts.spotify.com/authorize")?;
    let token_url = Url::parse("https://accounts.spotify.com/api/token")?;
    let redirect_url = Url::parse("http://localhost:8080/api/auth/redirect")?;

    let mut client = Client::new(config.client_id, auth_url, token_url);
    client.set_client_secret(config.client_secret);
    client.add_scope("user-read-email");
    client.set_redirect_url(redirect_url);

    let state = State::new_random();
    let auth_url = client.authorize_url(&state);

    println!("Browse to: {}", auth_url);

    let received = listen_for_code(8080).await?;

    if received.state != state {
        panic!("CSRF token mismatch :(");
    }

    let token = client
        .exchange_code(received.code)
        .with_client(&reqwest_client)
        .execute::<StandardToken>()
        .await?;

    println!("Token: {:?}", token.access_token().to_string());

    let response = reqwest_client.get("https://api.spotify.com/v1/me")
    .bearer_auth(token.access_token().to_string())
    .header(ACCEPT,HeaderValue::from_bytes(b"application/json").unwrap())
    .header(CONTENT_TYPE,HeaderValue::from_bytes(b"application/json").unwrap())
    .send()
    .await?;

    println!("Status Code: {}", response.status().as_str());
    println!("Full response test: {:?}", response.text().await?);


    Ok(())
}
