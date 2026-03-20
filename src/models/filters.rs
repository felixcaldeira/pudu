use serde::Deserialize;
use sqlx::Arguments;

#[derive(Deserialize)]
pub struct Filters {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub order: Option<String>,
    pub descending: Option<bool>,
    pub published: Option<bool>,
}

impl Filters {
    pub fn order(&self) -> &str {
        self.order.as_deref().unwrap_or("_")
    }

    pub fn descending(&self) -> bool {
        self.descending.unwrap_or(false)
    }

    pub fn limit(&self) -> u32 {
        self.limit.unwrap_or(10)
    }

    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1)
    }

    pub fn apply_published(&self, query: &mut String, args: &mut sqlx::mysql::MySqlArguments, prefix: Option<&str>) {
        if let Some(published) = self.published {
            let col = match prefix {
                Some(p) => format!(" AND {}.published = ?", p),
                None    => " AND published = ?".to_string(),
            };
            query.push_str(&col);
            args.add(published);
        }
    }

    pub fn apply_pagination(&self, query: &mut String, args: &mut sqlx::mysql::MySqlArguments) {
        if let Some(limit) = self.limit {
            let offset = (self.page() - 1) * limit;
            query.push_str(" LIMIT ? OFFSET ?");
            args.add(limit);
            args.add(offset);
        }
    }

    pub fn order_clause(&self, allowed: &[&str], default: &str, prefix: Option<&str>) -> String {
        let order = if allowed.contains(&self.order()) { self.order() } else { default };
        let dir = if self.descending() { "DESC" } else { "ASC" };
        match prefix {
            Some(p) => format!(" ORDER BY {}.{} {}", p, order, dir),
            None => format!(" ORDER BY {} {}", order, dir),
        }
    }
}