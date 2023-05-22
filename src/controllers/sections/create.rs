use std::error::Error;

pub struct Section<'l> {
    pub title: &'l str,
    pub description: &'l str,
    pub parent_section_id: Option<i32>,
}

pub async fn execute(section: Section<'_>, pool: &sqlx::PgPool) -> Result<(), Box<dyn Error>> {
    let query = "INSERT INTO sections (title, description, parent_section_id) VALUES ($1, $2, $3)";

    sqlx::query(query)
        .bind(section.title)
        .bind(section.description)
        .bind(section.parent_section_id)
        .execute(pool)
        .await?;

    Ok(())
}