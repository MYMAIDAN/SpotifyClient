use serde::{Deserialize, Serialize};

pub const GET_CURRENT_USER_PROFILE: &str = "https://api.spotify.com/v1/me";

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
pub struct UserProfile {
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
