pub mod api_calls;

use api_calls::blocking::blocking_api_request;

fn main() -> Result<(), anyhow::Error> {
    let res_body = blocking_api_request()?;
    println!("Body: \n{}", res_body);
    Ok(())
}
