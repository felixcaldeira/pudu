use axum::{
    extract::{Request, State, Path},
    response::{Html, Redirect, Response, IntoResponse},
    http::{StatusCode, header, HeaderValue},
    Form,
};
use tera::Context;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use crate::handlers::AppState;
use crate::models::{User, LoginRequest};
use crate::midware::auth::{Claims};
use crate::TERA;
use crate::AppError;
use crate::helpers::base_context;

pub async fn get(
    State(state): State<AppState>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    context.insert("page_title", "Login");
    
    let html = TERA.render("login.html", &context)?;
    
    Ok(Html(html))
}

pub async fn post(
    State(state): State<AppState>,
    Form(login_data): Form<LoginRequest>,
) -> Result<Response, AppError> {
    let user = User::find_by_username(&state.db, &login_data.username).await?;

    match user {
        Some(user) if user.verify_password(&login_data.password) => {
            let expiration = Utc::now() + Duration::days(14);
            
            let claims = Claims {
                sub: user.id.to_string(),
                exp: expiration.timestamp() as usize,
                iat: Utc::now().timestamp() as usize,
            };
            
            let token = encode(
                &Header::default(), 
                &claims, 
                &EncodingKey::from_secret(state.config.jwt_secret.as_bytes())
            )
            .map_err(|e| {
                tracing::error!("Failed to create JWT: {:?}", e);
                AppError::Internal("Token-Erstellung fehlgeschlagen".into())
            })?;
            
            let cookie = format!(
                "auth_token={}; HttpOnly; SameSite=Lax; Path=/; Max-Age=1209600{}", 
                token,
                if cfg!(debug_assertions) { "" } else { "; Secure" }
            );
            
            let mut response = Redirect::to("/manage").into_response();
            let cookie_value = HeaderValue::from_str(&cookie)
                .map_err(|_| AppError::Internal("Invalid cookie value".into()))?;
            response.headers_mut().insert(header::SET_COOKIE, cookie_value);
            
            Ok(response)
        }
        Some(_) => {
            tracing::warn!("Failed login attempt for username: {}", login_data.username);
            let mut context = Context::new();
            context.insert("page_title", "Login");
            context.insert("error", "Ungültiger Benutzername oder Passwort");
            let html = TERA.render("login.html", &context)?;
            Ok((StatusCode::UNAUTHORIZED, Html(html)).into_response())
        }
        None => {
            User::dummy_verify(); // constant-time dummy bcrypt to prevent timing attacks
            let mut context = Context::new();
            context.insert("page_title", "Login");
            context.insert("error", "Ungültiger Benutzername oder Passwort");
            let html = TERA.render("login.html", &context)?;
            Ok((StatusCode::UNAUTHORIZED, Html(html)).into_response())
        }
    }
}

pub async fn logout() -> Response {
    let mut response = Redirect::to("/login").into_response();
    response.headers_mut().insert(
        header::SET_COOKIE,
        HeaderValue::from_static("auth_token=; HttpOnly; Path=/; Max-Age=0; SameSite=Lax; Secure"),
    );
    response
}
