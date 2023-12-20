use std::env;

use dotenv;

use actix_web::{web, App, HttpServer};
use transaction::routes::route::config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let environment = env::var("ENV").unwrap_or_else(|_| "dev".to_string());
    if environment == "dev" {
        dotenv::from_filename("node.env").ok();
    }
    let port = env::var("PORT").expect("Failed to load the Port !!");
    
    HttpServer::new(|| 
        App::new()
        .configure(config)
    )
    .bind(("0.0.0.0", port.parse::<u16>().unwrap()))?
    .run()
    .await
}