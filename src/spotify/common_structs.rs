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