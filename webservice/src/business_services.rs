use crate::errors::{AppError, AppCodeStatus};
use crate::responders::*;

use serde::Serialize;

#[derive(Serialize)]
pub struct BusinessObject {
    id:u32,
    name: String,
}

impl BusinessObject {
    fn new(id: u32, _name: String) -> Self {
        Self {
            id,
            name: _name,
        }
    }
}

#[derive(Serialize)]
pub struct AnyResponse {
    message : String,
    code : i32
}

pub fn query_business_object(one_id :u32) -> Result<ObjectWrapper<BusinessObject>, AppError> {
    //query business object using the id with successful response
    let business_obj = BusinessObject::new (
        one_id,
        String::from("Juan Carlos")
    );
    let  response = ObjectWrapper::new(business_obj);
    Ok(response)
}

pub fn something_ok() -> Result<ObjectWrapper<AnyResponse>, AppError> {
    //do business stuff ans response any object
    let any_response = AnyResponse{
                            message:String::from("The process response was bla, bla , bla ...."),
                            code:12345
                        };
    let  response = ObjectWrapper::new(any_response);
    Ok(response)
}

pub fn something_fails() -> Result<ObjectWrapper<BusinessObject>, AppError> {
    //validate req_body
    //do business stuff that return some error
    let result =  AppError{
        status_code: AppCodeStatus::BadRequest,
        message:"something went wrong",
    };
    Err(result)
}