// src/models/module.rs
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::AppError;
use sqlx::{FromRow, Arguments};
use crate::models::{ModuleMaterial, Filters, GradeFlags};

#[derive(Debug, Serialize, Deserialize, FromRow, Clone)]
pub struct Module {
    pub id: u32,
    pub image_id: Option<u32>,
    pub user_id: Option<u32>,
    pub category_id: Option<u32>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub content: Option<String>,

    #[sqlx(try_from = "u32")]
    pub grade_flags: GradeFlags,
    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,

    pub lesson_count: Option<i64>,
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
    pub grade_flags: GradeFlags,
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
    pub grade_flags: Option<GradeFlags>,
    pub published: Option<i8>,
}

const BASE_SELECT: &str = "
    SELECT m.*, COUNT(l.id) as lesson_count
    FROM modules m
    LEFT JOIN module_lessons l ON l.module_id = m.id
";

impl Module {
    pub async fn find_by_id(pool: &sqlx::MySqlPool, id: u32) -> Result<Option<Self>, AppError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        args.add(id);
        Ok(sqlx::query_as_with::<_, Self, _>(
            &format!("{} WHERE m.id = ? GROUP BY m.id", BASE_SELECT),
            args,
        )
        .fetch_optional(pool)
        .await?)
    }

    pub async fn find_by_slug(
        pool: &sqlx::MySqlPool,
        slug: &str,
        published: Option<bool>,
    ) -> Result<Option<Self>, AppError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        args.add(slug);
        let mut query = format!("{} WHERE m.slug = ?", BASE_SELECT);
        if let Some(published) = published {
            query.push_str(" AND m.published = ?");
            args.add(published);
        }
        query.push_str(" GROUP BY m.id");
        Ok(sqlx::query_as_with::<_, Self, _>(&query, args)
            .fetch_optional(pool)
            .await?)
    }

    pub async fn find_by_category(
        pool: &sqlx::MySqlPool,
        category_id: u32,
        filters: Filters,
    ) -> Result<Vec<Self>, AppError> {
        let mut query = format!("{} WHERE m.category_id = ?", BASE_SELECT);
        let mut args = sqlx::mysql::MySqlArguments::default();
        args.add(category_id);
        filters.apply_published(&mut query, &mut args, Some("m"));
        query.push_str(" GROUP BY m.id");
        query.push_str(&filters.order_clause(&["id", "title", "created_at"], "title", Some("m")));
        filters.apply_pagination(&mut query, &mut args);
        Ok(sqlx::query_as_with::<_, Self, _>(&query, args)
            .fetch_all(pool)
            .await?)
    }

    pub async fn find_all(pool: &sqlx::MySqlPool) -> Result<Vec<Self>, AppError> {
        let query = format!("{} GROUP BY m.id ORDER BY m.created_at DESC", BASE_SELECT);
        Ok(sqlx::query_as_with::<_, Self, _>(&query, sqlx::mysql::MySqlArguments::default())
            .fetch_all(pool)
            .await?)
    }

    pub async fn count_by_category(
        pool: &sqlx::MySqlPool,
        category_id: u32,
        published: Option<bool>,
    ) -> Result<i64, AppError> {
        let mut query = "SELECT COUNT(*) FROM modules WHERE category_id = ?".to_string();
        let mut args = sqlx::mysql::MySqlArguments::default();
        args.add(category_id);
        if let Some(published) = published {
            query.push_str(" AND published = ?");
            args.add(published);
        }
        Ok(sqlx::query_scalar_with::<_, i64, _>(&query, args)
            .fetch_one(pool)
            .await?)
    }

    pub async fn create(pool: &sqlx::MySqlPool, data: ModuleCreate) -> Result<Self, AppError> {
        let result = sqlx::query!(
            "INSERT INTO modules
                (image_id, user_id, category_id, slug, title, description, content, grade_flags, published)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
            data.image_id, data.user_id, data.category_id, data.slug,
            data.title, data.description, data.content, data.grade_flags.bits(), data.published,
        )
        .execute(pool)
        .await?;

        Self::find_by_id(pool, result.last_insert_id() as u32)
            .await?
            .ok_or(AppError::Internal("Failed to load created module".into()))
    }

    pub async fn update(pool: &sqlx::MySqlPool, id: u32, data: ModuleUpdate) -> Result<(), AppError> {
        Self::find_by_id(pool, id).await?.ok_or(AppError::NotFound)?;

        let mut qb = sqlx::QueryBuilder::new("UPDATE modules SET ");
        let mut sep = qb.separated(", ");

        if let Some(v) = data.title       { sep.push("title = ").push_bind_unseparated(v); }
        if let Some(v) = data.description { sep.push("description = ").push_bind_unseparated(v); }
        if let Some(v) = data.content     { sep.push("content = ").push_bind_unseparated(v); }
        if let Some(v) = data.category_id { sep.push("category_id = ").push_bind_unseparated(v); }
        if let Some(v) = data.image_id    { sep.push("image_id = ").push_bind_unseparated(v); }
        if let Some(v) = data.user_id     { sep.push("user_id = ").push_bind_unseparated(v); }
        if let Some(v) = data.grade_flags { sep.push("grade_flags = ").push_bind_unseparated(v.bits()); }
        match data.published {
            Some(v) => { sep.push("published = ").push_bind_unseparated(v); }
            None    => { sep.push("published = false"); }
        }

        qb.push(" WHERE id = ").push_bind(id);
        qb.build().execute(pool).await?;
        Ok(())
    }

    pub async fn delete(pool: &sqlx::MySqlPool, id: u32) -> Result<(), AppError> {
        ModuleMaterial::delete_by_module(pool, id).await?;
        sqlx::query!("DELETE FROM modules WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}