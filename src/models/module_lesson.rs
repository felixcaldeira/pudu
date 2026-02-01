use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::handlers::error::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ModuleLesson {
    pub id: u32,
    pub module_id: Option<u32>, // parent modul
    pub title: String,
    pub position: u32,
}

#[derive(Debug)]
pub struct ModuleLessonCreate {
    pub module_id: u32,
    pub title: String,
    pub position: u32,
}

#[derive(Debug)]
pub struct ModuleLessonUpdate {
    pub title: Option<String>,
    pub position: Option<u32>,
}

impl ModuleLesson {
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                ModuleLesson,
                "SELECT * FROM module_lessons WHERE id = ?",
                id
            )
            .fetch_optional(pool)
            .await?
        )
    }
    pub async fn find_by_module(
        pool: &sqlx::MySqlPool,
        module_id: u32,
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                ModuleLesson,
                "SELECT * FROM module_lessons WHERE module_id = ? ORDER BY position",
                module_id
            )
            .fetch_all(pool)
            .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        data: ModuleLessonCreate,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "INSERT INTO module_lessons (module_id, title, position) VALUES (?, ?, ?)",
            data.module_id,
            data.title,
            data.position
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // ModuleLesson::update
    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: ModuleLessonUpdate
    ) -> Result<(), AppError> {
        ModuleLesson::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE module_lessons SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(title) = data.title {
            separated.push("title = ").push_bind_unseparated(title);
        }
        if let Some(position) = data.position {
            separated.push("position = ").push_bind_unseparated(position);
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
            "DELETE FROM module_lessons WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}