// src/models/user.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use sqlx::FromRow;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::AppError;
use crate::UserFlags;

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: u32,
    pub image_id: Option<u32>,
    pub username: String,
    pub academic_title: Option<String>,
    pub first_name: String,
    pub last_name: String,
    
    #[sqlx(try_from = "u32")]
    pub flags: UserFlags,

    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct UserCreate {
    pub username: String, 
    pub email: String, 
    pub password: String,
    pub academic_title: String,
    pub first_name: String,
    pub last_name: String,
    pub flags: UserFlags
}

#[derive(Debug)]
pub struct UserUpdate {
    pub username: Option<String>,
    pub academic_title: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub flags: Option<UserFlags>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn find_all(
        pool: &sqlx::MySqlPool
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                User,
                "SELECT * FROM users",
            )
            .fetch_all(pool)
            .await?
        )
    }

    pub async fn find_by_username(pool: &sqlx::MySqlPool, username: &str) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE username = ?",
                username
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_by_email(pool: &sqlx::MySqlPool, email: &str) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE email = ?",
                email
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_by_id(pool: &sqlx::MySqlPool, id: u32) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE id = ?",
                id
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool, 
        data: UserCreate,
    ) -> Result<u32, AppError> {
        let password_hash = hash(data.password, DEFAULT_COST)
            .map_err(|_| AppError::Internal("Failed to hash password".into()))?;

        let result = sqlx::query!(
            "INSERT INTO users (username, email, password_hash, academic_title, first_name, last_name, flags) VALUES (?, ?, ?, ?, ?, ?, ?)",
            data.username,
            data.email,
            password_hash,
            data.academic_title,
            data.first_name,
            data.last_name,
            data.flags.bits(),
        )
        .execute(pool)
        .await?;

        Ok(result.last_insert_id() as u32)
    }

    pub async fn update_password(
        pool: &sqlx::MySqlPool, 
        id: u32,
        current_password: &str,
        password: &str,
    ) -> Result<(), AppError> {
        let user = User::find_by_id(pool, id).await?.ok_or(AppError::NotFound)?;
        if !user.verify_password(current_password) {
            return Err(AppError::BadRequest("Falsches altes Passwort".into()));
        }

        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|_| AppError::Internal("Failed to hash password".into()))?;

        sqlx::query!(
            "UPDATE users SET password_hash = ? WHERE id = ?",
            password_hash,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_image_id(
        pool: &sqlx::MySqlPool,
        id: u32,
        image_id: u32
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            UPDATE users
            SET image_id = ?
            WHERE id = ?
            "#,
            image_id, id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // User::update
    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: UserUpdate
    ) -> Result<(), AppError> {
        User::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE users SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(username) = data.username {
            separated.push("username = ").push_bind_unseparated(username);
        }
        if let Some(academic_title) = data.academic_title {
            separated.push("academic_title = ").push_bind_unseparated(academic_title);
        }
        if let Some(first_name) = data.first_name {
            separated.push("first_name = ").push_bind_unseparated(first_name);
        }
        if let Some(last_name) = data.last_name {
            separated.push("last_name = ").push_bind_unseparated(last_name);
        }
        if let Some(flags) = data.flags {
            separated.push("flags = ").push_bind_unseparated(flags.bits());
        }
        if let Some(email) = data.email {
            separated.push("email = ").push_bind_unseparated(email);
        }

        query_builder.push(" WHERE id = ").push_bind(id);
        query_builder.build().execute(pool).await?;

        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> bool {
        verify(password, &self.password_hash).unwrap_or(false)
    }

    pub fn dummy_verify() {
        let _ = verify("dummy", "$2b$12$invalidhashfortimingprotection000000000000000000000000000");
    }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM users WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}