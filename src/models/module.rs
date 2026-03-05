// src/models/module.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::AppError;
use sqlx::FromRow;
use crate::models::ModuleMaterial;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Module {
    pub id: u32,
    pub image_id: Option<u32>, // banner bild
    pub user_id: Option<u32>, // autor:in
    pub category_id: Option<u32>, // fachgruppe

    pub slug: String, // seo-freundliche-domain
    pub title: String,
    pub description: String,
    pub content: Option<String>, // inhalt in markdown
    pub grade_flags: u32, // bitfield von klassen

    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug)]
pub struct ModuleCreate {
    pub image_id: Option<u32>,
    pub user_id: Option<u32>,
    pub category_id: Option<u32>,
    pub slug: String, 
    pub title: String,
    pub description: String,
    pub content: Option<String>,
    pub grade_flags: u32,
    pub published: Option<i8>,
}

#[derive(Debug)]
pub struct ModuleUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub category_id: Option<u32>,
    pub image_id: Option<u32>,
    pub user_id: Option<u32>,
    pub grade_flags: Option<u32>,
    pub published: Option<i8>,
}

impl Module {
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                Module,
                "SELECT * FROM modules WHERE id = ?",
                id
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_by_slug(
        pool: &sqlx::MySqlPool,
        slug: &str,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                Module,
                "SELECT * FROM modules WHERE slug = ?",
                slug
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_by_category(
        pool: &sqlx::MySqlPool,
        category_id: u32,
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                Module,
                "SELECT * FROM modules WHERE category_id = ? ORDER BY created_at DESC",
                category_id
            )
            .fetch_all(pool)
            .await?
        )
    }
    pub async fn find_all(
        pool: &sqlx::MySqlPool,
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                    Module, 
                    "SELECT * FROM modules ORDER BY created_at DESC"
                )
                .fetch_all(pool)
                .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        data: ModuleCreate,
    ) -> Result<Self, AppError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO modules
                (image_id, user_id, category_id, slug, title, description, content, grade_flags, published)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            data.image_id,
            data.user_id,
            data.category_id,
            data.slug,
            data.title,
            data.description,
            data.content,
            data.grade_flags,
            data.published,
        )
        .execute(pool)
        .await?;

        let id = result.last_insert_id() as u32;

        Self::find_by_id(pool, id)
            .await?
            .ok_or(AppError::Internal("Failed to load created module".into()))
    }

    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: ModuleUpdate
    ) -> Result<(), AppError> {
        // Verify the module exists
        Module::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE modules SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(title) = data.title {
            separated.push("title = ").push_bind_unseparated(title);
        }
        if let Some(description) = data.description {
            separated.push("description = ").push_bind_unseparated(description);
        }
        if let Some(content) = data.content {
            separated.push("content = ").push_bind_unseparated(content);
        }
        if let Some(category_id) = data.category_id {
            separated.push("category_id = ").push_bind_unseparated(category_id);
        }
        if let Some(image_id) = data.image_id {
            separated.push("image_id = ").push_bind_unseparated(image_id);
        }
        if let Some(user_id) = data.user_id {
            separated.push("user_id = ").push_bind_unseparated(user_id);
        }
        if let Some(grade_flags) = data.grade_flags {
            separated.push("grade_flags = ").push_bind_unseparated(grade_flags);
        }
        if let Some(published) = data.published {
            separated.push("published = ").push_bind_unseparated(published);
        } else {
            separated.push("published = false");
        }

        query_builder.push(" WHERE id = ").push_bind(id);
        query_builder.build().execute(pool).await?;

        Ok(())
    }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<(), AppError> {
        ModuleMaterial::delete_by_module(pool, id).await?;

        sqlx::query!(
            "DELETE FROM modules WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}