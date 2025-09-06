use crate::worker_map;
use actix_web::{App, HttpResponse, HttpServer, Responder, dev::Server, get, post, web};
use std::time::{SystemTime, UNIX_EPOCH};

struct AppState {
    map: worker_map::MapManager,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("pong")
}

#[post("/sql")]
async fn sql(data: web::Data<AppState>) -> impl Responder {
    let now = SystemTime::now();
    let duration_since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let timestamp_secs = duration_since_epoch.as_secs() - 30;

    let test = data
        .map
        .map
        .lock()
        .expect("error acquiring mutex lock")
        .retain(|_key, value| *value > timestamp_secs);

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
