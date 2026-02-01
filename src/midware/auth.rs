// src/middleware/auth.rs
use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::{Redirect, Response, IntoResponse},
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::handlers::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String, // User ID
    pub username: String,
    pub exp: usize,
}

// Middleware that requires authentication
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Try to get JWT from cookie
    let auth_cookie = request
        .headers()
        .get(header::COOKIE)
        .and_then(|cookie| cookie.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find(|cookie| cookie.trim().starts_with("auth_token="))
                .map(|cookie| cookie.trim().strip_prefix("auth_token=").unwrap_or(""))
        });

    if let Some(token) = auth_cookie {
        // Validate JWT token
        let decoding_key = DecodingKey::from_secret(state.config.jwt_secret.as_bytes());
        let validation = Validation::default();
        
        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                // Add user info to request extensions
                request.extensions_mut().insert(token_data.claims);
                Ok(next.run(request).await)
            }
            Err(_) => {
                // Invalid token - redirect to login
                Ok(Redirect::to("/admin/login").into_response())
            }
        }
    } else {
        // No token - redirect to login
        Ok(Redirect::to("/admin/login").into_response())
    }
}

// Optional auth middleware (doesn't redirect, just adds user info if available)
pub async fn optional_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    // Try to get JWT from cookie
    let auth_cookie = request
        .headers()
        .get(header::COOKIE)
        .and_then(|cookie| cookie.to_str().ok())
        .and_then(|cookies| {
            cookies
                .split(';')
                .find(|cookie| cookie.trim().starts_with("auth_token="))
                .map(|cookie| cookie.trim().strip_prefix("auth_token=").unwrap_or(""))
        });

    if let Some(token) = auth_cookie {
        let decoding_key = DecodingKey::from_secret(state.config.jwt_secret.as_bytes());
        let validation = Validation::default();
        
        if let Ok(token_data) = decode::<Claims>(token, &decoding_key, &validation) {
            request.extensions_mut().insert(token_data.claims);
        }
    }
    
    next.run(request).await
}

// Helper to extract user from request
pub fn get_user_from_request(request: &Request) -> Option<&Claims> {
    request.extensions().get::<Claims>()
}