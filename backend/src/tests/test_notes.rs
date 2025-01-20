use crate::routes;
use actix_web::{test, App};
use serial_test::serial;
use sqlx::{Executor, PgPool};
use uuid::Uuid;

/// Shared database pool and Actix app context for the test module
use actix_service::Service;

// TODO fix tests!!


struct TestContext {
    pool: PgPool,
    app: Service<test::TestRequest>,
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

    // Initialize the Actix app
    let app = test::init_service(
        App::new()
            .app_data(actix_web::web::Data::new(pool.clone()))
            .configure(routes::init_routes),
    )
    .await;

    TestContext { pool, app }
}

/// Teardown function to clean up the test database
async fn teardown(ctx: TestContext) {
    let TestContext { pool, .. } = ctx;
    drop(pool); // Close all connections to the database

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

    // Use the API to create a note
    let req = test::TestRequest::post()
        .uri("/notes")
        .set_json(&serde_json::json!({ "title": "Test Note" }))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;
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

    // Use the API to create a note
    let req = test::TestRequest::post()
        .uri("/notes")
        .set_json(&serde_json::json!({ "title": "First Note" }))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;
    assert_eq!(resp.status(), 200);

    // Use the API to retrieve all notes
    let req = test::TestRequest::get().uri("/notes").to_request();
    let resp = test::call_service(&ctx.app, req).await;
    assert_eq!(resp.status(), 200);

    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert_eq!(body.len(), 1);
    assert_eq!(body[0]["title"], "First Note");

    teardown(ctx).await;
}

#[serial]
#[actix_web::test]
async fn test_delete_note() {
    let ctx = setup().await;

    // Use the API to create a note
    let req = test::TestRequest::post()
        .uri("/notes")
        .set_json(&serde_json::json!({ "title": "Note to Delete" }))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;
    assert_eq!(resp.status(), 200);

    let note_id: Uuid = test::read_body_json(resp).await["id"]
        .as_str()
        .unwrap()
        .parse()
        .unwrap();

    // Use the API to delete the note
    let req = test::TestRequest::delete()
        .uri(&format!("/notes/{}", note_id))
        .to_request();

    let resp = test::call_service(&ctx.app, req).await;
    assert_eq!(resp.status(), 200);

    // Verify the note was deleted
    let result = sqlx::query!("SELECT id FROM notes WHERE id = $1", note_id)
        .fetch_optional(&ctx.pool)
        .await
        .unwrap();

    assert!(result.is_none());

    teardown(ctx).await;
}
