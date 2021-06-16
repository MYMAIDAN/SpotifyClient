use super::user_profile::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct ExternalUrls {
    spotify: String,
}

#[derive(Deserialize, Debug)]
pub struct Followers {
    href: Option<String>,
    total: u32,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    height: u32,
    url: Option<String>,
    width: u32,
}

#[derive(Deserialize, Debug)]
pub struct Items {
    external_urls: ExternalUrls,
    followers: Followers,
    genres: Vec<String>,
    href: Option<String>,
    id: String,
    images: Vec<Images>,
    name: String,
    popularity: u32,
    uri: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct UserTopArtistAndTraks {
    items: Vec<Items>,
    next: Option<String>,
    previous: Option<String>,
    total: u32,
    limit: u32,
    href: Option<String>,
}
