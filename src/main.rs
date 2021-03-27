use oauth2::{Client, Token, StandardToken, State, Url};
use oauth2_examples::{config_from_args, listen_for_code};
use reqwest::header::{HeaderName,HeaderValue,ACCEPT,CONTENT_TYPE};
use serde::{Deserialize, Serialize};

pub mod spotify;
use spotify::autorize::*;

#[derive(Deserialize)]
#[derive(Debug)]
struct ExternalUrls
{
    spotify : String
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Followers{
    href : Option<String>,
    total: u8
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Config {
    display_name: String,
    email: String,
    external_urls : ExternalUrls,
    followers: Followers,
    href : Option<String>,
    id: String,
    images: Vec<Option<String>>,
    r#type: Option<String>,
    uri: String,
}



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config_from_args("Spotify Example")?;

    let reqwest_client = reqwest::Client::new();

    let token = spotify::autorize::get_auto_token(config.client_id, config.client_secret).await.unwrap();

    println!("Token: {}", token.to_string());

    let response = reqwest_client.get("https://api.spotify.com/v1/me")
    .bearer_auth(token.to_string())
    .header(ACCEPT,HeaderValue::from_bytes(b"application/json").unwrap())
    .header(CONTENT_TYPE,HeaderValue::from_bytes(b"application/json").unwrap())
    .send()
    .await?;



    println!("Status Code: {}", response.status().as_str());
    //println!("Full response test: {:?}", response.text().await?);
    let config = response.json::<Config>().await?;

    println!("Json {:?}", config);
    


    Ok(())
}
