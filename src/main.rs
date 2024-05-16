use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web::Data, App, HttpServer};
use env_logger::Env;

mod routes;
mod services;
mod model;

mod utils;
use routes::signature::{get_signatures_handler, get_image};
use services::db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let db = Database::init().await.unwrap();
    let db_data = Data::new(db);

    let server = HttpServer::new(move || {
        // Create CORS middleware configuration
        let cors = Cors::default()
            .allowed_origin("http://localhost:5174")  // Specify the allowed origin
            .allowed_methods(vec!["GET", "POST"])  // Specify the allowed methods
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);  // Optional: specify max age for preflight requests

        App::new()
            .wrap(Logger::default())
            .wrap(cors)  // Apply CORS middleware globally
            .app_data(db_data.clone())
            .service(get_signatures_handler)
            .service(get_image)
    })
    .bind(("0.0.0.0", 5001))?;

    println!("Server is running on port 5001");

    server.run().await
}
