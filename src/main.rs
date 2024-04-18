mod model;
mod services;
mod routes;
mod utils;

use env_logger::Env;
use actix_web::{get, middleware::Logger, web::Data, App, HttpResponse, HttpServer, Responder};
use routes::signature::get_signature;
use services::db::Database;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // match Database::check_connection().await {
    //     Ok(()) => println!("Connection to MongoDB established successfully"),
    //     Err(err) => {
    //         eprintln!("Failed to establish connection to MongoDB: {}", err);
    //         // return Err(err.into());
    //     }
    // };
    let db = Database::init().await.unwrap();
    let db_data = Data::new(db);

    let server = HttpServer::new(move || {
        App::new().wrap(Logger::default()).app_data(db_data.clone()).service(get_signature)
    }).bind(("0.0.0.0",5001))?;

    // Log a message indicating that the server is running
    println!("Server is running on port 5001");

    server.run().await
    // println!("Hello, world!");
}
