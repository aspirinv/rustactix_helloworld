use actix_web::{get, App, HttpServer, HttpRequest, Responder};
use actix_files as fs;

mod domain;
mod controller;
use controller::customer_controller;

#[get("/{filename}")]
async fn files(req: HttpRequest) -> impl Responder {
    let path: std::path::PathBuf = ("src/front/dist/front/".to_owned() + req.match_info().query("filename")).parse().unwrap();
    fs::NamedFile::open(path)
}

#[get("/")]
async fn index() -> impl Responder {
    fs::NamedFile::open("src/front/dist/front/index.html")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(index)
        .service(files)
        .service(customer_controller::get_customer))
        .bind("127.0.0.1:58080")?
        .run()
        .await
}