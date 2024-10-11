use serde::Deserialize;

fn default_collection() -> String {
    return "texts".to_string()
}

fn default_database() -> String {
    return "db".to_string()
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongodb: MongoDB,
}

#[derive(Debug, Deserialize)]
pub struct MongoDB {
    pub host: String,
    #[serde(default = "default_collection")]
    pub collection: String,
    #[serde(default = "default_database")]
    pub database: String,
}
