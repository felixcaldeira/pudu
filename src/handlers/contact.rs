use axum::{
    extract::{Request, State, Form},
    response::{Html, Response, IntoResponse},
};
use tera::Context;
use lettre::{
    Message, SmtpTransport, Transport, Address,
    message::{header::ContentType, Mailbox},
    transport::smtp::{authentication::Credentials}
};
use validator::Validate;
use crate::handlers::AppState;
use crate::AppError;
use crate::TERA;
use crate::models::Contact;
use crate::helpers::base_context;

pub async fn get(
    State(state): State<AppState>,
    request: Request,
) -> Result<Html<String>, AppError> {
    let mut context = base_context(&request);

    context.insert("page_title", "Kontakt");
    
    let html = TERA.render("contact.html", &context)?;
    
    Ok(Html(html))
}

pub async fn post(
    State(state): State<AppState>,
    Form(data): Form<Contact>,
) -> Result<Response, AppError> {
    let mut context = Context::new();
    context.insert("page_title", "Kontakt");
    println!("Raw form payload: {:?}", data);

    if let Err(errors) = data.validate() {
        let error_message = errors
            .field_errors()
            .iter()
            .next()
            .and_then(|(_, errs)| errs.first())
            .and_then(|err| err.message.clone())
            .unwrap_or_else(|| "Invalid input".into());
        
        context.insert("error", &error_message.to_string());
        context.insert("contact_name", &data.name);
        context.insert("contact_email", &data.email);
        context.insert("contact_tel", &data.tel.unwrap_or_default());
        context.insert("contact_message", &data.message);
        
        let html = TERA.render("contact.html", &context)?;
            
        return Ok(Html(html).into_response());
    }
    
    let email_body = format!(
        "Vor- und Nachname: {}\nE-Mail Adresse: {}\nTelefonnummer: {}\n\nNachricht:\n{}",
        data.name,
        data.email,
        data.tel.as_deref().unwrap_or("Telefonnummer nicht gegeben."),
        data.message
    );

    let from_mailbox: Mailbox = format!("PuDU-Netzwerk <{}>", state.config.smtp_user)
        .parse()
        .map_err(|e| AppError::Internal(format!("Failed to parse 'from' mailbox: {}", e)))?;

    let to_mailbox: Mailbox = state.config.smtp_user
        .parse()
        .map_err(|e| AppError::Internal(format!("Failed to parse 'reply_to' mailbox from '{}': {}", data.email, e)))?;

    let email = Message::builder()
        .from(from_mailbox)
        .to(to_mailbox)
        .subject(format!("[PuDU-Netzwerk] Kontaktnachricht von {}", data.name))
        .header(ContentType::TEXT_PLAIN)
        .body(email_body)
        .map_err(|e| AppError::Internal(format!("Failed to build email message: {}", e)))?;    
        
    let creds = Credentials::new(
        state.config.smtp_user.clone(),
        state.config.smtp_pass.clone(),
    );

    let mailer = SmtpTransport::starttls_relay(&state.config.smtp_host)
        .map_err(|e| AppError::Internal(format!("Failed to create SMTP relay for host '{}': {}", state.config.smtp_host, e)))?
        .port(state.config.smtp_port)
        .credentials(creds)
        // .tls(tls)

        .build();
    
    match mailer.send(&email) {
        Ok(_) => {
            context.insert("success", "Nachricht erfolgreich abgeschickt");
            
            let html = TERA.render("contact.html", &context)?;

            Ok(Html(html).into_response())
        }
        Err(e) => {
            context.insert("error", "Schicken der E-Mail nicht erfolgreich.");
            context.insert("contact_name", &data.name);
            context.insert("contact_email", &data.email);
            context.insert("contact_tel", &data.tel.unwrap_or_default());
            context.insert("contact_message", &data.message);
            
            let html = TERA.render("contact.html", &context)?;

            Ok(Html(html).into_response())
        }
    }
}