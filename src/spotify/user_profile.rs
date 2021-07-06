use serde::{Deserialize, Serialize};
use super::common_structs::*;


#[derive(Deserialize, Debug)]
struct Followers
{
    href  : Option<String>,
    total : u8,
}
#[derive(Deserialize, Debug)]
pub struct CurrentUserProfile
{
    display_name  : String,
    email         : String,
    external_urls : ExternalUrls,
    followers     : Followers,
    href          : Option<String>,
    id            : String,
    images        : Vec<Option<String>>,
    r#type        : Option<String>,
    uri           : String,
}
#[derive(Deserialize, Debug)]
pub struct UserProfile
{
    display_name  : String,
    external_urls : ExternalUrls,
    followers     : Followers,
    href          : Option<String>,
    id            : String,
    images        : Vec<Option<String>>,
    r#type        : Option<String>,
    uri           : String,
}


