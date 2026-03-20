use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use crate::AppError;
use sqlx::{FromRow, Arguments};
use crate::models::Filters;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ModuleCategory {
    pub id: u32,
    pub image_id: Option<u32>,
    pub slug: String,
    pub title: String,
    pub description: String,
    pub published: Option<i8>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub module_count: Option<i64>,
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

fn build_select(count_expr: &str) -> String {
    format!(
        "SELECT c.*, {} as module_count
         FROM module_categories c
         LEFT JOIN modules m ON m.category_id = c.id",
        count_expr
    )
}

fn module_count_expr(published: Option<bool>) -> &'static str {
    match published {
        Some(true)  => "COUNT(CASE WHEN m.published = 1 THEN 1 END)",
        Some(false) => "COUNT(CASE WHEN m.published = 0 THEN 1 END)",
        None        => "COUNT(m.id)",
    }
}

impl ModuleCategory {
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
        published: Option<bool>,
    ) -> Result<Option<Self>, AppError> {
        let mut args = sqlx::mysql::MySqlArguments::default();
        args.add(id);

        let query = format!("{} WHERE c.id = ? GROUP BY c.id", build_select(module_count_expr(published)));

        Ok(sqlx::query_as_with::<_, Self, _>(&query, args)
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

        let mut query = format!("{} WHERE c.slug = ?", build_select(module_count_expr(published)));

        if let Some(p) = published {
            query.push_str(" AND c.published = ?");
            args.add(p);
        }
        query.push_str(" GROUP BY c.id");

        Ok(sqlx::query_as_with::<_, Self, _>(&query, args)
            .fetch_optional(pool)
            .await?)
    }

    pub async fn find_all(
        pool: &sqlx::MySqlPool,
        filters: Filters,
    ) -> Result<Vec<Self>, AppError> {
        let mut query = format!("{} WHERE 1=1", build_select(module_count_expr(filters.published)));
        let mut args = sqlx::mysql::MySqlArguments::default();

        filters.apply_published(&mut query, &mut args, Some("c"));
        query.push_str(" GROUP BY c.id");
        query.push_str(&filters.order_clause(&["id", "title", "created_at"], "title", Some("c")));
        filters.apply_pagination(&mut query, &mut args);

        Ok(sqlx::query_as_with::<_, Self, _>(&query, args)
            .fetch_all(pool)
            .await?)
    }

    pub async fn count(
        pool: &sqlx::MySqlPool,
        published: Option<bool>,
    ) -> Result<i64, AppError> {
        let mut query = "SELECT COUNT(*) FROM module_categories WHERE 1=1".to_string();
        let mut args = sqlx::mysql::MySqlArguments::default();

        if let Some(published) = published {
            query.push_str(" AND published = ?");
            args.add(published);
        }

        Ok(sqlx::query_scalar_with::<_, i64, _>(&query, args)
            .fetch_one(pool)
            .await?)
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        data: ModuleCategoryCreate,
    ) -> Result<Self, AppError> {
        let result = sqlx::query!(
            "INSERT INTO module_categories (image_id, slug, title, description, published)
             VALUES (?, ?, ?, ?, ?)",
            data.image_id, data.slug, data.title, data.description, data.published,
        )
        .execute(pool)
        .await?;

        Self::find_by_id(pool, result.last_insert_id() as u32, None)
            .await?
            .ok_or(AppError::Internal("Failed to load created module category".into()))
    }

    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: ModuleCategoryUpdate,
    ) -> Result<(), AppError> {
        Self::find_by_id(pool, id, None).await?.ok_or(AppError::NotFound)?;

        let mut qb = sqlx::QueryBuilder::new("UPDATE module_categories SET ");
        let mut sep = qb.separated(", ");

        if let Some(v) = data.title       { sep.push("title = ").push_bind_unseparated(v); }
        if let Some(v) = data.description { sep.push("description = ").push_bind_unseparated(v); }
        if let Some(v) = data.image_id    { sep.push("image_id = ").push_bind_unseparated(v); }

        match data.published {
            Some(v) => { sep.push("published = ").push_bind_unseparated(v); }
            None    => { sep.push("published = false"); }
        }

        qb.push(" WHERE id = ").push_bind(id);
        qb.build().execute(pool).await?;

        Ok(())
    }

    pub async fn delete(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<(), AppError> {
        sqlx::query!("DELETE FROM module_categories WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}