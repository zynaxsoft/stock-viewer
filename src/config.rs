use std::fs;

use serde_derive::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Stock {
    pub name: String,
    pub url: String,
    pub threshold: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub stocks: Vec<Stock>,
}

impl Config {
    pub fn from_file(file: &str) -> Self {
        let toml_string = fs::read_to_string(file).expect(
            "Error when trying to read anime.toml file.\
                Make sure it is on the current directory.",
        );
        toml::from_str(&toml_string).unwrap()
    }
}
