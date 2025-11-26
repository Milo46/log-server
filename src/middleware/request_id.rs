use axum::{extract::Request, http::HeaderValue, middleware::Next, response::Response};
use tower_http::trace::MakeSpan;
use tracing::Span;
use uuid::Uuid;

pub const REQUEST_ID_HEADER: &str = "X-Request-ID";

#[derive(Clone)]
pub struct RequestIdLayer;

impl RequestIdLayer {
    pub fn new() -> Self {
        Self
    }

    pub async fn middleware(mut request: Request, next: Next) -> Response {
        let request_id = request
            .headers()
            .get(REQUEST_ID_HEADER)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| Uuid::new_v4().to_string());

        request.extensions_mut().insert(request_id.clone());

        let mut response = next.run(request).await;

        if let Ok(header_value) = HeaderValue::from_str(&request_id) {
            response
                .headers_mut()
                .insert(REQUEST_ID_HEADER, header_value);
        }

        response
    }
}

impl Default for RequestIdLayer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct RequestIdMakeSpan;

impl<B> MakeSpan<B> for RequestIdMakeSpan {
    fn make_span(&mut self, request: &axum::http::Request<B>) -> Span {
        let request_id = request
            .extensions()
            .get::<String>()
            .map(|s| s.as_str())
            .unwrap_or("unknown");

        tracing::info_span!(
            "http_request",
            method = %request.method(),
            uri = %request.uri(),
            version = ?request.version(),
            request_id = %request_id,
        )
    }
}
