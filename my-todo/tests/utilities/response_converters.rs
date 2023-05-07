use axum::response;
use serde::Deserialize;

pub async fn convert_to_todo<T>(res: response::Response) -> anyhow::Result<T>
where
    T: for<'a> Deserialize<'a>,
{
    let bytes = hyper::body::to_bytes(res.into_body()).await?;
    let body = String::from_utf8(bytes.to_vec())?;
    let todo: T = serde_json::from_str(&body)?;
    Ok(todo)
}
