use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::AppError;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ModuleLessonSection {
    pub id: u32,
    pub module_lesson_id: u32, // parent modul stunde
    pub title: String,
    pub content: Option<String>,
    pub duration: Option<u32>,
    pub position: u32,
}

#[derive(Debug)]
pub struct ModuleLessonSectionCreate {
    pub module_lesson_id: u32,
    pub title: String,
    pub content: Option<String>,
    pub duration: u32,
    pub position: u32,
}

#[derive(Debug)]
pub struct ModuleLessonSectionUpdate {
    pub title: Option<String>,
    pub content: Option<String>,
    pub duration: Option<u32>,
    pub position: Option<u32>,
}

impl ModuleLessonSection {
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                ModuleLessonSection,
                "SELECT * FROM module_lesson_sections WHERE id = ?",
                id
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn find_by_lesson(
        pool: &sqlx::MySqlPool,
        lesson_id: u32,
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                ModuleLessonSection,
                "SELECT * FROM module_lesson_sections WHERE module_lesson_id = ? ORDER BY position",
                lesson_id
            )
            .fetch_all(pool)
            .await?
        )
    }

    pub async fn find_by_lesson_ids(
        pool: &sqlx::MySqlPool,
        lesson_ids: &[u32],
    ) -> Result<Vec<Self>, AppError> {
        if lesson_ids.is_empty() {
            return Ok(vec![]);
        }

        let placeholders = lesson_ids.iter()
            .map(|_| "?")
            .collect::<Vec<_>>()
            .join(", ");

        let query = format!(
            "SELECT * FROM module_lesson_sections WHERE module_lesson_id IN ({}) ORDER BY module_lesson_id, position",
            placeholders
        );

        let mut query_builder = sqlx::query_as::<_, ModuleLessonSection>(&query);
        
        for id in lesson_ids {
            query_builder = query_builder.bind(id);
        }

        Ok(query_builder.fetch_all(pool).await?)
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        data: ModuleLessonSectionCreate,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO module_lesson_sections
                (module_lesson_id, title, content, duration, position)
            VALUES (?, ?, ?, ?, ?)
            "#,
            data.module_lesson_id,
            data.title,
            data.content,
            data.duration,
            data.position
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: ModuleLessonSectionUpdate
    ) -> Result<(), AppError> {
        ModuleLessonSection::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE module_lesson_sections SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(title) = data.title {
            separated.push("title = ").push_bind_unseparated(title);
        }
        if let Some(content) = data.content {
            separated.push("content = ").push_bind_unseparated(content);
        }
        if let Some(duration) = data.duration {
            separated.push("duration = ").push_bind_unseparated(duration);
        }
        if let Some(position) = data.position {
            separated.push("position = ").push_bind_unseparated(position);
        }

        query_builder.push(" WHERE id = ").push_bind(id);
        query_builder.build().execute(pool).await?;

        Ok(())
    }

    pub async fn batch_update_positions(
        pool: &sqlx::MySqlPool,
        updates: &[(u32, u32)], // Vec of (id, position)
    ) -> Result<(), AppError> {
        if updates.is_empty() {
            return Ok(());
        }

        let ids: Vec<u32> = updates.iter().map(|(id, _)| *id).collect();
        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
        
        let mut case_stmt = String::from("CASE id ");
        for (id, position) in updates {
            case_stmt.push_str(&format!("WHEN {} THEN {} ", id, position));
        }
        case_stmt.push_str("END");

        let query = format!(
            "UPDATE module_lesson_sections SET position = {} WHERE id IN ({})",
            case_stmt, placeholders
        );

        let mut query_builder = sqlx::query(&query);
        for id in ids {
            query_builder = query_builder.bind(id);
        }

        query_builder.execute(pool).await?;

        Ok(())
    }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<(), AppError> {
        sqlx::query!(
            "DELETE FROM module_lesson_sections WHERE id = ?",
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}
