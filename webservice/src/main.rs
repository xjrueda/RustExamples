mod services;
mod business_services;
mod errors;
mod responders;

use std::env;
use std::sync::Mutex;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use services::AppState;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let port = env::var("SERVER_PORT").expect("SERVER_PORT must be set");
    let ip = env::var("SERVER_IP").expect("SERVER_IP must be set");
    let url = format!("{}:{}",ip, port);
    let private_key_file = env::var("PRIVATE_KEY_FILE").expect("Private key file not found");
    let cert_file = env::var("CERT_KEY_FILE").expect("Certificate file not found");
    // create the application state
    let app_state = web::Data::new(AppState {
            counter: Mutex::new(0),
            app_name: Mutex::from(String::from("Actix Web POC"))
    });

    //openssl support
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file(private_key_file, SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file(cert_file).unwrap();

    // use move to send the variables to threads
    HttpServer::new( move || {
        App::new()
            .app_data(app_state.clone())
            .service(services::hello)
            .service(services::query_service)
            .service(services::echo)
            .service(services::person)
            .service(services::business_happy)
            .service(services::business_fails)
            .service(services::counter)
            .route("/hey", web::get().to(services::manual_hello))
    }).bind_openssl(url,builder)?
    .run()
    .await
}
