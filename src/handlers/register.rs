use axum::{
    Form,
    extract::{Request, State, Path, Multipart, multipart::MultipartRejection, FromRequest},
    response::{Html, Redirect, Response, IntoResponse},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::Image;
use crate::models::{User, PendingUser, user::UserCreate, UserFlags};
use std::collections::HashMap;
use serde::Deserialize;

pub async fn get(
    State(state): State<AppState>,
    Path(nanoid): Path<String>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let pending_user = PendingUser::find_by_nanoid(&state.db, &nanoid).await?;
        
    context.insert("page_title", "Konto erstellen");
    context.insert("pending_user", &pending_user);

    let html = TERA.render("register.html", &context)?;
    
    Ok(Html(html))
}

#[derive(Deserialize)]
pub struct RegisterForm {
    pub username: String,
    pub academic_title: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

pub async fn post(
    State(state): State<AppState>,
    Path(nanoid): Path<String>,
    Form(form): Form<RegisterForm>,
) -> Result<Response, AppError> {
    let username = form.username;
    let first_name = form.first_name;
    let last_name = form.last_name;
    let password = form.password;
    let academic_title = form.academic_title.unwrap_or_default();

    let pending_user = PendingUser::find_by_nanoid(&state.db, &nanoid)
        .await?
        .ok_or(AppError::NotFound)?;
    let email = pending_user.email;

    if User::find_by_email(&state.db, &email).await?.is_some() {
        return Err(AppError::BadRequest("User already exists".into()));
    }

    if User::find_by_username(&state.db, &username).await?.is_some() {
        return Err(AppError::BadRequest("Username already taken".into()));
    }

    let create_data = UserCreate {
        username,
        email: email.clone(),
        password,
        academic_title,
        first_name,
        last_name,
        flags: UserFlags::DEFAULT,
    };

    User::create(&state.db, create_data).await?;

    PendingUser::delete(&state.db, &email).await?;

    Ok(Redirect::to("/").into_response())
}