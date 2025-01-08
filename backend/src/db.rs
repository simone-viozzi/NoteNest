use sqlx::{PgPool, migrate::Migrator};

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = crate::config::get_database_url();
    let pool = PgPool::connect(&database_url).await?;
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}
