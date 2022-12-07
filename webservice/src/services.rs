use std::sync::Mutex;
use actix_web::{get, post, web, HttpResponse, Responder};
use serde::Serialize;
use serde_json::Result;
use crate::business_services::{query_business_object, something_fails, something_ok};

// This struct represents the app state
pub struct AppState {
    pub counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
    pub app_name: Mutex<String>,
}

#[get("/hello")]
pub async fn hello(data: web::Data<AppState>) -> impl Responder {
    let app_name = &data.app_name.lock().unwrap(); // <- get app_name's MutexGuard
    let resp = format!("Hello {app_name}!"); // <- response with app_name
    HttpResponse::Ok().body(resp)
}


#[get("/person/{id}")]
pub async fn query_service(path :web::Path<u32>) -> impl Responder{
    let user_id = path.into_inner();
    let response = query_business_object(user_id);
    response.unwrap()
}

#[post("/echo")]
pub async fn echo(req_body: String) -> HttpResponse {
    HttpResponse::Ok().body(req_body)
}

#[post("/person")]
pub async fn person(req_body: String) -> HttpResponse {
    #[derive(Serialize, serde::Deserialize, Debug)]
    struct Person {
        first_name : String,
        last_name : String,
    }
    let person : Result<Person> = serde_json::from_str(&req_body);
    let msg;
    match person {
        Ok(person) => {
            msg = format!("Person object received {} {}", person.first_name, person.last_name);
        }
        Err(e) => {
            msg = format!("Invalid person {}", e.to_string());
        }
    }
    HttpResponse::Ok().body(msg)
}

#[get("/counter")]
pub async fn counter(data: web::Data<AppState>) -> HttpResponse {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard
    let resp = format!("Request number: {counter}"); // <- response with count
    HttpResponse::Ok().body(resp)
}

#[get("/business_happy")]
pub async fn business_happy() -> impl Responder {
    let response = something_ok();
    response.unwrap()
}

#[get("/business_fails")]
pub async fn business_fails() -> impl Responder {
    let response = something_fails();
    response
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there I am using route!")
}
