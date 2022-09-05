use core::panic;
use std::sync::{Mutex, Arc};

use rocket::serde::json::Json;
use spmc::Receiver;

use crate::lg_ac;

#[derive(Clone)]
struct StateManager {
    state_rx: Receiver<lg_ac::State>,
    last_state: lg_ac::State,
}

#[get("/")]
fn index(state: &rocket::State<Arc::<Mutex<StateManager>>>) -> Json<lg_ac::State> {
    let l = state.lock();
    match l {
        Err(_) => return Json(lg_ac::State::default()),
        Ok(mut s) => {
            let res = s.state_rx.try_recv();
            match res {
                Ok(new_s) => {
                    s.last_state = new_s;
                    return Json(new_s);
                }
                Err(_) => {
                    return Json(s.last_state);
                }
            }
        }
    }
}

pub async fn launch(state_rx: Receiver<lg_ac::State>) {
    let arc = Arc::<Mutex<StateManager>>::new(Mutex::new(StateManager{state_rx, last_state: lg_ac::State::default()}));
    let r = rocket::build()
    .manage(arc)
    .mount("/", routes![index])
    .launch().await;
    if let Err(_) = r {
        panic!("Rocket faild to launch")
    }
}