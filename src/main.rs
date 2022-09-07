
#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[macro_use]
extern crate diesel;

mod lg_ac;
mod db;
mod ir;
mod ds18b20;
mod api;

use crossbeam_channel::TryRecvError;
use ir::IR;
use db::DB;

struct Control {
    running: bool,
}

#[rocket::main]
async fn main () {
    let c = Control{
        running: true,
    };

    // Initial Messages
    let (state_tx, state_rx) = crossbeam_channel::unbounded::<lg_ac::State>();
    let (control_tx, control_rx) = crossbeam_channel::unbounded::<Control>();
    if let Err(err) = control_tx.send(c) {
        panic!("failed to send initial control signal: {}", err);
    }

    let (current_temp_tx, current_temp_rx) = crossbeam_channel::unbounded::<f64>();

    // =====  Initialize DB
    println!("Initializing DB...");
    let mut db = DB::new();
    db.run_migrations();
    println!("Initialized DB.");

    // Get state from DB
    let starting_state = db.get_state();
    let res = state_tx.send(starting_state);
    if let Err(r) = res {
        println!("Failed to send starting state {}", r);
    }

    // Setup main thread
    let (main_state_tx, main_state_rx) = (state_tx.clone(), state_rx.clone());
    let (main_control_tx, main_control_rx) = (control_tx.clone(), control_rx.clone());
    std::thread::spawn(move || {
        let ctrlc_tx = main_control_tx.clone();
        ctrlc::set_handler(move || {
            ctrlc_tx.send(Control {running: false}).expect("Failed to send control+c signal");
        }).expect("Failed to set ctrl+c handler");

        // ====== Initialize IR
        println!("Initializing IR...");
        let res = IR::new(main_state_tx.clone());
        match res {
            Err(e) => println!("{}", e),
            Ok(_ir) => {
                IR::startup_ir_read(_ir);
                println!("Initialized IR.");
            }
        }

        // ===== Setup temperature sensor
        let res = ds18b20::DS18B20::new();
        let mut temp: Option<ds18b20::DS18B20> = None;
        match res {
            Err(e) => println!("Failed to initialize temperature sensor {}", e),
            Ok(t) => temp = Some(t),
        }

        loop {
            let ctrl = main_control_rx.try_recv();
            if let Ok(c) = ctrl {
                if !c.running {
                    break;
                }
            }

            if let Some(t) = &temp {
                println!("Reading temp");
                let t = t.read_temp();
                match t {
                    Ok(t) => {
                        let celsius = t.to_celsius();
                        db.new_temp(celsius);
                        if let Err(e) = current_temp_tx.send(celsius) {
                            println!("Failed to send state update: {}", e);
                        }
                    }
                    Err(_) => {
                        println!("Failed to get temp");
                    }
                }
            }

            // State Updates
            let res = main_state_rx.try_recv();
            match res {
                Ok(update) => {
                    db.update_state(update);
                },
                Err(err) => {
                    match err {
                        TryRecvError::Disconnected => {
                            println!("IR updater disconnected");
                        },
                        TryRecvError::Empty => (),
                    }
                }
            }
        }

        let ret = rust_lirc_client_sys::deinit();
        if ret == -1 {
            println!("Failed to deinit\n");
        }
    });

    let res = api::launch(state_tx.clone(), state_rx.clone(), current_temp_rx.clone());
    res.await;
}
