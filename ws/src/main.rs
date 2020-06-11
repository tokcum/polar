use model::ObjectType;
use appl::pass;

use actix_web::{post, App, HttpServer, Responder, web};

#[post("/api/{resource}")]
async fn post(info: web::Path<String>) -> impl Responder {
    let ot = ObjectType::new();
    pass(&ot);
    format!("ObjectType: {:?}", ot)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(post))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
