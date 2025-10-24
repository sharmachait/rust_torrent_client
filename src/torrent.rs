use serde::Deserialize;
use crate::hashes;

#[derive(Debug,Clone,Deserialize)]
pub struct Torrent {
    // #[serde(deserialize_with = "deserialize_url")]
    // announce: reqwest::Url,
    pub announce: String,
    pub info: Info,
}

fn deserialize_url<'de, D>(deserializer: D) -> Result<reqwest::Url, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    reqwest::Url::parse(&s).map_err(serde::de::Error::custom)
}

#[derive(Debug,Clone,Deserialize)]
pub struct Info{
    name: String,
    #[serde(rename="piece length")]
    plength:usize,
    pieces: hashes::Hashes,
    #[serde(flatten)] // kinda like @Embedded in java spring boot
    pub key:Keys
}

#[derive(Debug,Clone,Deserialize)]
#[serde(untagged)]
pub enum Keys{
    SingleFile {length:usize},
    MultiFile {files: Vec<File>},
}
#[derive(Debug,Clone,Deserialize)]
pub struct File{
    pub length: usize,
    paths: Vec<String>
}
