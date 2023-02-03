mod tools;
use tools::{with_server, TestError};

use serde_json::{json, Value};
use axum::http::StatusCode;

use assert_json_diff::assert_json_eq;

#[tokio::test]
async fn health_test() -> Result<(), TestError> {
    with_server(|server_addr| async move {
        let url = format!("http://{}/health", server_addr);

        let resp = reqwest::get(&url).await?;
        assert_eq!(StatusCode::OK, resp.status());

        let body = resp.text().await?;

        let sample_json = json!({ "status": "OK" });
        let json: Value = serde_json::from_str(&body)?;
        assert_json_eq!(sample_json, json);

        Ok(())
    })
        .await
}
