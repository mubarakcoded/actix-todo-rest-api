mod config; 
mod models; 

use crate::models::Status;
use dotenv::dotenv;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status{ status:"OK".to_string() })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = crate::config::Config::from_env().unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))? 
    .run()
    .await
}
 