use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]

/// Configuration structure
pub struct ConfigurationFile {
    /// Output path where to write diagnostick
    pub output: String,
}

pub fn load_configuration(path: Option<&str>) -> ConfigurationFile {
    match path {
        Some(s) => from_file(s),
        None => default_configuration(),
    }
}

fn from_file(filepath: &str) -> ConfigurationFile {
    let content: String = fs::read_to_string(filepath)
        .expect(format!("Cannot load configuration file {}", filepath).as_str());

    serde_json::from_str(&content).unwrap()
}

fn default_configuration() -> ConfigurationFile {
    ConfigurationFile {
        output: ".".to_string(),
    }
}
