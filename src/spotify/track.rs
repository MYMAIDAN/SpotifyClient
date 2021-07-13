use serde::{Deserialize, Serialize};
use super::common_structs::*;


/// Struct for track api https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-several-tracks
/// This structure for part of API where using market attribute is not empty
/// - available_markest is allowd when market attribute not empty
#[derive(Deserialize, Debug)]
pub struct LocalTrackAlbum
{
    album_type             : String,
    artists                : Vec<Artist>,
    //available_markets : Vec<String>,
    external_urls          : ExternalUrls,
    href                   : Option<String>,
    id                     : String,
    images                 : Vec<Image>,
    name                   : String,
    release_date           : String,
    release_date_precision : String,
    total_tracks           : u32,
    r#type                 : String,
    uri                    : Option<String>,

}

/// Struct for track api https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-several-tracks
/// This structure for part of API where  market attribute default or empty
/// - available_markest is allowd when market attribute empty
#[derive(Deserialize, Debug)]
pub struct GlobalTrackAlbum
{
    album_type             : String,
    artists                : Vec<Artist>,
    available_markets      : Vec<String>,
    external_urls          : ExternalUrls,
    href                   : Option<String>,
    id                     : String,
    images                 : Vec<Image>,
    name                   : String,
    release_date           : String,
    release_date_precision : String,
    total_tracks           : u32,
    r#type                 : String,
    uri                    : Option<String>,
}

/// Struct
#[derive(Deserialize, Debug)]
pub struct LinkedFrom 
{
    external_urls : ExternalUrls,
    href          : Option<String>,
    id            : Option<String>,
    r#type        : String,
    uri           : Option<String>
}

/// Struct for track api https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-several-tracks
/// This structure for part of API where  market attribute not empty
/// - is local is not allowd when market attribute  empty
/// - is playable is not allowd when market attribute  empty
#[derive(Deserialize, Debug)]
pub struct GlobalTrack
{
    album         : GlobalTrackAlbum,
    artists       : Vec<Artist>,
    disc_number   : u32,
    duration_ms   : u32,
    explicit      : bool,
    external_ids  : ExternalIds,
    external_urls : ExternalUrls,
    href          : Option<String>,
    id            : Option<String>,
    is_local      : bool,
    //is_playable : bool,
    //linked_from  : LinkedFrom,
    name          : String,
    popularity    : u32,
    preview_url   : Option<String>,
    track_number  : u32,
    r#type        : String,
    uri           : Option<String>
}

/// Struct for track api https://developer.spotify.com/documentation/web-api/reference/#endpoint-get-several-tracks
/// This structure for part of API where  market attribute not empty
/// - is local is allowd when market attribute not empty
/// - is playable is allowd when market attribute not empty
#[derive(Deserialize, Debug)]
pub struct LocalTrack
{
    album         : LocalTrackAlbum,
    artists       : Vec<Artist>,
    disc_number   : u32,
    duration_ms   : u32,
    explicit      : bool,
    external_ids  : ExternalIds,
    external_urls : ExternalUrls,
    href          : Option<String>,
    id            : Option<String>,
    is_local      : bool,
    is_playable   : bool,
    linked_from   : LinkedFrom,
    name          : String,
    popularity    : u32,
    preview_url   : Option<String>,
    track_number  : u32,
    r#type        : String,
    uri           : Option<String>
}

///! Enum for json deserialize method
#[derive(Deserialize, Debug)]
pub enum Track
{
    eGlobal(GlobalTrack),
    eLocal(LocalTrack),
}