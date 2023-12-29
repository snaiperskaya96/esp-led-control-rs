use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GpioConfig {
    pub gpio: u16,
    pub channel: u16,
    pub alias: String,
    pub frequency: u32,
}

pub fn get_configs() -> anyhow::Result<&'static HashMap<String, GpioConfig>> {
    lazy_static::lazy_static! {
        static ref PARSED_CONFIG: HashMap<String, GpioConfig> = {
            serde_yaml::from_str(include_str!("../gpio_config.yaml")).unwrap()
        };
    }

    Ok(&PARSED_CONFIG)
}