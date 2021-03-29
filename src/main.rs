use oauth2::{Client, Token, StandardToken, State, Url};
use oauth2_examples::{config_from_args, listen_for_code};
use reqwest::header::{HeaderName,HeaderValue,ACCEPT,CONTENT_TYPE};
use serde::{Deserialize, Serialize};

pub mod spotify;
use spotify::spotify_client_api;
use tokio::select_priv_declare_output_enum;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config_from_args("Spotify Example")?;

    let reqwest_client = reqwest::Client::new();

    let spotify_cliet = spotify_client_api::ClientApi::new(config.client_id,config.client_secret).await.unwrap();

    let  config = spotify_cliet.get_current_user_profile().await?;

    println!("Json {:?}", config);
    


    Ok(())
}
