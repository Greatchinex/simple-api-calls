use reqwest::blocking::get;
use std::io::Read;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockingRequestError {
    #[error("Failed to read api response body to string")]
    IoError(#[from] std::io::Error),

    #[error("Failed to make API request")]
    HttpRequestError(#[from] reqwest::Error),
}

pub fn blocking_api_request() -> Result<String, BlockingRequestError> {
    let mut res = get("http://httpbin.org/get")?;
    let mut res_body = String::new();
    res.read_to_string(&mut res_body)?;

    println!("Status: {}", res.status());
    println!("Headers: {:?}", res.headers());

    Ok(res_body)
}
