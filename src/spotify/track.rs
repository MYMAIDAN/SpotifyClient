use serde::{Deserialize, Serialize};
use super::common_structs::*;

#[derive(Deserialize, Debug)]
pub struct TrackAlbum
{
    album_type : String,
    artists : Vec<Artist>,
    available_markets : Vec<String>,
    external_urls : ExternalUrls,
    href : Option<String>,
    id : String,
    images : Vec<Image>,
    name : String,
    release_date : String,
    release_date_precision : String,
    total_tracks : u32,
    r#type : String,
    uri: Option<String>,

}

#[derive(Deserialize, Debug)]
pub struct Track
{
    album : Vec<TrackAlbum>,
    artists : Vec<TrackItem>
}