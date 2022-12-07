//candidate for library of patterns
use actix_web::{
    error as aw_error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use serde::Serialize;
use derive_more::{Display, Error};
#[allow(dead_code)]
#[derive(Debug, Serialize, Display, Error)]
pub enum AppCodeStatus {
    //Redirection Codes
    Ok,
    Created,
    Accepted,
    MultiStatus,
    AlreadyReported,
    //ClientErrors
    BadRequest,
    Unauthorized,
    PaymentRequired,
    Forbidden,
    NotFound,
    MethodNotAllowed,
    RequestTimeout,
    Conflict,
    Gone,
    PreconditionFailed,
    PayloadTooLarge,
    ExpectationFailed,
    Locked,
    PreconditionRequired,
    TooManyRequests,
    InternalError,
    NotImplemented,
    ServiceUnavailable,
    GatewayTimeout,
    HttpVersionNotSupported,
    InsufficientStorage,
}

#[derive(Debug, Serialize, Display, Error)]
#[display(fmt = "Application error: {}", message)]
pub struct AppError {
    pub status_code: AppCodeStatus,
    pub message: &'static str,
}

impl aw_error::ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match &self.status_code {
            AppCodeStatus::Ok => StatusCode::OK,
            AppCodeStatus::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            AppCodeStatus::BadRequest => StatusCode::BAD_REQUEST,
            AppCodeStatus::GatewayTimeout => StatusCode::GATEWAY_TIMEOUT,
            AppCodeStatus::MultiStatus => StatusCode::MULTI_STATUS,
            AppCodeStatus::AlreadyReported => StatusCode::ALREADY_REPORTED,
            AppCodeStatus::Accepted => StatusCode::ACCEPTED,
            AppCodeStatus::Conflict => StatusCode::CONFLICT,
            AppCodeStatus::Created => StatusCode::CREATED,
            AppCodeStatus::ExpectationFailed => StatusCode::EXPECTATION_FAILED,
            AppCodeStatus::Forbidden => StatusCode::FORBIDDEN,
            AppCodeStatus::Gone => StatusCode::GONE,
            AppCodeStatus::HttpVersionNotSupported => StatusCode::HTTP_VERSION_NOT_SUPPORTED,
            AppCodeStatus::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
            AppCodeStatus::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            AppCodeStatus::PayloadTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            AppCodeStatus::NotFound => StatusCode::NOT_FOUND,
            AppCodeStatus::PreconditionRequired => StatusCode::PRECONDITION_REQUIRED,
            AppCodeStatus::PreconditionFailed => StatusCode::PRECONDITION_FAILED,
            AppCodeStatus::PaymentRequired => StatusCode::PAYMENT_REQUIRED,
            AppCodeStatus::RequestTimeout => StatusCode::REQUEST_TIMEOUT,
            AppCodeStatus::ServiceUnavailable => StatusCode::SERVICE_UNAVAILABLE,
            AppCodeStatus::Unauthorized => StatusCode::UNAUTHORIZED,
            AppCodeStatus::TooManyRequests => StatusCode::TOO_MANY_REQUESTS,
            AppCodeStatus::Locked => StatusCode::LOCKED,
            AppCodeStatus::InsufficientStorage => StatusCode::INSUFFICIENT_STORAGE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(body)
    }
}
