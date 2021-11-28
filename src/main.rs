use std::sync::Arc;

use reqwest::Client;
use warp::Filter;

mod routes;

#[tokio::main]
async fn main() {
    let client = Arc::new(reqwest::Client::new());

    let index = warp::path::end().and_then(routes::index);

    let search = warp::path("search")
        .and(warp::query::<routes::search_route::Params>())
        .and(with_http_client(client))
        .and_then(routes::search_route::search);

    let routes = warp::get().and(index).or(search);

    let address = match std::env::var("PORT") {
        Ok(port) => ([0, 0, 0, 0], port.parse::<u16>().unwrap()),
        Err(_e) => ([127, 0, 0, 1], 8080),
    };

    warp::serve(routes).run(address).await;
}

fn with_http_client(
    client: Arc<Client>,
) -> impl Filter<Extract = (Arc<Client>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || client.clone())
}
