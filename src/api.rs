use actix_web::{get, Responder, HttpServer, HttpResponse, App, web};
use std::sync::{Arc, Mutex};

use crate::lg_ac;

#[derive(Clone)]
struct AppState {
    current_state: Arc<Mutex<lg_ac::State>>,
    current_temp: Arc<Mutex<f64>>,
}

#[get("/state")]
async fn get_state(data: web::Data<AppState>) -> impl Responder {
    let l = data.current_state.lock();
    match l {
        Ok(current_state) => {
            return HttpResponse::Ok().json(*current_state);
        },
        Err(_) => {
            return HttpResponse::InternalServerError().body("Error");
        }
    }
}

#[get("/current_temp")]
async fn get_current_temp(data: web::Data<AppState>) -> impl Responder {
    let l = data.current_temp.lock();
    match l {
        Ok(current_temp) => {
            return HttpResponse::Ok().json(*current_temp);
        },
        Err(_) => {
            return HttpResponse::InternalServerError().body("Error");
        }
    }
}

pub async fn launch(current_state: Arc<Mutex<lg_ac::State>>, current_temp: Arc<Mutex<f64>>) -> std::io::Result<()> {
    let app_state = AppState {
        current_state,
        current_temp
    };

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(app_state.clone()))
        .service(get_state)
        .service(get_current_temp)
    }).bind(("0.0.0.0", 8000))?
    .run()
    .await
}