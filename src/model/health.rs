use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum HealthStatus {
    OK,
    FAIL,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthInfo {
    status: HealthStatus,
}

impl HealthInfo {
    pub fn new(status: HealthStatus) -> Self {
        HealthInfo { status }
    }

    pub fn ok() -> Self {
        HealthInfo {
            status: HealthStatus::OK,
        }
    }

    pub fn fail() -> Self {
        HealthInfo {
            status: HealthStatus::FAIL,
        }
    }
}
