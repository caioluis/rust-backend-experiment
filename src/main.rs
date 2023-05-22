use std::error::Error;
use sqlx::Row;

mod controllers;

use controllers::sections::create::Section;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // build our application with a single route
    let url = "postgresql://postgres:pass@localhost:5432/postgres";
    let pool = sqlx::postgres::PgPool::connect(url).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    let section = Section {
        title: "Título",
        description: "descrição",
        parent_section_id: None,
    };

    controllers::sections::create::execute(section, &pool).await?;

    Ok(())
}