use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
pub struct Config {
    pub general: General,
    pub repositories: Repositories,
}

#[derive(Deserialize)]
pub struct General {
    pub bind: Ipv4Addr,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct RepoStruct {
    pub user: Option<String>,
    pub provider: Option<String>,
    pub repo: String,
    pub key: String,
}

#[derive(Deserialize)]
pub struct Repositories {
    pub source: Vec<RepoStruct>,
    pub target: Vec<RepoStruct>,
}

pub fn read_config() -> std::io::Result<Config> {
    let confg = std::fs::read_to_string("grok.conf")?;
    Ok(toml::from_str(&confg)?)
}
