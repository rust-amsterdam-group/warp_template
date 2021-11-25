use std::convert::Infallible;
use std::sync::Arc;

use reqwest::Client;
use serde::Deserialize;

pub async fn search(params: Params, client: Arc<Client>) -> Result<&'static str, Infallible> {
    Ok(client
        .get(format!("https://en.wikipedia.org/wiki/{}", params.query))
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_or("oh snap!", |_| "success"))
}

#[derive(Deserialize)]
pub struct Params {
    query: String,
}
