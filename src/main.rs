use actix_web::{Result, get, web, App, HttpServer};
use actix_web::web::{ Json };

mod domain;
use domain::customer::{ Customer };

#[get("customer/{id}/{name}")]
async fn get_customer(info: web::Path<(u32, String)>) -> Result<Json<Customer>> {
    Ok(web::Json(Customer{
        name: info.1.to_string(),
        id: info.0
    }))
}

#[get("/")]
async fn index() -> &'static str {
    "Welcome!!!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
            .service(index)
            .service(get_customer))
        .bind("127.0.0.1:58080")?
        .run()
        .await
}