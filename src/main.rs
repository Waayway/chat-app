use actix_web::{get, App, HttpServer, HttpResponse, Result};
use actix_files as fs;
use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct Index;

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let s = Index.render().unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
        .service(fs::Files::new("/", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}