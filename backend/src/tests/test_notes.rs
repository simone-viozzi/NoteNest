use crate::routes;
use actix_web::{test, App};
use serial_test::serial;
use sqlx::{Executor, PgPool};

/// Test context containing the database pool and app
struct TestContext {
    pool: PgPool,
}

/// Setup function to create the test database and initialize the app
async fn setup() -> TestContext {
    let admin_url = "postgres://user:password@127.0.0.1:5432/postgres";
    let admin_pool = PgPool::connect(admin_url).await.unwrap();

    // Drop and recreate the test database
    admin_pool
        .execute("DROP DATABASE IF EXISTS notenest_test WITH (FORCE)")
        .await
        .expect("Failed to drop test database");

    admin_pool
        .execute("CREATE DATABASE notenest_test")
        .await
        .expect("Failed to create test database");

    // Connect to the test database
    let test_db_url = "postgres://user:password@127.0.0.1:5432/notenest_test";
    let pool = PgPool::connect(&test_db_url)
        .await
        .expect("Failed to connect to test database");

    // Apply migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to apply migrations");

    TestContext { pool }
}

/// Teardown function to clean up the test database
async fn teardown(ctx: TestContext) {
    drop(ctx.pool); // Close all connections to the database

    let admin_url = "postgres://user:password@127.0.0.1:5432/postgres";
    let admin_pool = PgPool::connect(admin_url).await.unwrap();

    admin_pool
        .execute("DROP DATABASE IF EXISTS notenest_test WITH (FORCE)")
        .await
        .expect("Failed to drop test database");
}

// Tests

#[serial]
#[actix_web::test]
async fn test_create_note() {
    let ctx = setup().await;

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(ctx.pool.clone()))
            .configure(routes::init_routes),
    )
    .await;

    let req = test::TestRequest::post()
        .uri("/notes")
        .set_json(&serde_json::json!({ "title": "Test Note" }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let note = sqlx::query!("SELECT title FROM notes LIMIT 1")
        .fetch_one(&ctx.pool)
        .await
        .unwrap();

    assert_eq!(note.title, "Test Note");

    teardown(ctx).await;
}

#[serial]
#[actix_web::test]
async fn test_get_all_notes() {
    let ctx = setup().await;

    sqlx::query!("INSERT INTO notes (title) VALUES ($1)", "First Note")
        .execute(&ctx.pool)
        .await
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(ctx.pool.clone()))
            .configure(routes::init_routes),
    )
    .await;

    let req = test::TestRequest::get().uri("/notes").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["title"], "First Note");

    teardown(ctx).await;
}

#[serial]
#[actix_web::test]
async fn test_get_note_by_id() {
    let ctx = setup().await;

    // Insert a test note
    let note_id = sqlx::query!(
        "INSERT INTO notes (title) VALUES ($1) RETURNING id",
        "Test Note"
    )
    .fetch_one(&ctx.pool)
    .await
    .unwrap()
    .id;

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(ctx.pool.clone()))
            .configure(routes::init_routes),
    )
    .await;

    // Send a GET request to retrieve the note by ID
    let req = test::TestRequest::get()
        .uri(&format!("/notes/{}", note_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(body["title"], "Test Note");

    teardown(ctx).await;
}

#[serial]
#[actix_web::test]
async fn test_delete_note() {
    let ctx = setup().await;

    // Insert a test note
    let note_id = sqlx::query!(
        "INSERT INTO notes (title) VALUES ($1) RETURNING id",
        "Note to Delete"
    )
    .fetch_one(&ctx.pool)
    .await
    .unwrap()
    .id;

    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(ctx.pool.clone()))
            .configure(routes::init_routes),
    )
    .await;

    // Send a DELETE request to delete the note
    let req = test::TestRequest::delete()
        .uri(&format!("/notes/{}", note_id))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // Verify the note was deleted from the database
    let result = sqlx::query!("SELECT id FROM notes WHERE id = $1", note_id)
        .fetch_optional(&ctx.pool)
        .await
        .unwrap();

    assert!(result.is_none());

    teardown(ctx).await;
}
