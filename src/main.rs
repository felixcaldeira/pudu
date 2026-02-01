// main.rs
use axum::{
    routing::{get/*, post*/},
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
        .route("/", get(handlers::home::index))
        // .nest("/modules", Router::new()
        //     .route("/", get(handlers::module_list))
        //     .route("/:module_category", get(handlers::module_category))
        //     .route("/:module_category/:slug", get(handlers::module_detail))
        //     .route("/:module_category/:slug/lessons", get(handlers::module_lessons))
        //     .route("/:module_category/:slug/materials", get(handlers::module_materialien))
        // )

        // .route("/authors", get(handlers::authors))
        // .route("/news", get(handlers::news))
        // .route("/workshops", get(handlers::workshops))
        // .route("/about", get(handlers::about))
        // .route("/contact", get(handlers::contact::get))
        // .route("/contact", post(handlers::contact::post))

        // .route("/agb", get(handlers::contact::get))
        // .route("/datenschutz", get(handlers::contact::get))
        // .route("/impressum", get(handlers::contact::get))
        
        // manage routes
        // .nest("/manage", Router::new()
        //     // Public manage routes
        //     .route("/login", get(handlers::manage::show_login))
        //     .route("/login", post(handlers::manage::login))
        //     .route("/logout", post(handlers::manage::logout))
        //     // Protected manage routes
        //     .nest("/", Router::new()
        //         .route("/", get(handlers::manage::dashboard))
        //         .layer(middleware::from_fn_with_state(
        //             state.clone(),
        //             midware::auth::require_auth,
        //         ))
        //     )
        // )

        .route_service("/robots.txt", ServeFile::new(Path::new(&config.static_dir).join("robots.txt")))

        // Static files
        .nest_service("/static", ServeDir::new(&config.static_dir))
        .nest_service("/files", ServeDir::new(&config.files_dir))
        // .nest_service("/images", ...)

        // In your router:
        // .layer(middleware::from_fn_with_state(state.clone(), error_handler))
        .fallback(handler_404)

        // Middleware
        // Global middleware
        .layer(middleware::from_fn_with_state(
            state.clone(),
            midware::auth::optional_auth,
        ))
        .layer(CorsLayer::permissive())
        .layer(DefaultBodyLimit::max(10 * 1024 * 1024)) // 10MB max
        .with_state(state);
    
    // Start server
    let addr = SocketAddr::from((config.host, config.port));
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    tracing::info!("Server running on http://{}", addr);
    
    Ok(())
}