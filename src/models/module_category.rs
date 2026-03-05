use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ModuleCategory {
    pub id: u32,
    pub image_id: Option<u32>, // banner bild

    pub slug: String, // seo-freundliche-domain
    pub title: String,
    pub description: String,

    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}


#[derive(Debug, Deserialize)]
pub struct ModuleCategoryCreate {
    pub image_id: Option<u32>,
    pub slug: String, 
    pub title: String,
    pub description: String,
    pub published: Option<i8>,
}

#[derive(Debug, Deserialize)]
pub struct ModuleCategoryUpdate {
    pub title: Option<String>,
    pub description: Option<String>,
    pub image_id: Option<u32>,
    pub published: Option<i8>,
}

impl ModuleCategory {
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                ModuleCategory,
                "SELECT * FROM module_categories WHERE id = ?",
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
                ModuleCategory,
                "SELECT * FROM module_categories WHERE slug = ?",
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
                    ModuleCategory, 
                    "SELECT * FROM module_categories ORDER BY title ASC"
                )
                .fetch_all(pool)
                .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        data: ModuleCategoryCreate,
    ) -> Result<Self, AppError> {
        let result = sqlx::query!(
            r#"
            INSERT INTO module_categories
                (image_id, slug, title, description, published)
            VALUES (?, ?, ?, ?, ?)
            "#,
            data.image_id,
            data.slug,
            data.title,
            data.description,
            data.published,
        )
        .execute(pool)
        .await?;

        let id = result.last_insert_id() as u32;

        Self::find_by_id(pool, id)
            .await?
            .ok_or(AppError::Internal("Failed to load created module category".into()))
    }

    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: ModuleCategoryUpdate
    ) -> Result<(), AppError> {
        // Verify the module exists
        ModuleCategory::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE module_categories SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(title) = data.title {
            separated.push("title = ").push_bind_unseparated(title);
        }
        if let Some(description) = data.description {
            separated.push("description = ").push_bind_unseparated(description);
        }
        if let Some(image_id) = data.image_id {
            separated.push("image_id = ").push_bind_unseparated(image_id);
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
        sqlx::query!(
            "DELETE FROM module_categories WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}