use axum::{
    extract::Request,
    middleware::Next,
    response::{Html, Response, IntoResponse},
    http::StatusCode,
};
use crate::{TERA, AppError};
use crate::helpers::base_context;

pub async fn error_renderer(
    request: Request,
    next: Next,
) -> Response {
    let mut ctx = base_context(&request);
    let response = next.run(request).await;

    let status = response.status();

    if status.is_redirection() || status.is_success() {
        return response;
    }

    // let message = match status {
    //     StatusCode::NOT_FOUND => "Die Seite wurde nicht gefunden.",
    //     StatusCode::BAD_REQUEST => "Ungültige Anfrage.",
    //     StatusCode::UNAUTHORIZED => "Nicht autorisiert.",
    //     StatusCode::INTERNAL_SERVER_ERROR => "Ein interner Fehler ist aufgetreten.",
    //     _ => "Ein Fehler ist aufgetreten.",
    // };

    let message = response
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or_else(|| "Ein Fehler ist aufgetreten.".to_string());

    ctx.insert("status", &status.as_u16());
    ctx.insert("message", &message);

    let html = TERA.render("error.html", &ctx).unwrap_or_else(|_| {
        format!(
            "<h1>{}</h1><p>{}</p>",
            status.as_u16(),
            message
        )
    });

    (status, Html(html)).into_response()
}
