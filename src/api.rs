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

#[get("/state")]
fn index(state: &rocket::State<Arc::<Mutex<StateManager>>>) -> Json<lg_ac::State> {
    let l = state.lock();
    if let Ok(s) = l {
        return Json(s.last_state);
    } else {
        return Json(lg_ac::State::default())
    }
}

pub async fn launch(state_rx: Receiver<lg_ac::State>) {
    let arc = Arc::<Mutex<StateManager>>::new(Mutex::new(StateManager{state_rx, last_state: lg_ac::State::default()}));
    
    let cloned = arc.clone();
    std::thread::spawn(move || {
        loop {
            let l = cloned.lock();
            if let Ok(mut s) = l {
                let res = s.state_rx.recv();
                if let Ok(new_s) = res {
                    println!("Found new state");
                    s.last_state = new_s;
                }
            }
        }
    });

    let r = rocket::build()
    .manage(arc.clone())
    .mount("/", routes![index])
    .launch().await;
    if let Err(_) = r {
        panic!("Rocket faild to launch")
    }
}