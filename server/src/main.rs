use actix_web::{get, web, App, HttpServer, Responder};

#[get("/")]
async fn get() -> impl Responder {
    "server!"
}

#[get("/{name}")]
async fn greeting(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get).service(greeting))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
