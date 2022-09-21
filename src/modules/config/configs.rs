
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Configs {
    pub server: Server
}
#[derive(Debug, Deserialize)]
pub struct Server {
    pub address: String,
    pub port: u16,
}