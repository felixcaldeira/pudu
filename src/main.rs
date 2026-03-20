// main.rs
use axum::{
    routing::{get, post, put, delete},
    Router,
    extract::DefaultBodyLimit,
    middleware
};
use tower_http::{services::ServeDir, services::ServeFile, cors::CorsLayer};
use std::net::{SocketAddr};
use std::path::Path;

mod config;
mod database;
mod models;
mod handlers;
mod midware;
mod templates;
mod helpers;

use models::UserFlags;
use midware::auth::{intersects_flag, contains_flag};
use config::Config;
use handlers::error::handler_404;
pub use handlers::error::AppError;
pub use templates::TERA;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    let config = Config::from_env()?;
    let db = database::setup(&config.database_url).await?;
    database::migrate(&db).await?;
    
    let state = handlers::AppState {
        db,
        config: config.clone(),
    };
    
    let app = Router::new()
        .route("/", get(handlers::home::get))
        .nest("/contact", Router::new()
            .route("/", get(handlers::contact::get))
            .route("/", post(handlers::contact::post))
        )
        .nest("/modules", Router::new()
            .route("/", get(handlers::modules::get_all))
            .route("/:module_category", get(handlers::modules::get))
            .route("/:module_category/:module_slug", get(handlers::single_module::get))
            .route("/:module_category/:module_slug/lessons", get(handlers::single_module::get_lessons))
            .route("/:module_category/:module_slug/materials", get(handlers::single_module::get_materials))
        )

        // .route("/authors", get(handlers::authors))
        // .route("/news", get(handlers::news))
        // .route("/workshops", get(handlers::workshops))
        // .route("/about", get(handlers::about))
        // .route("/contact", get(handlers::contact::get))
        // .route("/contact", post(handlers::contact::post))

        // .route("/agb", get(handlers::contact::get))
        // .route("/datenschutz", get(handlers::contact::get))
        // .route("/impressum", get(handlers::contact::get))
        
        .nest("/login", Router::new() 
            .route("/", get(handlers::login::get))
            .route("/", post(handlers::login::post))
            .layer(middleware::from_fn_with_state(
                state.clone(), 
                midware::auth::redirect_if_authenticated
            ))
        )
        .nest("/register", Router::new() 
            .route("/:nanoid", get(handlers::register::get))
            .route("/:nanoid", post(handlers::register::post))
            .layer(middleware::from_fn_with_state(
                state.clone(), 
                midware::auth::redirect_if_authenticated
            ))
        )
        .route("/logout", post(handlers::login::logout))
        .nest("/manage", Router::new()
            // Protected manage routes
            .route("/", get(handlers::manage::dashboard::get))
            .nest("/users", Router::new()
                .route("/", get(handlers::manage::users::get))
                .route("/", post(handlers::manage::users::post))
                .route("/:user_id", delete(handlers::manage::single_user::delete))
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    |state, req, next| intersects_flag(state, req, next, UserFlags::MANAGE_USERS),
                ))
            )
            .nest("/modules", Router::new()
                .route("/", get(handlers::manage::module_categories::get))
                .route("/", post(handlers::manage::module_categories::post))
                .route("/:category_slug", put(handlers::manage::module_categories::put))
                .route("/:category_slug", delete(handlers::manage::module_categories::delete))
                .nest("/:category_slug", Router::new()
                    .route("/", get(handlers::manage::modules::get))
                    .route("/", post(handlers::manage::modules::post))
                    .route("/:module_slug", get(handlers::manage::single_module::get))
                    .route("/:module_slug", put(handlers::manage::single_module::put))
                    .route("/:module_slug", delete(handlers::manage::single_module::delete))
                    .route("/:module_slug/lessons", get(handlers::manage::single_module_lessons::get))
                    .route("/:module_slug/lessons", post(handlers::manage::single_module_lessons::post))
                    .route("/:module_slug/lessons/reorder", put(handlers::manage::single_module_lessons::reorder))
                    .route("/:module_slug/lessons/:lesson_id", put(handlers::manage::single_lesson::put))
                    .route("/:module_slug/lessons/:lesson_id", delete(handlers::manage::single_lesson::delete))
                    .route("/:module_slug/lessons/:lesson_id/sections", post(handlers::manage::single_lesson_sections::post))
                    .route("/:module_slug/lessons/:lesson_id/sections/reorder", put(handlers::manage::single_lesson_sections::reorder))
                    .route("/:module_slug/lessons/:lesson_id/sections/:section_id", put(handlers::manage::single_section::put))
                    .route("/:module_slug/lessons/:lesson_id/sections/:section_id", delete(handlers::manage::single_section::delete))
                    .route("/:module_slug/materials", get(handlers::manage::single_module_materials::get))
                    .route("/:module_slug/materials", post(handlers::manage::single_module_materials::post))
                    .route("/:module_slug/materials/reorder", put(handlers::manage::single_module_materials::reorder))
                    .route("/:module_slug/materials/:material_id", put(handlers::manage::single_module_materials::put))
                    .route("/:module_slug/materials/:material_id", delete(handlers::manage::single_module_materials::delete))
                )
                .layer(middleware::from_fn_with_state(
                    state.clone(),
                    |state, req, next| intersects_flag(state, req, next, UserFlags::MANAGE_MODULES),
                ))
            )
    //         .route("/users", get(handlers::manage::dashboard))
    //         .route("/news", get(handlers::manage::dashboard))
    //         .route("/workshops", get(handlers::manage::dashboard))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                midware::auth::require_auth,
            ))
            // .layer(middleware::from_fn_with_state(
            //     state.clone(),
            //     |state, req, next| intersects_flag(state, req, next, UserFlags::ADMIN),
            // ))
        )

        .nest("/debug/errors", Router::new()
            .route("/not-found", get(|| async {
                Err::<(), _>(AppError::NotFound)
            }))
            .route("/unauthorized", get(|| async {
                Err::<(), _>(AppError::Unauthorized)
            }))
            .route("/bad-request", get(|| async {
                Err::<(), _>(AppError::BadRequest("Invalid input".into()))
            }))
            .route("/db", get(|| async {
                Err::<(), _>(AppError::Database(
                    sqlx::Error::RowNotFound
                ))
            }))
            .route("/internal", get(|| async {
                Err::<(), _>(AppError::Internal("Something exploded".into()))
            }))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                |state, req, next| contains_flag(state, req, next, UserFlags::ADMIN),
            ))
        )

        .route_service("/robots.txt", ServeFile::new(Path::new(&config.static_dir).join("robots.txt")))

        // Static files
        .nest_service("/static", ServeDir::new(&config.static_dir))
        .nest_service("/files", ServeDir::new(&config.files_dir))
        .nest_service("/images", ServeDir::new(&config.images_dir))

        // In your router:
        // .layer(middleware::from_fn_with_state(state.clone(), error_handler))
        .fallback(handler_404)

        // Middleware
        // Global middleware
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB max
        .layer(middleware::from_fn(midware::error_renderer))
        .layer(middleware::from_fn_with_state(
            state.clone(), 
            midware::auth::clear_stale_auth_cookie
        ))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            midware::auth::optional_auth,
        ))
        .with_state(state);
    
    // Start server
    let addr = SocketAddr::from((config.host, config.port));
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    tracing::info!("Server running on http://{}", addr);
    
    Ok(())
}