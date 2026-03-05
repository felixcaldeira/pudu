// src/middleware/auth.rs
use axum::{
    extract::{Request, State},
    http::{header, HeaderValue},
    middleware::Next,
    response::{Redirect, Response, IntoResponse},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use tracing::warn;
use crate::handlers::AppState;
use crate::models::{User, UserFlags};
use crate::AppError;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,  
}

pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let token = extract_auth_token(&request)
        .ok_or(AppError::Unauthorized)?;
    let claims = verify_token(token, state.config.jwt_secret.as_bytes())
        .map_err(|_| AppError::Unauthorized)?;
    let user_id: u32 = claims.sub
        .parse()
        .map_err(|_| AppError::Unauthorized)?;
    let user = User::find_by_id(&state.db, user_id)
        .await?
        .ok_or(AppError::Unauthorized)?;
    request.extensions_mut().insert(Some(user));
    Ok(next.run(request).await)
}

pub async fn optional_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let mut user: Option<User> = None;
    let mut should_clear = false;
    if let Some(token) = extract_auth_token(&request) {
        match verify_token(token, state.config.jwt_secret.as_bytes()) {
            Ok(claims) => {
                match claims.sub.parse::<u32>() {
                    Ok(user_id) => {
                        match User::find_by_id(&state.db, user_id).await {
                            Ok(Some(u)) => user = Some(u),
                            _ => should_clear = true, // user deleted
                        }
                    }
                    Err(_) => should_clear = true,
                }
            }
            Err(_) => {
                warn!("Invalid token");
                should_clear = true;
            }
        }
    }
    request.extensions_mut().insert(user);
    let mut response = next.run(request).await;
    if should_clear {
        response.headers_mut().insert(
            header::SET_COOKIE,
            "auth_token=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax"
                .parse()
                .unwrap(),
        );
    }
    response
}

pub async fn redirect_if_authenticated(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, AppError> {
    if let Some(token) = extract_auth_token(&request) {
        if let Ok(claims) = verify_token(token, state.config.jwt_secret.as_bytes()) {
            let user_id: u32 = claims.sub
                .parse()
                .map_err(|_| AppError::Unauthorized)?;
            if let Ok(Some(_user)) = User::find_by_id(&state.db, user_id).await {
                return Ok(Redirect::to("/manage").into_response());
            }
        }
    }
    Ok(next.run(request).await)
}

pub async fn clear_stale_auth_cookie(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    let had_token = extract_auth_token(&request).is_some();
    
    // pull this out BEFORE consuming the request
    let user_from_request = request.extensions().get::<Option<User>>().cloned();
    
    let mut response = next.run(request).await;

    if had_token {
        let is_stale = matches!(user_from_request, None | Some(None));
        if is_stale {
            response.headers_mut().insert(
                header::SET_COOKIE,
                HeaderValue::from_static(
                    "auth_token=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax; Secure"
                ),
            );
        }
    }

    response
}


pub async fn intersects_flag(
    State(state): State<AppState>,
    request: Request,
    next: Next,
    flag: UserFlags,
) -> Result<Response, AppError> {
    let user = request
        .extensions()
        .get::<Option<User>>()
        .and_then(|u| u.as_ref())
        .ok_or(AppError::Unauthorized)?;

    if !user.flags.intersects(flag) {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}
pub async fn contains_flag(
    State(state): State<AppState>,
    request: Request,
    next: Next,
    flag: UserFlags,
) -> Result<Response, AppError> {
    let user = request
        .extensions()
        .get::<Option<User>>()
        .and_then(|u| u.as_ref())
        .ok_or(AppError::Unauthorized)?;

    if !user.flags.contains(flag) {
        return Err(AppError::Unauthorized);
    }

    Ok(next.run(request).await)
}

fn extract_auth_token(request: &Request) -> Option<&str> {
    request
        .headers()
        .get(header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .find(|c| c.trim().starts_with("auth_token="))?
        .trim()
        .strip_prefix("auth_token=")
}

fn verify_token(token: &str, secret: &[u8]) -> Result<Claims, ()> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::default()
    )
    .map(|data| data.claims)
    .map_err(|_| ())
}