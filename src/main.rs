use actix_web::{App, HttpResponse, HttpServer, get};

#[get("/ping")]
async fn ping() -> HttpResponse {
	HttpResponse::Ok().finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ping)
    })
    .bind(("0.0.0.0", 3030))?
    .run()
    .await
}