use std::io;

use actix_web::{App, get, HttpResponse, HttpServer};
use actix_web::http::header::ContentType;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[utoipa::path(
    responses(
    (status = 200, description = "Shutting down"),
    (status = 500, description = "Failed to shutdown"),
    ),
)]
#[get("/shutdown")]
pub async fn shutdown() -> HttpResponse {
    match system_shutdown::shutdown() {
        Err(e) => { HttpResponse::InternalServerError()
            .content_type(ContentType::plaintext())
            .body(format!("{e:#?}"))}
        Ok(_) => HttpResponse::Ok().finish()
    }
}

#[utoipa::path(
    responses(
    (status = 200, description = "Rebooting"),
    (status = 500, description = "Failed to reboot"),
    ),
)]
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
    #[derive(OpenApi)]
    #[openapi(paths(reboot, shutdown))]
    struct ApiDoc;
    
    HttpServer::new(|| {
        App::new()
            .service(shutdown)
            .service(reboot)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
        .bind("0.0.0.0:9090")?
        .run()
        .await
}