// // midware/error.rs
// use axum::{
//     middleware::Next,
//     response::{Html, IntoResponse, Response},
//     extract::{Request, State},
//     http::StatusCode,
// };
// use crate::handlers::{AppState, AppError};
// use serde::Serialize;

// pub async fn error_handler(
//     State(state): State<AppState>,
//     request: Request,
//     next: Next,
// ) -> Response {
//     if cfg!(debug_assertions) {
//         eprintln!("[DEBUG] Processing request: {} {}", request.method(), request.uri());
//     }

//     let response = next.run(request).await;
    
//     // Check if the response is an error status
//     if response.status().is_client_error() || response.status().is_server_error() {
//         if cfg!(debug_assertions) {
//             eprintln!("[ERROR MIDDLEWARE] Caught error response: {:#?}", response.status());
//         }

//         let error = match response.status() {
//             StatusCode::NOT_FOUND => AppError::NotFound,
//             StatusCode::UNAUTHORIZED => AppError::Unauthorized,
//             StatusCode::BAD_REQUEST => AppError::BadRequest("Bad request".to_string()),
//             _ => AppError::Internal(response.status().to_string()),
//         };
        
//         return handle_error_with_template(State(state), error).await;
//     }
    
//     response
// }

// #[derive(Serialize)]
// struct ErrorContext {
//     status_code: u16,
//     error_message: String,
//     error_description: String,
//     show_details: bool,
// }

// async fn handle_error_with_template(
//     State(app_state): State<crate::handlers::AppState>,
//     err: AppError,
// ) -> Response {
//     // Log in debug mode
//     if cfg!(debug_assertions) {
//         eprintln!("[ERROR HANDLER] Processing error: {:#?}", err);
//     }

//     let (status, error_message, error_description) = match err {
//         AppError::Database(ref e) => (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             "Database Error".to_string(),
//             if cfg!(debug_assertions) {
//                 format!("Database operation failed: {}", e)
//             } else {
//                 "A database error occurred. Please try again later.".to_string()
//             }
//         ),
//         AppError::Template(ref e) => (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             "Template Error".to_string(),
//             if cfg!(debug_assertions) {
//                 format!("Template rendering failed: {}", e)
//             } else {
//                 "A server error occurred. Please try again later.".to_string()
//             }
//         ),
//         AppError::NotFound => (
//             StatusCode::NOT_FOUND,
//             "Page Not Found".to_string(),
//             "The page you're looking for doesn't exist or may have been moved.".to_string(),
//         ),
//         AppError::Unauthorized => (
//             StatusCode::UNAUTHORIZED,
//             "Unauthorized".to_string(),
//             "You don't have permission to access this resource.".to_string(),
//         ),
//         AppError::BadRequest(ref msg) => (
//             StatusCode::BAD_REQUEST,
//             "Bad Request".to_string(),
//             msg.clone(),
//         ),
//         AppError::Internal(ref msg) => (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             "Internal Server Error".to_string(),
//             if cfg!(debug_assertions) {
//                 msg.clone()
//             } else {
//                 "An unexpected error occurred. Please try again later.".to_string()
//             }
//         ),
//     };

//     let context = ErrorContext {
//         status_code: status.as_u16(),
//         error_message: error_message.clone(),
//         error_description: error_description.clone(),
//         show_details: cfg!(debug_assertions),
//     };

//     let body= match app_state.tera
//         .render("error.html", &tera::Context::from_serialize(&context).unwrap_or_default()) {
//             Ok(rendered) => {
//                 if cfg!(debug_assertions) {
//                     eprintln!("[DEBUG] Successfully rendered error template");
//                 }
//                 rendered
//             },
//             Err(e) => {
//                 if cfg!(debug_assertions) {
//                     eprintln!("[DEBUG] Template render failed: {:#?}, using fallback", e);
//                 }
//                 format!(
//                     r#"<!DOCTYPE html>
//                     <html><head><title>Error {}</title></head>
//                     <body style="font-family: Arial; text-align: center; margin-top: 100px;">
//                         <h1>{}</h1><p>{}</p><a href="/">Go Home</a>
//                     </body></html>"#,
//                     status.as_u16(), status.as_u16(), error_message
//                 )
//             }
//         };

//     (status, Html(body)).into_response()
// }