use serde::{Deserialize, Serialize};
use super::common_structs::*;
pub const GET_CURRENT_USER_PROFILE: &str = "https://api.spotify.com/v1/me";
pub const GET_USER_PROFILE: &str = "https://api.spotify.com/v1/users/";
pub const GET_CURRENT_USERS_SAVED_ALBUMS: &str = "https://api.spotify.com/v1/me/albums";
pub const GET_CURRENT_USER_TOP_ARTIST_AND_TRAKS: &str = "https://api.spotify.com/v1/me/top/";
pub const GET_TRACK_BY_ID : &str = "https://api.spotify.com/v1/tracks/";

#[derive(Deserialize, Debug)]
struct Followers
{
    href: Option<String>,
    total: u8,
}
#[derive(Deserialize, Debug)]
pub struct CurrentUserProfile
{
    display_name: String,
    email: String,
    external_urls: ExternalUrls,
    followers: Followers,
    href: Option<String>,
    id: String,
    images: Vec<Option<String>>,
    r#type: Option<String>,
    uri: String,
}
#[derive(Deserialize, Debug)]
pub struct UserProfile
{
    display_name: String,
    external_urls: ExternalUrls,
    followers: Followers,
    href: Option<String>,
    id: String,
    images: Vec<Option<String>>,
    r#type: Option<String>,
    uri: String,
}


