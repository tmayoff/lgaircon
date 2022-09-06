use core::panic;
use std::sync::{Mutex, Arc};

use rocket::serde::json::Json;
use crossbeam_channel::{Receiver, Sender};

use crate::lg_ac;

#[derive(Clone)]
struct StateManager {
    state_tx: Sender<lg_ac::State>,
    state_rx: Receiver<lg_ac::State>,
    last_state: lg_ac::State,
}

#[get("/state")]
fn get_state(state: &rocket::State<Arc::<Mutex<StateManager>>>) -> Json<lg_ac::State> {
    let l = state.lock();
    if let Ok(mut s) = l {
        if let Ok(new_state) = s.state_rx.try_recv() {
            println!("New State found in StateManager::state_rx");

            s.last_state = new_state;
            return Json(new_state);
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

pub async fn launch(state_tx: Sender<lg_ac::State>, state_rx: Receiver<lg_ac::State>) {
    let state_manager = StateManager {
        state_tx,
        state_rx,
        last_state: lg_ac::State::default(),
    };

    let arc = Arc::<Mutex<StateManager>>::new(Mutex::new(state_manager));
    
    // let cloned = arc.clone();
    // std::thread::spawn(move || {
    //     loop {
    //         let l = cloned.lock();
    //         if let Ok(mut s) = l {
    //             let res = s.state_rx.try_recv();
    //             if let Ok(new_s) = res {
    //                 println!("Found new state");
    //                 s.last_state = new_s;
    //             }
    //             std::mem::drop(s);
    //         }

    //         std::thread::sleep(std::time::Duration::new(1, 0));
    //     }
    // });

    let r = rocket::build()
    .manage(arc.clone())
    .mount("/", routes![get_state, set_state])
    .launch().await;
    if let Err(_) = r {
        panic!("Rocket faild to launch")
    }
}