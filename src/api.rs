use std::io;
use actix_web::{App, get, HttpResponse, HttpServer};
use actix_web::http::header::ContentType;

#[get("/shutdown")]
pub async fn shutdown() -> HttpResponse {
    // TODO find the last 50 tweets and return them
    
    match system_shutdown::shutdown() {
        Err(e) => { HttpResponse::InternalServerError()
            .content_type(ContentType::plaintext())
            .body(format!("{e:#?}"))}
        Ok(_) => HttpResponse::Ok().finish()
    }
}

#[get("/reboot")]
pub async fn reboot() -> HttpResponse {
    match system_shutdown::reboot() {
        Err(e) => { HttpResponse::InternalServerError()
            .content_type(ContentType::plaintext())
            .body(format!("{e:#?}"))}
        Ok(_) => HttpResponse::Ok().finish()
    }
}


pub(crate) async fn api() -> io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(shutdown)
            .service(reboot)
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}