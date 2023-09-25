use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub tcc_email: String,
    pub tcc_password: String,
    pub max_temperature: f32,
    pub interval_sec: u64,
}

impl Config {
    pub fn from_env() -> color_eyre::Result<Self> {
        Ok(envy::from_env()?)
    }
}
