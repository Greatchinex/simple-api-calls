pub mod api_calls;

use api_calls::blocking::blocking_api_request;
use api_calls::non_blocking::{async_api_request, github_stargazers};

fn main() -> Result<(), anyhow::Error> {
    let res_body = blocking_api_request()?;
    println!("Body: \n{}", res_body);

    let res_body_async = async_api_request()?;
    println!("ASYNC_BODY: \n{}", res_body_async);

    let star_gazers = github_stargazers()?;
    println!("GITHUB REPO GAZERS: {:?}", star_gazers);

    Ok(())
}
