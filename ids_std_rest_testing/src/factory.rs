use axum::{body::Body, extract::Request, http};

pub struct RequestFactory;

impl RequestFactory {
    pub fn post(uri: &str, body: Body) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method(http::Method::POST)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(body)
            .unwrap()
    }

    pub fn put(uri: &str, body: Body) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .method(http::Method::PUT)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(body)
            .unwrap()
    }

    pub fn get(uri: &str) -> Request<Body> {
        Request::builder()
            .uri(uri)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::empty())
            .unwrap()
    }
}
