use super::user_profile::*;
use serde::{Deserialize, Serialize};
use super::common_structs::*;

#[derive(Deserialize, Debug)]
pub struct Artist
{
    external_urls: ExternalUrls,
    href: Option<String>,
    id: String,
    name: String,
    r#type: String,
    uri: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Copyrights
{
    text: String,
    r#type: String,
}
#[derive(Deserialize, Debug)]
pub struct ExternalIds
{
    upc: String,
}

#[derive(Deserialize, Debug)]
pub struct TrackItem
{
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    disc_number: u32,
    duration_ms: u64,
    explicit: bool,
    external_urls: ExternalUrls,
    href: Option<String>,
    id: String,
    name: String,
    preview_url: String,
    track_number: u32,
    r#type: String,
    uri: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Track
{
    href: Option<String>,
    items: Vec<TrackItem>,
    limit: u32,
    next: Option<String>,
    offset: u32,
    previous: Option<String>,
    total: u32,
}
#[derive(Deserialize, Debug)]
pub struct AlbumRef
{
    album_type: String,
    artists: Vec<Artist>,
    available_markets: Vec<String>,
    copyrights: Vec<Copyrights>,
    external_ids: ExternalIds,
    external_urls: ExternalUrls,
    genres: Vec<String>,
    href: Option<String>,
    id: String,
    images: Vec<Image>,
    name: String,
    popularity: u32,
    release_date: String,
    release_date_precision: String,
    total_tracks: u32,
    tracks: Track,
    r#type: String,
    uri: Option<String>,
}
#[derive(Deserialize, Debug)]
pub struct Item
{
    added_at: String,
    album: AlbumRef,
}
#[derive(Deserialize, Debug)]
pub struct Album
{
    href: Option<String>,
    items: Vec<Item>,
    limit: u32,
    next: Option<String>,
    offset: u32,
    previous: Option<String>,
    total: u32,
}
