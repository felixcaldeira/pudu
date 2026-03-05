use axum::{
    extract::{Request, State, Path, Multipart, multipart::MultipartRejection, FromRequest},
    response::{Html, Redirect, Response, IntoResponse},
    http::StatusCode,
};
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::helpers::base_context;
use crate::models::Image;
use crate::models::{User};
use std::collections::HashMap;

// pub async fn get(
//     State(state): State<AppState>,
//     Path((category_slug, module_slug)): Path<(String, String)>,
//     request: Request,
// ) -> Result<Html<String>, AppError> {
//     let mut context = base_context(&request);

//     let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     let module = Module::find_by_slug(&state.db, &module_slug)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     let module_categories = ModuleCategory::find_all(&state.db).await?;
//     let users = User::find_all(&state.db).await?;

//     let mut image: Option<Image> = None;

//     if let Some(image_id) = module.image_id {
//         image = Image::find_by_id(&state.db, image_id).await?;
//     }
        
//     context.insert("page_title", "Dashboard / Unterrichtseinheiten");
//     context.insert("users", &users);
//     context.insert("image", &image);

//     let html = TERA.render("manage/single_module.html", &context)?;
    
//     Ok(Html(html))
// }

// pub async fn put(
//     State(state): State<AppState>,
//     Path((category_slug, module_slug)): Path<(String, String)>,
//     multipart: Result<Multipart, MultipartRejection>,
// ) -> Result<Response, AppError> {
//     let mut multipart = multipart?;

//     let module_category = ModuleCategory::find_by_slug(&state.db, &category_slug)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     let module = Module::find_by_slug(&state.db, &module_slug)
//         .await?
//         .ok_or(AppError::NotFound)?;

//     let mut title: Option<String> = None;
//     let mut description: Option<String> = None;
//     let mut content: Option<String> = None;
//     let mut published: Option<i8> = None;
//     let mut image_id: Option<u32> = None;
//     let mut category_id: Option<u32> = None;
//     let mut user_id: Option<u32> = None;
//     let mut grade_flags: u32 = 0;

//     while let Some(field) = multipart.next_field().await? {
//         let name = field.name().unwrap_or("").to_string();
        
//         match name.as_str() {
//             "title" => {
//                 let text = field.text().await?;
//                 if !text.is_empty() {
//                     title = Some(text);
//                 }
//             }
//             _ => {}
//         }
//     }

//     let category_id = category_id.ok_or(AppError::BadRequest("Kategorie ist erforderlich".into()))?;

//     // if let Some(cat_id) = category_id {
//     let category = ModuleCategory::find_by_id(&state.db, category_id)
//         .await?
//         .ok_or(AppError::NotFound)?;
//     // }

//     if let Some(uid) = user_id {
//         User::find_by_id(&state.db, uid)
//             .await?
//             .ok_or(AppError::NotFound)?;
//     }

//     let update_data = ModuleUpdate {
//         title,
//         description,
//     };

//     Module::update(&state.db, module.id, update_data).await?;

//     Ok(Redirect::to(&format!("/manage/modules/{}/{}", category.slug, module_slug).to_string()).into_response())
// }

pub async fn delete(
    State(state): State<AppState>,
    Path(user_id): Path<u32>,
) -> Result<Response, AppError> {
    let user = User::find_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if let Some(image_id) = user.image_id {
        Image::delete(&state.db, image_id).await?;
    }
    User::delete(&state.db, user_id).await?;

    Ok(Redirect::to("/manage/users").into_response())
}