use axum::{
    middleware::Next,
    extract::Request,
    extract::multipart::{MultipartRejection, MultipartError},
    response::{Html, IntoResponse, Response, Redirect},
    http::StatusCode,
};
use thiserror::Error;
use tracing::{error, warn};
use crate::{TERA, helpers::base_context};

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    #[error("Template error")]
    Template(#[from] tera::Error),
    #[error("File upload error")]
    Multipart(#[from] MultipartError),
    #[error("Multipart rejection error")]
    MultipartRejection(#[from] MultipartRejection),
    #[error("File system error")]
    Io(#[from] std::io::Error),
    #[error("Not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    BadRequest(String),
    #[error("Internal error: {0}")]
    Internal(String),
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::Unauthorized => {
                warn!("401 Unauthorized");
                (
                    StatusCode::UNAUTHORIZED,
                    "Keine Befugnis diese Seite aufzurufen.".to_string(),
                )
            }
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Die Seite wurde nicht gefunden.".to_string(),
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg.clone(),
            ),
            AppError::Internal(msg) => {
                error!("Internal server error: {:?}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    msg.clone(),
                )
            },
            AppError::Database(err) => {
                error!("Database error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Ein Datenbankfehler ist aufgetreten.".to_string(),
                )
            }
            AppError::Template(err) => {
                error!("Template error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Ein Vorlagenfehler ist aufgetreten.".to_string(),
                )
            }
            AppError::Multipart(err) => {
                error!("Multipart error: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "Fehler beim Hochladen der Datei.".to_string(),
                )
            }
            AppError::MultipartRejection(err) => {
                error!("Multipart rejection: {:?}", err);
                (
                    StatusCode::BAD_REQUEST,
                    "Ungültige Multipart-Anfrage.".to_string(),
                )
            }
            AppError::Io(err) => {
                error!("IO error: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Ein Dateifehler ist aufgetreten.".to_string(),
                )
            }
        };

        let mut response = (status, message.clone()).into_response();
        response.extensions_mut().insert(message);

        response
    }
}

pub async fn handler_404() -> Response {
    AppError::NotFound.into_response()
}