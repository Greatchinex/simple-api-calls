use reqwest::get;

#[tokio::main]
pub async fn async_api_request() -> Result<String, anyhow::Error> {
    let res_body = get("http://httpbin.org/get").await?.text().await?;

    Ok(res_body)
}
