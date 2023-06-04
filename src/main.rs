use axum::{response::Html, routing::get, routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;

mod equity_fx_main;

type HtmlString = Html<&'static str>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/price", post(calculate_price));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Serving on http://localhost:3000...");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn index() -> HtmlString {
    Html(include_str!("web/index.html"))
}

#[derive(Deserialize)]
struct PriceParameters {
    option_type: String,
    expiry: f64,
    strike: f64,
    spot: f64,
    vol: f64,
    r: f64,
    d: f64,
    number_of_dates: usize,
    number_of_paths: usize,
}

async fn calculate_price(form: axum::extract::Form<PriceParameters>) -> Result<String, String> {
    let result = equity_fx_main::price(
        form.option_type.as_str(),
        form.expiry,
        form.strike,
        form.spot,
        form.vol,
        form.r,
        form.d,
        form.number_of_dates,
        form.number_of_paths,
    );
    if let Ok(result) = result {
        Ok(format!("The price is {}\n", result))
    } else {
        Err(format!("{}", result.err().unwrap()))
    }
}
