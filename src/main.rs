use oauth2::{Client, StandardToken, State, Token, Url};
use oauth2_examples::{config_from_args, listen_for_code};
use reqwest::header::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
pub mod spotify;
use spotify::spotify_client_api;
use tokio::select_priv_declare_output_enum;
#[tokio::main]
async fn main() -> anyhow::Result<()>
{
    let config = config_from_args("Spotify Example")?;
    let reqwest_client = reqwest::Client::new();
    let spotify_cliet =
        spotify_client_api::ClientApi::new(config.client_id, config.client_secret).await
                                                                                  .unwrap();
    let config = spotify_cliet.get_current_user_profile().await?;
    println!("Json {:?}", config);
    //let user = spotify_cliet.get_user_profile("sdvdfqwe123`12").await?;
    let album = spotify_cliet.get_current_users_saved_albums(1, 0, "")
                             .await?;
    println!("User Profile {:?}", album);
    let top = spotify_cliet.get_current_user_top_artist("", 0, 1).await?;
    println!("Top {:?}", top);

    let track  = spotify_cliet.get_track("11dFghVXANMlKmJXsNCbNl",Some("ES")).await?;
    println!("Track {:?}", track);
    Ok(())
}
