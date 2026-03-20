// src/models/contact.rs
use serde::{Deserialize, Deserializer};
use validator::{Validate, ValidationError};

#[derive(Debug, Deserialize, Validate)]
pub struct Contact {
    #[validate(length(min = 1, message = "Ihr Vor- und Nachname ist benötigt."))]
    pub name: String,
    
    #[validate(email(message = "Ungültige E-Mail Adresse"))]
    pub email: String,
    
    #[validate(custom(function = "validate_phone"))]
    pub tel: Option<String>,
    
    #[validate(length(min = 10, message = "Nachricht muss mindestens 10 Charaktere lang sein."))]
    pub message: String,

    #[validate(required(message = "Sie müssen den Datenschutzrichtlinien zustimmen."))]
    #[validate(custom(function = "validate_datenschutz"))]
    pub datenschutz: Option<String>,
}

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    if phone.is_empty() {
        return Ok(());
    }
    let cleaned: String = phone
        .chars()
        .filter(|c| c.is_numeric() || *c == '+')
        .collect();
    if cleaned.starts_with('+') {
        if cleaned.len() >= 8 && cleaned.len() <= 16 {
            return Ok(());
        }
    } else if cleaned.starts_with("00") {
        if cleaned.len() >= 9 && cleaned.len() <= 17 {
            return Ok(());
        }
    } else {
        if cleaned.len() >= 7 && cleaned.len() <= 15 {
            return Ok(());
        }
    }
    Err(validator::ValidationError::new("Ungültige Telefonnummer."))
}

fn validate_datenschutz(datenschutz: &str) -> Result<(), ValidationError> {
    match datenschutz {
       "on" => Ok(()),
        _ => Err(ValidationError::new(
            "Sie müssen den Datenschutzrichtlinien zustimmen."
        )),
    }
}