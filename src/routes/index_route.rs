use std::convert::Infallible;

pub async fn index() -> Result<&'static str, Infallible> {
    Ok("Hello, world!")
}
