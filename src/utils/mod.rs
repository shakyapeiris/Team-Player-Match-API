use core::fmt;
use std::collections::HashMap;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use aws_sdk_dynamodb::error::SdkError;
use serde::{Serialize, Deserialize};

use self::ddb::DDB;

pub mod ddb;

#[derive(Serialize, Deserialize, Debug)]
pub struct AppResponse<T> {
    pub data: T,
    pub message: String,
    pub status: i32,
    pub success: bool
}

impl<T> AppResponse<T> {
    pub fn new(data: T, message: Option<String>, status: i32, success: bool) -> AppResponse<T> {
        AppResponse { data, message: match message {
            Some(msg) => msg,
            None => "".to_string()
        }, status, success }
    }
}

pub fn item_to_value(key: &str, item: &HashMap<String, aws_sdk_dynamodb::types::AttributeValue>) -> Result<Option<String>, ()> {
    match item.get(key) {
        Some(value) => {
            match value.as_s() {
                Ok(value) => Ok(Some(value.clone())),
                Err(_) => Err(())
            }
        },
        None => Ok(None)
    }
}

pub fn item_to_vector(key: &str, item: &HashMap<String, aws_sdk_dynamodb::types::AttributeValue>) -> Result<Option<Vec<String>>, ()> {
    match item.get(key) {
        Some(value) => {
            match value.as_ss() {
                Ok(value) => Ok(Some(value.clone())),
                Err(_) => Err(())
            }
        },
        None => Ok(None)
    }
}


pub struct AppState {
    pub ddb: DDB,
}


#[derive(Debug)]
pub enum AppErrorType {
    DbError(String),
    NotFoundError(String),
}

impl AppErrorType {
    fn message(&self) -> String {
        match self {
            Self::NotFoundError(str) => str.to_string(),
            _ => "An error occured".to_string()
        }
    }
}

impl ResponseError for AppErrorType {
    fn status_code(&self) -> StatusCode {
        match self {
            AppErrorType::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError(_) => StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppResponse {
            data: {},
            message: self.message(),
            status: self.status_code().as_u16() as i32,
            success: false
        })
    }
}


impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl <T> From<SdkError<T>> for AppErrorType {
    fn from(e: SdkError<T>) -> Self {
        AppErrorType::NotFoundError(e.to_string())
    }
}

