use axum::{body::Body, http::Request};

/// Creates a POST request to the specified endpoint with the given JSON payload.
///
/// This function constructs an HTTP POST request to the specified endpoint. It sets the
/// "Content-type" header to "application/json" and includes the provided JSON payload in the
/// request body.
///
/// # Parameters
/// - `endpoint`: A string slice that specifies the endpoint to which the request is sent.
/// - `json`: A `serde_json::Value` that represents the JSON payload to be included in the request body.
///
/// # Returns
/// - `Request<Body>`: The constructed HTTP POST request.
///
/// # Panics
/// This function will panic if:
/// - The JSON payload cannot be serialized to a vector of bytes.
/// - The request builder fails to construct the request.
pub fn create_post_request_to(endpoint: &str, json: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(format!("/{}", endpoint))
        .header("Content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

/// Creates a DELETE request to the specified endpoint with the given JSON payload.
///
/// This function constructs an HTTP DELETE request to the specified endpoint. It sets the
/// "Content-type" header to "application/json" and includes the provided JSON payload in the
/// request body.
///
/// # Parameters
/// - `endpoint`: A string slice that specifies the endpoint to which the request is sent.
/// - `json`: A `serde_json::Value` that represents the JSON payload to be included in the request body.
///
/// # Returns
/// - `Request<Body>`: The constructed HTTP DELETE request.
///
/// # Panics
/// This function will panic if:
/// - The JSON payload cannot be serialized to a vector of bytes.
/// - The request builder fails to construct the request.
pub fn create_delete_request_to(endpoint: &str, json: serde_json::Value) -> Request<Body> {
    Request::builder()
        .method("DELETE")
        .uri(format!("/{}", endpoint))
        .header("Content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

/// Creates a PUT request to the specified endpoint with the given JSON payload and resource ID.
///
/// This function constructs an HTTP PUT request to the specified endpoint, targeting a specific
/// resource identified by `resource_id`. It sets the "Content-type" header to "application/json"
/// and includes the provided JSON payload in the request body.
///
/// # Parameters
/// - `endpoint`: A string slice that specifies the endpoint to which the request is sent.
/// - `resource_id`: An `i32` that represents the ID of the resource being targeted by the request.
/// - `json`: A `serde_json::Value` that represents the JSON payload to be included in the request body.
///
/// # Returns
/// - `Request<Body>`: The constructed HTTP PUT request.
///
/// # Panics
/// This function will panic if:
/// - The JSON payload cannot be serialized to a vector of bytes.
/// - The request builder fails to construct the request.
pub fn create_put_request_to(
    endpoint: &str,
    resource_id: i32,
    json: serde_json::Value,
) -> Request<Body> {
    Request::builder()
        .method("PUT")
        .uri(format!("/{}/{}", endpoint, resource_id))
        .header("Content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}

/// Creates a GET request to the specified endpoint with optional resource ID, query parameters, and JSON payload.
///
/// This function constructs an HTTP GET request to the specified endpoint. It optionally includes a
/// resource ID and query parameters in the URI. The JSON payload is included in the request body.
///
/// # Parameters
/// - `endpoint`: A string slice that specifies the endpoint to which the request is sent.
/// - `resource_id`: An optional `i32` that represents the ID of the resource being targeted by the request.
/// - `query_params`: An optional `String` that represents the query parameters to be included in the URI.
///   - The query string should be in the format "key1=value1&key2=value2".
/// - `json`: A `serde_json::Value` that represents the JSON payload to be included in the request body.
///
/// # Returns
/// - `Request<Body>`: The constructed HTTP GET request.
///
/// # Panics
/// This function will panic if:
/// - The JSON payload cannot be serialized to a vector of bytes.
/// - The request builder fails to construct the request.
///
pub fn create_get_request_to(
    endpoint: &str,
    resource_id: Option<i32>,
    query_params: Option<String>,
    json: serde_json::Value,
) -> Request<Body> {
    let resource;

    if resource_id.is_none() {
        resource = String::from("");
    } else {
        resource = format!("/{}", resource_id.unwrap());
    }
    let query;
    if query_params.is_some() {
        query = "?".to_owned() + &query_params.unwrap();
    } else {
        query = String::from("");
    }
    Request::builder()
        .method("GET")
        .uri(format!("/{}{}{}", endpoint, resource, query))
        .body(Body::from(serde_json::to_vec(&json).unwrap()))
        .unwrap()
}
