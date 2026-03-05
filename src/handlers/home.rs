use axum::{
    extract::{Request, State},
    response::{Html},
};
use tera::Context;
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;

pub async fn get(
    State(state): State<AppState>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    context.insert("page_title", "Start");
    
    let html = TERA.render("home.html", &context)?;
    
    Ok(Html(html))
}
