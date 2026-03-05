// src/models/article.rs
use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Article {
    pub id: u32,
    pub image_id: Option<u32>,
    
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: Option<String>,

    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>, 
}

#[derive(Debug)]
pub struct ArticleCreate {
    pub image_id: u32,
    pub title: String,
    pub slug: String,
    pub description: String,
    pub content: Option<String>,
    pub published: i8,
}

#[derive(Debug)]
pub struct ArticleUpdate {
    pub image_id: Option<u32>,
    pub title: Option<String>,
    pub slug: Option<String>,
    pub description: Option<String>,
    pub content: Option<String>,
    pub published: Option<i8>,
}

impl Article {
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                Article,
                "SELECT * FROM articles WHERE id = ?",
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
                Article,
                "SELECT * FROM articles WHERE slug = ?",
                slug
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_all(
        pool: &sqlx::MySqlPool,
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                    Article, 
                    "SELECT * FROM articles ORDER BY created_at DESC"
                )
                .fetch_all(pool)
                .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        data: ArticleCreate,
    ) -> Result<Self, AppError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO articles (image_id, title, slug, description, content, published)
            VALUES (?, ?, ?, ?, ?, ?)
            "#,
            data.image_id,
            data.title,
            data.slug,
            data.description,
            data.content,
            data.published,
        )
        .execute(pool)
        .await?;

        let id = result.last_insert_id() as u32;

        Self::find_by_id(pool, id)
            .await?
            .ok_or(AppError::Internal("Failed to load created article".into()))
    }

    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: ArticleUpdate
    ) -> Result<(), AppError> {
        // Verify the module exists
        Article::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE articles SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(image_id) = data.image_id {
            separated.push("image_id = ").push_bind_unseparated(image_id);
        }
        if let Some(slug) = data.slug {
            separated.push("slug = ").push_bind_unseparated(slug);
        }
        if let Some(title) = data.title {
            separated.push("title = ").push_bind_unseparated(title);
        }
        if let Some(description) = data.description {
            separated.push("description = ").push_bind_unseparated(description);
        }
        if let Some(content) = data.content {
            separated.push("content = ").push_bind_unseparated(content);
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
    
    // ModuleUpdate {
    //     title: Some("New Title".to_string()),
    //     published: Some(1),
    //     description: None,
    //     content: None,
    //     category_id: None,
    //     image_id: None,
    //     grade_flags: None,
    // }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM articles WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}