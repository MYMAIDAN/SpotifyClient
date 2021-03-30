use hyper::header::ACCEPT_ENCODING;
use reqwest::*;
use reqwest::header::{HeaderName,HeaderValue,ACCEPT,CONTENT_TYPE};
use super::authorize::*;
use super::user_profile::*;

pub struct ClientApi {
    token : SpotifyToken,
    client : reqwest::Client,
}

impl ClientApi {
    pub async fn new(client_id : String, client_secret : String) ->Result<ClientApi> {
        let token  = get_auth_token(client_id, client_secret).await;
        let reqwest_client = reqwest::Client::new();
        match token{
            Ok(to) => Ok(ClientApi{token : to, client : reqwest_client}),
            Err(e) => panic!("Cannot authorize"),
        }
    }

    pub async fn get_current_user_profile(&self) -> Result<CurrentUserProfile> 
    {   
        let response  = self.client.get(GET_CURRENT_USER_PROFILE)
                                                .bearer_auth(self.token.to_string())
                                                .header(ACCEPT, HeaderValue::from_bytes(b"application/json").unwrap())
                                                .header(CONTENT_TYPE, HeaderValue::from_bytes(b"application/json").unwrap())
                                                .send()
                                                .await?;


        let user_profile = response.json::<CurrentUserProfile>().await?;
        Ok(user_profile)
    }

    pub async fn get_user_profile(&self, user_id : &str) -> Result<UserProfile>
    {
        let mut user_profile_url = String::from(GET_USER_PROFILE);
        user_profile_url.push_str(user_id);

        let response = self.client.get(user_profile_url)
                                               .bearer_auth(self.token.to_string())
                                               .header(ACCEPT, HeaderValue::from_bytes(b"application/json").unwrap())
                                               .header(CONTENT_TYPE, HeaderValue::from_bytes(b"application/json").unwrap())
                                               .send()
                                               .await?;
        
        let user_profile = response.json::<UserProfile>().await;

        match user_profile
        {
            Err(e) => Err(e),
            Ok(user_profile) => Ok(user_profile),
        }
    }
}