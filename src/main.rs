use actix_web::{web, App, HttpResponse, HttpServer};
pub mod equity_fx_main;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/price", web::post().to(post_price))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("error binding server to address")
        .run()
        .await
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(include_str!("web/index.html"))
}

use equity_fx_main::price;
use serde::Deserialize;

#[derive(Deserialize)]
struct PriceParameters {
    expiry: f64,
    strike: f64,
    spot: f64,
    vol: f64,
    r: f64,
    d: f64,
    number_of_dates: u64,
    number_of_paths: u64,
}

async fn post_price(form: web::Form<PriceParameters>) -> HttpResponse {
    let response = format!(
        "The price is {}\n",
        price(
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

    HttpResponse::Ok().content_type("text/html").body(response)
}
