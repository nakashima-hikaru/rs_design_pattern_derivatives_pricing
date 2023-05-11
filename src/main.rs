use axum::{response::Html, routing::get, routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;

pub mod equity_fx_main;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(get_index))
        .route("/price", post(post_price));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Serving on http://localhost:3000...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn get_index() -> Html<&'static str> {
    Html(include_str!("web/index.html"))
}

#[derive(Deserialize)]
struct PriceParameters {
    expiry: f64,
    strike: f64,
    spot: f64,
    vol: f64,
    r: f64,
    d: f64,
    number_of_dates: usize,
    number_of_paths: usize,
}

async fn post_price(form: axum::extract::Form<PriceParameters>) -> String {
    let response = format!(
        "The price is {}\n",
        equity_fx_main::price(
            form.expiry,
            form.strike,
            form.spot,
            form.vol,
            form.r,
            form.d,
            form.number_of_dates,
            form.number_of_paths,
        )
    );

    response
}
