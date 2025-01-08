#[cfg(test)]
mod tests {
    use sqlx::{PgPool, query};
    use uuid::Uuid;

    #[actix_web::test]
    async fn test_notes_table() {
        let pool = PgPool::connect("postgres://user:password@127.0.0.1:5432/notenest").await.unwrap();

        // Test inserting a note
        let note_id = Uuid::new_v4();
        query!(
            "INSERT INTO notes (id, title) VALUES ($1, $2)",
            note_id,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        // Test retrieving the note
        let row = query!("SELECT title FROM notes WHERE id = $1", note_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.title, "Test Note");
    }

    #[actix_web::test]
    async fn test_checklist_items_table() {
        let pool = PgPool::connect("postgres://user:password@127.0.0.1:5432/notenest").await.unwrap();

        // Test inserting a checklist item
        let note_id = Uuid::new_v4();
        let item_id = Uuid::new_v4();

        query!(
            "INSERT INTO notes (id, title) VALUES ($1, $2)",
            note_id,
            "Test Note"
        )
        .execute(&pool)
        .await
        .unwrap();

        query!(
            "INSERT INTO checklist_items (id, note_id, content, is_checked) VALUES ($1, $2, $3, $4)",
            item_id,
            note_id,
            "Test Item",
            false
        )
        .execute(&pool)
        .await
        .unwrap();

        // Test retrieving the checklist item
        let row = query!("SELECT content FROM checklist_items WHERE id = $1", item_id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert_eq!(row.content, "Test Item");
    }
}
