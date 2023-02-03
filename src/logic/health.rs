use crate::model::HealthInfo;
use crate::AppResult;

pub async fn health_status() -> AppResult<HealthInfo> {
    Ok(HealthInfo::ok())
}
