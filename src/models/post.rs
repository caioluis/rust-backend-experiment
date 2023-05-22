use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow, Deserialize, Serialize)]
pub struct Post {
    pub id: u32,
    pub topic_id: u32,
    pub character_id: u32,
    pub character_name: &str,
    pub content: &str,
    pub words_count: u16,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
