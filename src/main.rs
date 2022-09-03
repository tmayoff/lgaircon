#[macro_use]
extern crate diesel;

mod lg_ac;
mod db;
mod ir;
mod ds18b20;

use std::sync::mpsc;

use ir::IR;
use db::DB;

fn main () {
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
    let ir_arc = IR::new(state_tx);
    let ir_thread = IR::startup_ir_read(ir_arc);
    println!("Initialized IR.");

    let temp = ds18b20::DS18B20::new().unwrap();

    while running {
        let ctrl = control_rx.try_recv();
        match ctrl {
            Ok(ctrl) => running = ctrl,
            Err(_) => ()
        }

        let t = temp.read_temp().unwrap();

        // IR Receiver Update
        let ir_update = state_rx.try_recv();
        match ir_update {
            Ok(update) => db.update_state(update),
            Err(err) => {
                match err {
                    mpsc::TryRecvError::Disconnected => println!("IR updater disconnected"),
                    mpsc::TryRecvError::Empty => (),
                }
            }
        }
    }

    let ret = rust_lirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit\n");
    }

    ir_thread.join().unwrap();
}
