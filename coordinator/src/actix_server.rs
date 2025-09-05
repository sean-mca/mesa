use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get, post, web};

use crate::worker_map;

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

struct AppState {
    map: worker_map::MapManager,
}

pub fn create_actix_server(wmap: worker_map::MapManager) -> Server {
    let actix_future = {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState { map: wmap.clone() }))
                .service(health)
        })
        .bind("0.0.0.0:3000")
        .expect("error launching actix server")
        .run()
    };

    actix_future
}
