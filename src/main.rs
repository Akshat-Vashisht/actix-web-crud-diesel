use actix_web::{web, App, HttpServer, Responder};

mod routes;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/app")
                .route("/index.html", web::get().to(routes::index)),
        )
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
