use super::coms::HttpApi;
use crate::GAME_NAME;
use actix_web::{App, HttpResponse, HttpServer, Responder, get, post, web};
use tokio::sync::Mutex;

#[actix_web::main]
pub async fn start_api(api_channel: HttpApi) -> std::io::Result<()> {
    let api_channel = web::Data::new(Mutex::new(api_channel));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(api_channel.clone())
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 5000))?
    .bind(("localhost", 5000))?;

    #[cfg(not(all(
        target_os = "windows",
        target_os = "ios",
        target_os = "android",
        target_os = "none"
    )))]
    let server = server.bind_uds(format!("/tmp/{GAME_NAME}.sock"))?;

    server.run().await
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
