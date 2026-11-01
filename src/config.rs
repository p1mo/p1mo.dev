use crate::Error;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i16,
}

pub fn load_config<P>(path: P) -> Result<Config, Error>
where 
    P: AsRef<std::path::Path>
{
    Ok(serde_json::from_slice(std::fs::read(path.as_ref())?.as_slice())?)
}