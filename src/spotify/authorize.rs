use oauth2::{Client, Token, StandardToken, State, Url};
use oauth2_examples::{config_from_args, listen_for_code};
use reqwest::header::{HeaderName,HeaderValue,ACCEPT,CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::fmt;

const AUTH_URL : &str = "https://accounts.spotify.com/authorize";
const TOKEN_URL : &str = "https://accounts.spotify.com/api/token";
const REDIRECT_URL : &str = "http://localhost:8080/api/auth/redirect";

pub struct SpotifyToken(String);

impl fmt::Display for SpotifyToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub async fn get_auth_token(client_id: String, client_secret: String) -> Result<SpotifyToken, String> 
{
    let received_client = reqwest::Client::new();
    let auth_url = Url::parse(AUTH_URL).unwrap();
    let token_url = Url::parse(TOKEN_URL).unwrap();
    let redirect_url = Url::parse(REDIRECT_URL).unwrap();
    let mut client = Client::new(client_id, auth_url, token_url);
    client.set_client_secret(client_secret);
    client.add_scope("user-read-email");
    client.set_redirect_url(redirect_url);

    let state = State::new_random();
    let auth_url = client.authorize_url(&state);

    println!("Browse to: {}", auth_url);

    let received = listen_for_code(8080).await.unwrap();

    let token = client.exchange_code(received.code)
                      .with_client(&received_client)
                      .execute::<StandardToken>()
                      .await.unwrap();

    Ok(SpotifyToken(token.access_token().to_string()))                 
}


