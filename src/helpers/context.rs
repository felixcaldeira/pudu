use tera::Context;
use axum::http::Request;
use crate::models::{User};

pub fn base_context(req: &Request<impl Send>) -> Context {
    let mut ctx = Context::new();

    if let Some(user) = req.extensions().get::<Option<User>>() {
        ctx.insert("user", user);
    }

    ctx
}