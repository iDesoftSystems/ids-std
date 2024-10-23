use axum::async_trait;
use http_body_util::BodyExt;

#[async_trait]
pub trait IntoValueExt {
    async fn into_value(self) -> serde_json::Value;
}

#[async_trait]
impl IntoValueExt for axum::http::Response<axum::body::Body> {
    async fn into_value(self) -> serde_json::Value {
        let collected = self.into_body().collect().await.unwrap();
        let response_in_bytes = collected.to_bytes();

        serde_json::from_slice(&response_in_bytes).unwrap()
    }
}
