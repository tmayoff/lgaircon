use core::panic;
use std::sync::{Mutex, Arc};

use rocket::serde::json::Json;
use crossbeam_channel::{Receiver, Sender};

use crate::lg_ac;

#[derive(Clone)]
struct StateManager {
    state_tx: Sender<lg_ac::State>,
    state_rx: Receiver<lg_ac::State>,
    current_temp_rx: Receiver<f64>,
    last_state: lg_ac::State,
    last_temp: f64,
}

#[get("/state")]
fn get_state(state: &rocket::State<Arc::<Mutex<StateManager>>>) -> Json<lg_ac::State> {
    let l = state.lock();
    if let Ok(mut s) = l {
        if let Ok(new_state) = s.state_rx.try_recv() {
            println!("New State found in StateManager::state_rx");

            s.last_state = new_state;
            return Json(s.last_state);
        }

        println!("No new State found in StateManager::state_rx");
        return Json(s.last_state);
    } else {
        println!("Failed to lock API StateManager");
        return Json(lg_ac::State::default())
    }
}

#[post("/state")]
fn set_state(state: &rocket::State<Arc::<Mutex<StateManager>>>) {
    let l = state.lock();
    if let Ok(_) = l {
        // s.state_tx 
    }
}

#[get("/current_temp")]
fn get_current_temp(state: &rocket::State<Arc::<Mutex<StateManager>>>) -> Json<f64> {
    let l = state.lock();
    if let Ok(mut s) = l {
        if let Ok(new_temp) = s.current_temp_rx.try_recv() {
            println!("New temp found in StateManager::current_temp_rx");
            s.last_temp = new_temp;
            return Json(new_temp);
        }

        println!("No new State found in StateManager::state_rx");
        return Json(s.last_temp);
    } else {
        println!("Failed to lock API StateManager");
        return Json(0.0);
    }
}

pub async fn launch(state_tx: Sender<lg_ac::State>, state_rx: Receiver<lg_ac::State>, current_temp_rx: Receiver<f64>) {
    let state_manager = StateManager {
        state_tx,
        state_rx,
        current_temp_rx,
        last_state: lg_ac::State::default(),
        last_temp: 0.0,
    };

    let arc = Arc::<Mutex<StateManager>>::new(Mutex::new(state_manager));

    let r = rocket::build()
    .manage(arc.clone())
    .mount("/", routes![get_state, set_state, get_current_temp])
    .launch().await;
    if let Err(_) = r {
        panic!("Rocket faild to launch")
    }
}