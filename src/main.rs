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

    let (tx, rx) = mpsc::channel::<lg_ac:: State>();

    // Initialize DB
    println!("Initializing DB...");
    let mut db = DB::new();
    let _ = db.run_migrations();
    println!("Initialized DB.");

    // Initialize IR
    println!("Initializing IR...");
    let ir_arc = IR::new(tx);
    let ir_thread = IR::startup_ir_read(ir_arc);
    println!("Initialized IR.");

    let temp = ds18b20::DS18B20::new().unwrap();

    loop {
        let t = temp.read_temp().unwrap();
        println!("{}", t.to_celsius());

        let ir_update = rx.try_recv();
        match ir_update {
            Ok(update) => db.update_state(update),
            Err(_err) => println!("Err"),
        }
    }

    let ret = rust_lirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit\n");
    }

    ir_thread.join().unwrap();
}
