#[macro_use]
extern crate diesel;

mod api;
mod db;
mod ds18b20;
mod ir;
mod lg_ac;

use std::sync::{Arc, Mutex};

use db::DB;
use ir::IR;

fn check_loop(current_state: Arc<Mutex<lg_ac::State>>, current_temp: Arc<Mutex<f64>>) {
    std::thread::spawn(move || {
        // ===== Setup temperature sensor
        let res = ds18b20::DS18B20::new();
        let mut temp: Option<ds18b20::DS18B20> = None;
        match res {
            Err(e) => println!("Failed to initialize temperature sensor {}", e),
            Ok(t) => temp = Some(t),
        }

        // ====== Initialize IR
        println!("Initializing IR...");
        let res = IR::new(Arc::clone(&current_state));
        let ir = match res {
            Ok(_ir) => _ir,
            Err(err) => panic!("Couldn't start IR {}", err),
        };
        let lirc_tx_fd = ir.lirc_tx_fd;
        ir.startup_ir_read();

        // =====  Initialize DB
        println!("Initializing DB...");
        let mut db = DB::new();
        db.run_migrations();
        println!("Initialized DB.");

        loop {
            if let Some(t) = &temp {
                println!("Reading temp");
                let t = t.read_temp();
                match t {
                    Ok(t) => {
                        let celsius = t.to_celsius();
                        println!("\t{}", celsius);
                        db.new_temp(celsius);
                        match current_temp.lock() {
                            Ok(mut current_temp) => {
                                *current_temp = celsius;
                            }
                            Err(err) => println!("Failed to lock current temp {}", err),
                        }
                    }
                    Err(_) => {
                        println!("Failed to get temp");
                    }
                }
            }

            // State Updates
            let l = current_state.lock();
            match l {
                Ok(mut current_state) => {
                    if current_state.updated {
                        println!("check_loop: found new state");
                        db.update_state(*current_state);
                        IR::send_once(lirc_tx_fd, *current_state);
                    }

                    current_state.updated = false;
                }
                Err(e) => {
                    println!("Failed to lock current state to save to DB {}", e);
                }
            }
        }

        let ret = rust_lirc_client_sys::deinit();
        if ret == -1 {
            println!("Failed to deinit\n");
        }
    });
}

#[tokio::main]
async fn main() {
    let current_state = Arc::new(Mutex::new(lg_ac::State::default()));
    let current_temp = Arc::new(Mutex::new(0.0));

    // Fill current state from DB
    {
        // =====  Initialize DB
        println!("Initializing DB...");
        let mut db = DB::new();
        db.run_migrations();
        println!("Initialized DB.");
        let mut l = current_state
            .lock()
            .expect("Failed to lock current_state at start");
        *l = db.get_state();
    }

    check_loop(Arc::clone(&current_state), Arc::clone(&current_temp));

    let res = api::launch(Arc::clone(&current_state), Arc::clone(&current_temp));
    match res.await {
        Ok(res) => res,
        Err(e) => println!("Error waiting for actix-web {}", e),
    }
}
