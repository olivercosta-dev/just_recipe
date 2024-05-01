
use axum::{body::Body, http::Request};

pub fn create_post_request_to(endpoint: &str, json: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(format!("/{}", endpoint))
        .header("Content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

