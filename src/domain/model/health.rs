use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
}

impl HealthStatus {
    pub fn new(status: String) -> HealthStatus {
        HealthStatus {
            status,
        }
    }
}
