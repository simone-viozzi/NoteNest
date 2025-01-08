use actix_web::{web, HttpResponse, Responder};

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

async fn get_notes() -> impl Responder {
    HttpResponse::Ok().json("Retrieve all notes")
}

async fn get_note(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner(); // Access the inner value
    HttpResponse::Ok().json(format!("Retrieve note with id {}", id))
}

async fn create_note() -> impl Responder {
    log::debug!("create_note handler invoked");
    HttpResponse::Ok().json("Create a new note")
}

async fn update_note(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner(); // Access the inner value
    HttpResponse::Ok().json(format!("Update note with id {}", id))
}

async fn delete_note(path: web::Path<u64>) -> impl Responder {
    let id = path.into_inner(); // Access the inner value
    HttpResponse::Ok().json(format!("Delete note with id {}", id))
}
