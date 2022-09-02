#[macro_use]
extern crate diesel;

mod lg_ac;
mod db;
mod ir;
mod ds18b20;

use std::sync::Mutex;

use ir::IR;
use db::DB;

fn main () {

    // Initialize DB
    println!("Initializing DB...");
    let mut db = DB::new();
    let _ = db.run_migrations();
    println!("Initialized DB.");

    // Initialize IR
    println!("Initializing IR...");
    let ir_arc = std::sync::Arc::<Mutex<IR>>::new(Mutex::new(IR::new()));
    let ir_thread = IR::startup_ir_read(ir_arc.clone());
    println!("Initialized IR.");

    let temp = ds18b20::DS18B20::new().unwrap();

    loop {
        let t = temp.read_temp().unwrap();
        println!("{}", t.as_u32());

        let mut l = ir_arc.lock().unwrap();
        if l.state_queue.len() > 0 {
            let s_opt = l.state_queue.pop_back();
            match s_opt {
                None => {
                    println!("Empty State");
                    break;
                }
                Some(s) => {
                    db.update_state(s);
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
