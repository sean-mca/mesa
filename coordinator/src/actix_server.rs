use crate::worker_map;
use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get, post, web};
use std::time::{SystemTime, UNIX_EPOCH};

struct AppState {
    pub map: worker_map::MapManager,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/sql")]
async fn sql(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok()
}

pub fn create_actix_server(wmap: worker_map::MapManager) -> Server {
    let actix_future = {
        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(AppState { map: wmap.clone() }))
                .service(health)
                .service(sql)
        })
        .bind("0.0.0.0:3000")
        .expect("error launching actix server")
        .run()
    };

    actix_future
}
