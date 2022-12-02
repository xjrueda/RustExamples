use crate::errors::{AppError, AppCodeStatus};
use crate::responders::*;

use serde::Serialize;

#[derive(Serialize)]
pub struct BusinessObject {
    id:i32,
    name: String,
}

impl BusinessObject {
    fn new(_id:i32, _name: String) -> Self {
        Self {
            id:_id,
            name: _name,
        }
    }
}

pub fn something_ok() -> Result<ObjectWrapper<BusinessObject>, AppError> {
    let business_obj = BusinessObject::new (
        5796982,
        String::from("Juan Carlos")
    );
    let  response = ObjectWrapper::new(business_obj);
    Ok(response)
}

pub fn something_fails() -> Result<ObjectWrapper<BusinessObject>, AppError> {
    let result =  AppError{
        status_code: AppCodeStatus::BadRequest,
        message:"something went wrong",
    };
    Err(result)
}