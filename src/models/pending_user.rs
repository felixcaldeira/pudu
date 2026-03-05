// src/models/user.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use sqlx::FromRow;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::config::Config;
use crate::AppError;
use lettre::{
    message::Mailbox,
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct PendingUser {
    pub id: u32,
    pub email: String,
    pub nanoid: String,
    pub expires_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl PendingUser {
    fn generate_nanoid() -> String {
        use nanoid::nanoid;
        nanoid!(64)
    }

    pub async fn send_verification(
        config: &Config,
        email: &str,
        nanoid: &str,
    ) -> Result<(), AppError> {
        let verification_link = format!("{}/register/{}", config.domain, nanoid);

        let email = Message::builder()
            .from(config.smtp_user.parse::<Mailbox>().unwrap())
            .to(email.parse::<Mailbox>().unwrap())
            .subject("Erstelle dein Konto für das PuDU-Netzwerk")
            .body(format!(
                "Drücke auf den folgenden Link, um dein Konto zu erstellen:\n\n{}",
                verification_link
            ))
            .unwrap();

        let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&config.smtp_host)
            .map_err(|e| AppError::Internal(format!("SMTP config error: {}", e)))?
            .port(config.smtp_port)
            .credentials(Credentials::new(config.smtp_user.clone(), config.smtp_pass.clone()))
            .build();

        mailer.send(email).await
            .map_err(|e| AppError::Internal(format!("Email send failed: {}", e)))?;

        Ok(())
    }

    pub async fn find_all(
        pool: &sqlx::MySqlPool
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                PendingUser,
                "SELECT * FROM pending_users WHERE expires_at > NOW()",
            )
            .fetch_all(pool)
            .await?
        )
    }

    pub async fn find_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                PendingUser,
                "SELECT * FROM pending_users WHERE email = ? AND expires_at > NOW()",
                email
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_by_nanoid(pool: &sqlx::MySqlPool, nanoid: &str) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                PendingUser,
                "SELECT * FROM pending_users WHERE nanoid = ? AND expires_at > NOW()",
                nanoid
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool, 
        config: &Config,
        email: &str,
    ) -> Result<u32, AppError> {
        let nanoid = Self::generate_nanoid();

        let result = sqlx::query!(
            "INSERT INTO pending_users (email, nanoid, expires_at) 
             VALUES (?, ?, NOW() + INTERVAL 28 DAY)",
            email,
            nanoid
        )
        .execute(pool)
        .await?;

        Self::send_verification(config, email, &nanoid).await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        email: &str,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM pending_users WHERE email = ?",
            email
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}