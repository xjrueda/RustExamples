//candidate for library of patterns
use actix_web::{
    body::BoxBody, http::header::ContentType, HttpRequest, HttpResponse, Responder
};

use serde::Serialize;

#[derive(Serialize)]
pub struct ObjectWrapper<T> {
    object: T
}

impl<T> ObjectWrapper<T> {
    pub fn new(object:T) -> Self {
        Self{object}
    }
    #[allow(dead_code)]
    pub fn object(&self) -> &T {
        &self.object
    }
}

impl<T : Serialize> Responder for ObjectWrapper<T> {
    type Body = BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        //Create response and set content type
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}