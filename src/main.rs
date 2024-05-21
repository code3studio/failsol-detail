use actix_cors::Cors;
use actix_web::{http, middleware::Logger, web::Data, App, HttpServer};
use env_logger::Env;

mod model;
mod routes;
mod services;

mod utils;
use routes::signature::{get_image, get_signatures_handler,get_specific_signature};
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
        .allow_any_origin()
        .allow_any_header()
        .allow_any_method();
        App::new()
            .wrap(Logger::default())
            .wrap(cors) // Apply CORS middleware globally
            .app_data(db_data.clone())
            .service(get_signatures_handler)
            .service(get_image).service(get_specific_signature)
    })
    .bind(("0.0.0.0", 5001))?;

    println!("Server is running on port 5001");

    server.run().await
}
