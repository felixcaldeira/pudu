use axum::{
    extract::{Request, State, Path, Multipart, Query, multipart::MultipartRejection, FromRequest},
    response::{Html, Redirect, Response, IntoResponse},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::Image;
use crate::models::{User, PendingUser, Filters};
use std::collections::HashMap;

pub async fn get(
    State(state): State<AppState>,
    Query(filters): Query<Filters>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    let users = User::find_all(&state.db, filters).await?;

    let image_ids: Vec<u32> = users
        .iter()
        .filter_map(|cat| cat.image_id)
        .collect();
    let images = Image::find_by_ids(&state.db, &image_ids).await?;
    let images_map: HashMap<u32, Image> = images
        .into_iter()
        .map(|img| (img.id, img))
        .collect();
        
    context.insert("page_title", "Dashboard / Benutzer");
    context.insert("users", &users);
    context.insert("images", &images_map);

    let html = TERA.render("manage/users.html", &context)?;
    
    Ok(Html(html))
}

pub async fn post(
    State(state): State<AppState>,
    multipart: Result<Multipart, MultipartRejection>,
) -> Result<Response, AppError> {
    let mut multipart = multipart?;

    let mut email: Option<String> = None;

    while let Some(field) = multipart.next_field().await? {
        let name = field.name().unwrap_or("").to_string();

        match name.as_str() {
            "email" => {
                email = Some(field.text().await?);
            }
            _ => {}
        }
    }

    let email = email.ok_or(AppError::BadRequest("Missing email".into()))?;

    if User::find_by_email(&state.db, &email).await?.is_some() {
        return Err(AppError::BadRequest("User already exists".into()));
    }

    if let Some(user) = PendingUser::find_by_email(&state.db, &email).await? {
        PendingUser::send_verification(&state.config, &email, &user.nanoid).await?;
    } else {
        PendingUser::create(&state.db, &state.config, &email).await?;
    }

    Ok(Redirect::to("/manage/users").into_response())
}