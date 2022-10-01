use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::{Arc, Mutex};

use crate::lg_ac;

#[derive(Clone)]
struct AppState {
    current_state: Arc<Mutex<lg_ac::State>>,
    current_temp: Arc<Mutex<f64>>,
}

#[get("/state")]
async fn get_state(data: web::Data<AppState>) -> impl Responder {
    println!("get_state");
    let l = data.current_state.lock();
    match l {
        Ok(current_state) => {
            println!("\tSent state");
            HttpResponse::Ok().json(*current_state)
        }
        Err(_) => {
            println!("\tailed to lock");
            HttpResponse::InternalServerError().body("Error")
        }
    }
}

#[post("/state")]
async fn set_state(body: web::Json<lg_ac::State>, data: web::Data<AppState>) -> impl Responder {
    println!("set_state:: {:?}", body);
    let l = data.current_state.lock();
    match l {
        Ok(mut current_state) => {
            println!("\tUpdated state");
            *current_state = body.0;
            current_state.updated = true;
            HttpResponse::Ok()
        }
        Err(_) => {
            println!("\tFailed to lock");
            HttpResponse::InternalServerError()
        }
    }
}

#[get("/current_temp")]
async fn get_current_temp(data: web::Data<AppState>) -> impl Responder {
    let l = data.current_temp.lock();
    match l {
        Ok(current_temp) => HttpResponse::Ok().json(*current_temp),
        Err(_) => HttpResponse::InternalServerError().body("Error"),
    }
}

pub async fn launch(
    current_state: Arc<Mutex<lg_ac::State>>,
    current_temp: Arc<Mutex<f64>>,
) -> std::io::Result<()> {
    let app_state = AppState {
        current_state,
        current_temp,
    };

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone()))
            .service(get_state)
            .service(set_state)
            .service(get_current_temp)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
