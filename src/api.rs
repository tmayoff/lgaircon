use core::panic;

use rocket::serde::json::Json;

use crate::lg_ac;

#[get("/state")]
fn index() -> Json<lg_ac::State> {
    let s = lg_ac::State::default();
    Json(s)
}

pub async fn launch() {
    let r = rocket::build().mount("/", routes![index]).launch().await;
    if let Err(_) = r {
        panic!("Rocket faild to launch")
    }
}