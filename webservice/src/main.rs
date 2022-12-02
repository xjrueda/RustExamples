mod services;
mod business_services;
mod errors;
mod responders;

use std::env;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use services::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let ip = env::var("SERVER_IP").expect("SERVER_IP must be set");
    // create the application state
    let app_state = web::Data::new(AppState {
            counter: Mutex::new(0),
            app_name: Mutex::from(String::from("Actix Web POC"))
    });

    // use move to send the variables to threads
    HttpServer::new( move || {
        App::new()
            .app_data(app_state.clone())
            .service(services::hello)
            .service(services::echo)
            .service(services::business_happy)
            .service(services::business_fails)
            .service(services::counter)
            .route("/hey", web::get().to(services::manual_hello))
    })
    .bind((ip, port.parse::<u16>().unwrap()))?
    .run()
    .await
}
