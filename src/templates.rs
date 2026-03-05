use once_cell::sync::Lazy;
use std::env;
use std::collections::HashMap;
use tera::{Tera, Value, Result as TeraResult};

pub static TERA: Lazy<Tera> = Lazy::new(|| {
    let template_dir =
        env::var("TEMPLATE_DIR").unwrap_or_else(|_| "./templates".to_string());
    let glob = format!("{}/**/*.html", template_dir);

    let mut tera = Tera::new(&glob).expect("Failed to initialize Tera templates");

    tera.register_function(
        "has_flag",
        |args: &HashMap<String, Value>| -> TeraResult<Value> {
            let flags = args.get("flags")
                .ok_or("Missing 'flags' argument")?
                .as_u64()
                .ok_or("flags must be a u64")?;

            let bit = args.get("bit")
                .ok_or("Missing 'bit' argument")?
                .as_u64()
                .ok_or("bit must be a u64")?;

            Ok((flags & bit != 0).into())
        },
    );

    tera
});