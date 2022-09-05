use core::panic;

use rocket::serde::json::Json;
use spmc::Receiver;

use crate::lg_ac;

#[get("/state")]
fn index(state_rx: &rocket::State<Receiver<lg_ac::State>>) -> Json<lg_ac::State> {
    let r = state_rx.recv();
    match r {
        Err(e) => {
            println!("Failed to get new state {}", e);
            return Json(lg_ac::State::default());
        }
        Ok(s) => {
            return Json(s);
        }
    }
}

pub async fn launch(state_rx: Receiver<lg_ac::State>) {
    let r = rocket::build()
    .manage(state_rx)
    .mount("/", routes![index])
    .launch().await;
    if let Err(_) = r {
        panic!("Rocket faild to launch")
    }
}