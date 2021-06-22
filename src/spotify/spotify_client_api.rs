use super::album::{Album};
use super::authorize::*;
use super::user_profile::*;
use super::user_top_artist_track::*;
use hyper::header::ACCEPT_ENCODING;
use reqwest::header::{HeaderName, HeaderValue, ACCEPT, CONTENT_TYPE};
use super::common_structs::*;
use super::track::{Track};
use reqwest::*;
pub struct ClientApi
{
    token: SpotifyToken,
    client: reqwest::Client,
}
impl ClientApi
{
    async fn get(&self, url: &str) -> Result<reqwest::Response>
    {
               println!("URL {:?}", url);
        self.client
            .get(url)
            .bearer_auth(self.token.to_string())
            .header(ACCEPT, HeaderValue::from_bytes(b"application/json").unwrap())
            .header(CONTENT_TYPE, HeaderValue::from_bytes(b"application/json").unwrap())
            .send()
            .await
    }

    pub async fn new(client_id: String, client_secret: String) -> Result<ClientApi>
    {
        let token = get_auth_token(client_id, client_secret).await;
        let reqwest_client = reqwest::Client::new();
        match token {
            Ok(to) => Ok(ClientApi { token: to,
                                     client: reqwest_client }),
            Err(e) => panic!("Cannot authorize"),
        }
    }
    pub async fn get_current_user_profile(&self) -> Result<CurrentUserProfile>
    {
        let response = ClientApi::get(self,GET_CURRENT_USER_PROFILE).await?;

        println!("USER URL {:?}", response);
        
        let user_profile = response.json::<CurrentUserProfile>().await?;
        Ok(user_profile)
    }
    pub async fn get_user_profile(&self, user_id: &str) -> Result<ResponseValue<UserProfile>>
    {
        let mut user_profile_url = String::from(GET_USER_PROFILE);
        user_profile_url.push_str(user_id);
        let response = ClientApi::get(self, &user_profile_url ).await?;

        let res = response.json::<ResponseValue<UserProfile>>().await?;
        Ok(res)
    }
    pub async fn get_current_users_saved_albums(&self,
                                                limit: u32,
                                                offset: usize,
                                                market: &str)
                                                -> Result<Album>
    {
        let mut url = String::from(GET_CURRENT_USERS_SAVED_ALBUMS);
        url.push_str("?");
        url.push_str("limit=1");
        println!("USER URL {:?}", url);
        let response = ClientApi::get(self, &url ).await?;

        //println!("Json {:?}",response);
        let res = response.json::<Album>().await?;
        //Ok(Album{})
        Ok(res)
    }
    pub async fn get_current_user_top_artist(&self,
                                             time_range: &str,
                                             limit: u32,
                                             offset: u32)
                                             -> Result<UserTopArtistAndTraks>
    {
        let mut url = String::from(GET_CURRENT_USER_TOP_ARTIST_AND_TRAKS);
        url.push_str("artists");
        url.push_str("?");
        url.push_str("limit=1");
        println!("USER URL {:?}", url);
        let response = ClientApi::get(self, &url ).await?;

        println!("Json {:?}", response);
        let res = response.json::<UserTopArtistAndTraks>().await?;
        Ok(res)
    }
    pub async fn get_current_user_top_traks(&self,
                                            time_range: &str,
                                            limit: u32,
                                            offset: u32)
                                            -> Result<UserTopArtistAndTraks>
    {
        let mut url = String::from(GET_CURRENT_USER_TOP_ARTIST_AND_TRAKS);
        url.push_str("tracks");
        url.push_str("?");
        url.push_str("limit=1");
        println!("USER URL {:?}", url);
        let response = ClientApi::get(self, &url).await?;

        println!("Json {:?}", response);
        let res = response.json::<UserTopArtistAndTraks>().await?;
        Ok(res)
    }

    pub async fn get_track(&self, id : &str, market : &str ) -> Result<Track>
    {
        let mut url = String::from(GET_TRACK_BY_ID);
        url.push_str(id);

        println!("USER URL {:?}", url);
        let response = ClientApi::get(self, &url).await?;

        println!("Json {:?}", response);
        let res = response.json::<Track>().await?;
        Ok(res)
    }


}
