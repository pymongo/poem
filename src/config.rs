use serde::{Deserialize, Serialize};

#[serde(Serialize, Deserialize)]
pub struct Config {
    limits: LimitsConfig,
}

#[serde(Serialize, Deserialize)]
pub struct LimitsConfig {
    form: usize,
}
