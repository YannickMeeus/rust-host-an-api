/*
    There's an abundance (https://lib.rs/web-programming/http-server) of 
    web frameworks, so it seems. I've done my thorough research and found
    (on Reddit, first hit...) that actix-web is 🏎 fast 🏎 and 🔥 simple 🔥.

    And Reddit is always right.
*/

use actix_web::{App, HttpServer, Responder, HttpResponse, get};
use serde::Serialize;


#[derive(Serialize)]
struct PingResponse {
    data: &'static str,
}

#[get("/_plain_ping")]
async fn plain_ping() -> impl Responder {
    HttpResponse::Ok().body("simple_pong")
}
#[get("/_json_ping")]
async fn json_ping() -> impl Responder {
    HttpResponse::Ok().json(PingResponse { data: "json_pong" })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(plain_ping) //Registers a handler
            .service(json_ping)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await

}
