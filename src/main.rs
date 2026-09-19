mod api_contract;
mod routes;
mod service;

use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let bind_address =
        std::env::var("API_BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    println!("listening on http://{bind_address}");

    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(service::default_service()))
            .configure(routes::configure)
    })
    .bind(bind_address)?
    .run()
    .await
}
