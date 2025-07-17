use actix_web::HttpResponse;
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel::result::Error::{DatabaseError, NotFound};
use std::fmt;


#[derive(Debug, Serialize)]
pub enum AppError {
    RecordAlreadyExists,
    RecordNotFound,
    DatabaseError(String),
    OperationCanceled,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::RecordAlreadyExists => write!(f, "This Record violates a unique constraint"),
            AppError::RecordNotFound => write!(f, "This record does not exist"),
            AppError::DatabaseError(e) => write!(f, "Database error: {:?}", e),
            AppError::OperationCanceled => write!(f, "The operation was cancelled"),
        }
    }   
}

impl From<diesel::result::Error> for AppError {
    fn from(e: diesel::result::Error) -> Self {
        match  e {
            DatabaseError(UniqueViolation, _) => AppError::RecordAlreadyExists,
            NotFound => AppError::RecordNotFound,
            _ => AppError::DatabaseError(format!("{:?}", e)),
        }
    }
}

// impl From<BlockingError<diesel::result::Error>> for AppError {
//     fn from(e: BlockingError<diesel::result::Error>) -> Self { 
//         match e {
//             BlockingError::Error(inner) => AppError::from(inner),
//             BlockingError::Canceled => AppError::OperationCanceled,
//         }
//     }
// }

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    err: String,
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        let err = format!("{}", self);
        match self {
            AppError::RecordAlreadyExists => HttpResponse::BadRequest().json(ErrorResponse { err }),
            AppError::RecordNotFound => HttpResponse::NotFound().json(ErrorResponse { err }),
            _ => HttpResponse::InternalServerError().json(ErrorResponse { err }),
        }
    }
}