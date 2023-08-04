use lambda_http::{run, service_fn, Body, Error, Request, Response};
use tracing::info;


async fn function_handler(_event: Request) -> Result<Response<Body>, Error> {
    let message = "redirecting to https://github.com/rayyildiz".to_string();
    info!("redirecting request to https://github.com/rayyildiz");

    let resp = Response::builder()
        .status(301)
        .header("location", "https://github.com/rayyildiz")
        .body(message.into())
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
