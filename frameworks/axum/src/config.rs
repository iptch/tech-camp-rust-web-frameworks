use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongodb: MongoDB,
}

#[derive(Debug, Deserialize)]
pub struct MongoDB {
    pub host: String,
}
