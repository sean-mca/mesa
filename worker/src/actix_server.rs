use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get, post, web};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

pub fn create_actix_server() -> Server {
    let actix_future = {
        HttpServer::new(move || App::new().service(health))
            .bind("0.0.0.0:4000")
            .expect("error launching actix server")
            .run()
    };

    actix_future
}
