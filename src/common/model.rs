use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Health {
    pub healthy: bool,
    pub version: String,
}
