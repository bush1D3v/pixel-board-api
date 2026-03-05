use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub order_by: Option<String>,
    pub order_direction: Option<String>,
}

impl QueryParams {
    pub fn offset(&self) -> i64 {
        self.offset.unwrap_or(0)
    }

    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }

    pub fn order_by(&self) -> &str {
        self.order_by.as_deref().unwrap_or("created_at")
    }

    pub fn order_direction(&self) -> &str {
        match self.order_direction.as_deref() {
            Some("asc" | "ASC") => "ASC",
            _ => "DESC",
        }
    }
}
