use axum::http;
use hyper::body;
use hyper::Body;
use pretty_assertions::assert_eq;
use tower::ServiceExt;

use my_todo::app::create_app;
use my_todo::repositories::in_memory::TodoRepositoryInMemory;

#[tokio::test]
async fn should_return_hello_world() -> anyhow::Result<()> {
    let repository = TodoRepositoryInMemory::new();

    let req = http::Request::builder().uri("/").body(Body::empty())?;
    let res = create_app(repository).oneshot(req).await?;

    let bytes = body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(bytes.to_vec())?;

    assert_eq!(body, "Hello, World");

    Ok(())
}

#[tokio::test]
async fn should_return_user_data() -> anyhow::Result<()> {
    // let repository = TodoRepositoryInMemory::new();
    //
    // let req = http::Request::builder()
    //     .uri("/users")
    //     .method(Method::POST)
    //     .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
    //     .body(Body::from(r#"{ "username": "John Doe" }"#))?;
    // let res = create_app(repository).oneshot(req).await?;
    //
    // let bytes = body::to_bytes(res.into_body()).await?;
    // let body = String::from_utf8(bytes.to_vec())?;
    // let user: User = serde_json::from_str(&body)?;
    //
    // assert_eq!(user, User::new(1336, "John Doe"));

    Ok(())
}
