use axum::http;

pub enum JsonOrEmpty {
    Empty,
    Json(String),
}
impl<S> From<S> for JsonOrEmpty
where
    S: AsRef<str>,
{
    fn from(value: S) -> Self {
        let v = value.as_ref().to_string();
        JsonOrEmpty::Json(v)
    }
}
impl Default for JsonOrEmpty {
    fn default() -> Self {
        JsonOrEmpty::Empty
    }
}

pub fn build_todo_request<B>(
    path: &str,
    method: http::Method,
    body: B,
) -> anyhow::Result<http::Request<hyper::Body>>
where
    B: Into<JsonOrEmpty>,
{
    let request = match body.into() {
        JsonOrEmpty::Json(body) => http::Request::builder()
            .uri(path)
            .method(method)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(hyper::Body::from(body))?,
        JsonOrEmpty::Empty => http::Request::builder()
            .uri(path)
            .method(method)
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(hyper::Body::empty())?,
    };

    Ok(request)
}
