use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

/// Initializes checklist-related routes.
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    log::debug!("Registering /notes/id/items routes...");
    cfg.service(
        web::scope("/notes/{id}/items") // Base path for all checklist routes
            .service(
                web::resource("")
                    .route(web::get().to(get_items)) // GET /notes/{id}/items
                    .route(web::post().to(create_item)), // POST /notes/{id}/items
            )
            .service(
                web::resource("/{item_id}")
                    .route(web::put().to(update_item)) // PUT /notes/{id}/items/{item_id}
                    .route(web::delete().to(delete_item)), // DELETE /notes/{id}/items/{item_id}
            ),
    );
}

async fn get_items(path: web::Path<u64>) -> impl Responder {
    let note_id = path.into_inner(); // Access the inner value
    HttpResponse::Ok().json(format!("Retrieve items for note {}", note_id))
}

async fn create_item(path: web::Path<u64>) -> impl Responder {
    let note_id = path.into_inner(); // Access the inner value
    HttpResponse::Ok().json(format!("Create item for note {}", note_id))
}

async fn update_item(path: web::Path<(u64, u64)>) -> impl Responder {
    let (note_id, item_id) = path.into_inner(); // Access the inner tuple
    HttpResponse::Ok().json(format!("Update item {} for note {}", item_id, note_id))
}

async fn delete_item(path: web::Path<(u64, u64)>) -> impl Responder {
    let (note_id, item_id) = path.into_inner(); // Access the inner tuple
    HttpResponse::Ok().json(format!("Delete item {} for note {}", item_id, note_id))
}
