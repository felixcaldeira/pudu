use serde::{Deserialize, Serialize};
use chrono::{NaiveDateTime};
use crate::AppError;
use sqlx::FromRow;
use crate::models::File;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ModuleMaterial {
    pub id: u32,
    pub module_id: Option<u32>, // parent modul
    pub file_id: u32,
    
    pub title: String,
    pub material_type: String,
    pub position: u32,
}

#[derive(Debug)]
pub struct ModuleMaterialCreate {
    pub module_id: u32,
    pub file_id: u32,
    pub title: String,
    pub material_type: String,
    pub position: u32,
}

#[derive(Debug)]
pub struct ModuleMaterialUpdate {
    pub title: Option<String>,
    pub material_type: Option<String>,
    pub position: Option<u32>,
}

impl ModuleMaterial {
    pub async fn find_by_module(
        pool: &sqlx::MySqlPool,
        module_id: u32,
    ) -> Result<Vec<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                ModuleMaterial,
                "SELECT * FROM module_materials WHERE module_id = ? ORDER BY position",
                module_id
            )
            .fetch_all(pool)
            .await?
        )
    }
    
    pub async fn find_by_id(
        pool: &sqlx::MySqlPool,
        id: u32,
    ) -> Result<Option<Self>, AppError> {
        Ok(
            sqlx::query_as!(
                ModuleMaterial,
                "SELECT * FROM module_materials WHERE id = ?",
                id
            )
            .fetch_optional(pool)
            .await?
        )
    }

    pub async fn create(
        pool: &sqlx::MySqlPool,
        data: ModuleMaterialCreate,
    ) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO module_materials
                (module_id, file_id, title, material_type, position)
            VALUES (?, ?, ?, ?, ?)
            "#,
            data.module_id,
            data.file_id,
            data.title,
            data.material_type,
            data.position
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // ModuleMaterial::update
    pub async fn update(
        pool: &sqlx::MySqlPool,
        id: u32,
        data: ModuleMaterialUpdate
    ) -> Result<(), AppError> {
        ModuleMaterial::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        let mut query_builder = sqlx::QueryBuilder::new("UPDATE module_materials SET ");
        let mut separated = query_builder.separated(", ");

        if let Some(title) = data.title {
            separated.push("title = ").push_bind_unseparated(title);
        }
        if let Some(material_type) = data.material_type {
            separated.push("material_type = ").push_bind_unseparated(material_type);
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
            "UPDATE module_materials SET position = {} WHERE id IN ({})",
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
        let material = ModuleMaterial::find_by_id(pool, id)
            .await?
            .ok_or(AppError::NotFound)?;

        File::delete(pool, material.file_id).await?;

        Ok(())
    }

    pub async fn delete_by_module(
        pool: &sqlx::MySqlPool,
        module_id: u32,
    ) -> Result<(), AppError> {
        let materials = ModuleMaterial::find_by_module(pool, module_id).await?;

        for material in materials {
            File::delete(pool, material.file_id).await?;
        }

        Ok(())
    }
}