use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Error
{
    status: u32,
    message: String,
}

#[derive(Deserialize, Debug)]
pub enum ResponseValue<T>
{
    value(T),
    error(Error),
}

#[derive(Deserialize,Debug)]
pub struct Image 
{
    height : u32,
    url : Option<String>,
    width : u32
}

#[derive(Deserialize, Debug)]
pub struct ExternalUrls
{
    spotify : String,
}

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