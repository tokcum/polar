use model::ObjectType;
use appl::{read, write};

use actix_files::{Files};
use actix_web::{get, post};
use actix_web::{App, HttpServer, Responder, web};

#[get("/api/{resource}")]
async fn get(info: web::Path<String>) -> impl Responder {
    let ot = read();
    format!("ObjectType: {:?}", ot)
}

#[post("/api/{resource}")]
async fn post(info: web::Path<String>) -> impl Responder {
    let ot = ObjectType::new();
    write(&ot);
    format!("ObjectType: {:?}", ot)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(post)
        .service(get)
        .service(Files::new("/", "/web").index_file("index.html")))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
