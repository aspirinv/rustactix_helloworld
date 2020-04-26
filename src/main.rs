use actix_web::{Result, get, web, App, HttpServer};
use actix_web::web::{ Json };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Customer {
    name: String,
    id: u32
}

#[get("customer/{id}/{name}")]
async fn index(info: web::Path<(u32, String)>) -> Result<Json<Customer>> {
    Ok(web::Json(Customer{
        name: info.1.to_string(),
        id: info.0
    }))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("127.0.0.1:58080")?
        .run()
        .await
}