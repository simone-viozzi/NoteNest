use actix_web::{web, HttpResponse, Responder};
use crate::services::note_service;
use sqlx::PgPool;
use uuid::Uuid;
use serde::Deserialize;
use serde_json::json;

/// Initializes note-related routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    log::debug!("Registering /notes routes...");
    cfg.service(
        web::scope("/notes") // Base path for all note routes
            .service(
                web::resource("")
                    .route(web::get().to(get_notes)) // GET /notes
                    .route(web::post().to(create_note)), // POST /notes
            )
            .service(
                web::resource("/{id}")
                    .route(web::get().to(get_note)) // GET /notes/{id}
                    .route(web::put().to(update_note)) // PUT /notes/{id}
                    .route(web::delete().to(delete_note)), // DELETE /notes/{id}
            ),
    );
}

/// Request payload for creating or updating a note.
#[derive(Deserialize)]
struct CreateNoteRequest {
    title: String,
}

/// GET /notes
async fn get_notes(pool: web::Data<PgPool>) -> impl Responder {
    match note_service::get_all_notes(pool.get_ref()).await {
        Ok(notes) => HttpResponse::Ok().json(notes),
        Err(err) => {
            log::error!("Failed to retrieve notes: {}", err);
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to retrieve notes" }))
        }
    }
}

/// GET /notes/{id}
async fn get_note(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();

    match note_service::get_note_by_id(pool.get_ref(), id).await {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => {
            log::warn!("Note with id {} not found", id);
            HttpResponse::NotFound().json(json!({ "error": "Note not found" }))
        }
        Err(err) => {
            log::error!("Failed to retrieve note: {}", err);
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to retrieve note" }))
        }
    }
}

/// POST /notes
async fn create_note(
    pool: web::Data<PgPool>,
    note_data: web::Json<CreateNoteRequest>,
) -> impl Responder {
    // Validate title length
    if note_data.title.len() > 256 {
        log::warn!("Validation failed: title exceeds 256 characters");
        return HttpResponse::BadRequest().json(json!({ "error": "Title must not exceed 256 characters" }));
    }

    match note_service::create_note(pool.get_ref(), note_data.title.clone()).await {
        Ok(note) => HttpResponse::Ok().json(note),
        Err(err) => {
            log::error!("Failed to create note: {}", err);
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to create note" }))
        }
    }
}

/// PUT /notes/{id}
async fn update_note(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
    note_data: web::Json<CreateNoteRequest>,
) -> impl Responder {
    let id = path.into_inner();

    // Validate title length
    if note_data.title.len() > 256 {
        log::warn!("Validation failed: title exceeds 256 characters");
        return HttpResponse::BadRequest().json(json!({ "error": "Title must not exceed 256 characters" }));
    }

    match note_service::update_note(pool.get_ref(), id, note_data.title.clone()).await {
        Ok(Some(note)) => HttpResponse::Ok().json(note),
        Ok(None) => {
            log::warn!("Note with id {} not found", id);
            HttpResponse::NotFound().json(json!({ "error": "Note not found" }))
        }
        Err(err) => {
            log::error!("Failed to update note: {}", err);
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to update note" }))
        }
    }
}

/// DELETE /notes/{id}
async fn delete_note(
    pool: web::Data<PgPool>,
    path: web::Path<Uuid>,
) -> impl Responder {
    let id = path.into_inner();

    match note_service::delete_note(pool.get_ref(), id).await {
        Ok(true) => HttpResponse::Ok().json(json!({ "message": "Note deleted successfully" })),
        Ok(false) => {
            log::warn!("Note with id {} not found", id);
            HttpResponse::NotFound().json(json!({ "error": "Note not found" }))
        }
        Err(err) => {
            log::error!("Failed to delete note: {}", err);
            HttpResponse::InternalServerError().json(json!({ "error": "Failed to delete note" }))
        }
    }
}
