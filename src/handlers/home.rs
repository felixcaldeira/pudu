use axum::{
    extract::State,
    response::Html,
};
use tera::Context;
use crate::handlers::AppState;
use crate::handlers::error::AppError;
use crate::models::{User};
use crate::TERA;

pub async fn index(State(state): State<AppState>) -> Result<Html<String>, AppError> {
    let mut context = Context::new();

    context.insert("page_title", "Start");
    
    let html = TERA.render("home.html", &context)?;
    
    Ok(Html(html))
}
