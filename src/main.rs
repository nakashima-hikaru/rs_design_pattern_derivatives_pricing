use axum::{response::Html, routing::get, routing::post, Router};
use serde::Deserialize;
use std::net::SocketAddr;
use std::ops::{Add, Div};
use time::Duration;

mod equity_fx_main;

type HtmlString = Html<&'static str>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", get(index))
        .route("/price", post(calculate_price));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Serving on http://localhost:3000...");
    axum::serve(listener, app.into_make_service()).await?;

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
    let mut times = Vec::<Duration>::new();
    let mut ret = Err("".to_string());
    for _ in 0..10 {
        let now = time::Instant::now();
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
        times.push(now.elapsed());
        if let Ok(result) = result {
            // Release: Duration { seconds: 2, nanoseconds: 770814229 } x1000000
            ret = Ok(format!("The price is {}\n", result));
        } else {
            ret = Err(format!("{}", result.err().unwrap()));
        }
    }
    let total_time = times
        .iter()
        .fold(Duration::default(), |acc, x| acc.add(x.to_owned()));
    let time_result: Duration = total_time.div(times.len() as f64);
    println!("{time_result:?}");
    ret
}
