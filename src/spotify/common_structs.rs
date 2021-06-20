#[derive(Deserialize, Debug)]
pub enum ResponseValue<T>
{
    value(T),
    error(Error),
}

#[derive(Deserialize,Debug)]
pub struct Images 
{
    height : u32,
    url : Option<String>,
    width : u32
}