use axum::{
    response::{Html, IntoResponse, Response},
    http::StatusCode,
};
use thiserror::Error;
use tracing::error;
use crate::TERA;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
    
    #[error("Template error")]
    Template(#[from] tera::Error),
    
    #[error("File upload error")]
    Multipart(#[from] axum::extract::multipart::MultipartError),
    
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
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Die Seite wurde nicht gefunden."
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "Sie haben keine Berechtigung."
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                msg.as_str()
            ),
            AppError::Database(_) => {
                error!("Database error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Ein Datenbankfehler ist aufgetreten.")
            },
            AppError::Template(_) => {
                error!("Template error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Ein Vorlagenfehler ist aufgetreten.")
            },
            AppError::Multipart(_) => {
                error!("Multipart error: {:?}", self);
                (StatusCode::BAD_REQUEST, "Fehler beim Hochladen der Datei.")
            },
            AppError::Io(_) => {
                error!("I/O error: {:?}", self);
                (StatusCode::INTERNAL_SERVER_ERROR, "Ein Dateifehler ist aufgetreten.")
            },
            AppError::Internal(msg) => {
                error!("Internal error: {:?}", msg);
                (StatusCode::INTERNAL_SERVER_ERROR, msg.as_str())
            },
        };

        let mut context = tera::Context::new();
        context.insert("status", &status.as_u16());
        context.insert("message", &message);

        let html = TERA
            .render("error.html", &context)
            .unwrap_or_else(|_| {
                format!(
                    r#"<!DOCTYPE html>
                    <html lang="de">
                    <head>
                        <meta charset="UTF-8">
                        <title>Error: {}</title>
                        <style>
                            body {{ font-family: sans-serif; max-width: 600px; margin: 100px auto; padding: 20px; text-align: center; }}
                            h1 {{ color: #d32f2f; font-size: 3em; }}
                            p {{ font-size: 1.2em; color: #666; }}
                            a {{ color: #1976d2; text-decoration: none; }}
                        </style>
                    </head>
                    <body>
                        <h1>{}</h1>
                        <p>{}</p>
                        <a href="/">← Zur Startseite</a>
                    </body>
                    </html>"#,
                    status.as_u16(), status.as_u16(), message
                )
            });

        (status, Html(html)).into_response()
    }
}

pub async fn handler_404() -> Response {
    AppError::NotFound.into_response()
}