use once_cell::sync::Lazy;
use tera::Tera;
use std::env;

pub static TERA: Lazy<Tera> = Lazy::new(|| {
    let template_dir =
        env::var("TEMPLATE_DIR").unwrap_or_else(|_| "./templates".to_string());

    let glob = format!("{}/**/*.html", template_dir);

    Tera::new(&glob)
        .expect("Failed to initialize Tera templates")
});