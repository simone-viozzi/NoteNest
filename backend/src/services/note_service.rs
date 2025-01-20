use sqlx::{PgPool, Error};
use crate::models::note::Note;
use uuid::Uuid;

/// Create a new note in the database.
pub async fn create_note(
    pool: &PgPool,
    title: String,
) -> Result<Note, Error> {
    let record = sqlx::query_as!(
        Note,
        r#"
        INSERT INTO notes (title) VALUES ($1)
        RETURNING id, title, created_at, updated_at
        "#,
        title
    )
    .fetch_one(pool)
    .await?;

    Ok(record)
}

/// Retrieve all notes from the database.
pub async fn get_all_notes(pool: &PgPool) -> Result<Vec<Note>, Error> {
    let records = sqlx::query_as!(
        Note,
        r#"
        SELECT id, title, created_at, updated_at
        FROM notes
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(records)
}

/// Retrieve a specific note by ID from the database.
pub async fn get_note_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Note>, Error> {
    let record = sqlx::query_as!(
        Note,
        r#"
        SELECT id, title, created_at, updated_at
        FROM notes
        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

/// Update a note by ID in the database.
pub async fn update_note(
    pool: &PgPool,
    id: Uuid,
    title: String,
) -> Result<Option<Note>, Error> {
    let record = sqlx::query_as!(
        Note,
        r#"
        UPDATE notes
        SET title = $1, updated_at = NOW()
        WHERE id = $2
        RETURNING id, title, created_at, updated_at
        "#,
        title,
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record)
}

/// Delete a note by ID from the database.
pub async fn delete_note(
    pool: &PgPool,
    id: Uuid,
) -> Result<bool, Error> {
    let result = sqlx::query!(
        r#"
        DELETE FROM notes
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    // Check if any rows were affected (i.e., if the note existed and was deleted)
    Ok(result.rows_affected() > 0)
}
