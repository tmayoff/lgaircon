
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

use std::sync::mpsc;

use ir::IR;
use db::DB;

#[rocket::main]
async fn main () {
    let apires = api::launch();

    std::thread::spawn(|| {
        let mut running: bool = true;
        let (control_tx, control_rx) = mpsc::channel::<bool>();
        ctrlc::set_handler(move || {
            control_tx.send(false).expect("Failed to send stop signal");
        }).expect("Failed to set ctrl+c handler");

        let (state_tx, state_rx) = mpsc::channel::<lg_ac:: State>();

        // Initialize DB
        println!("Initializing DB...");
        let mut db = DB::new();
        let _ = db.run_migrations();
        println!("Initialized DB.");

        // Initialize IR
        println!("Initializing IR...");
        let res = IR::new(state_tx);
        match res {
            Err(e) => println!("{}", e),
            Ok(ir) => {
                IR::startup_ir_read(ir);
                println!("Initialized IR.");        
            }
        }

        let res = ds18b20::DS18B20::new();
        let mut temp: Option<ds18b20::DS18B20> = None;
        match res {
            Err(e) => println!("{}", e),
            Ok(t) => temp = Some(t),
        }


        while running {
            let ctrl = control_rx.try_recv();
            match ctrl {
                Ok(ctrl) => running = ctrl,
                Err(_) => ()
            }
            
            if let Some(t) = &temp {
                let t = t.read_temp();
                if let Ok(t) = t {
                    db.new_temp(t.to_celsius())
                }
            }

            // IR Receiver Update
            let ir_update = state_rx.try_recv();
            match ir_update {
                Ok(update) => db.update_state(update),
                Err(err) => {
                    match err {
                        mpsc::TryRecvError::Disconnected => {
                            running = false;
                            println!("IR updater disconnected");
                        },
                        mpsc::TryRecvError::Empty => (),
                    }
                }
            }
        }

        let ret = rust_lirc_client_sys::deinit();
        if ret == -1 {
            println!("Failed to deinit\n");
        }
    });

    apires.await;
}
