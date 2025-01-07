use actix_web::{get, middleware::Logger, App, HttpServer, Responder, web};
use dotenvy::dotenv;
use log::{error, info};

mod config;
mod db;
mod models;
mod routes;
mod services;

#[get("/ping")]
async fn ping() -> impl Responder {
    "pong\n"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting the application...");

    // Load environment variables
    dotenv().ok();
    info!("Environment variables loaded.");

    // Initialize database connection
    let pool = match db::init_db().await {
        Ok(pool) => {
            info!("Database connection initialized.");
            pool
        }
        Err(e) => {
            error!("Failed to initialize database connection: {}", e);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Database initialization failed",
            ));
        }
    };

    // Start the HTTP server
    info!(
        "Starting the HTTP server on port {}...",
        config::get_server_port()
    );
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a \"%r\" %U %s %T"))
            .app_data(pool.clone())
            .service(ping)
            .configure(|cfg| {
                info!("Invoking routes::init_routes...");
                routes::init_routes(cfg);
            })
    })
    .bind(("127.0.0.1", config::get_server_port()))?
    .run()
    .await
}
