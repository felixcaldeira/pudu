use axum::{
    extract::{Request, State},
    response::Html,
};
use tera::Context;
use crate::handlers::AppState;
use crate::AppError;
use crate::models::{User};
use crate::TERA;
use crate::helpers::base_context;

pub async fn get(
    State(state): State<AppState>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    context.insert("page_title", "Dashboard");
    
    let html = TERA.render("manage/dashboard.html", &context)?;
    
    Ok(Html(html))
}