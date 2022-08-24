#[macro_use]
extern crate diesel;

mod lg_ac;
mod db;
mod ir;

use std::sync::Mutex;

use ir::IR;
use db::DB;

fn main () {

    // Initialize DB
    println!("Initializing DB...");
    let mut db = DB::new();
    println!("Initialized DB.");

    // Initialize IR
    println!("Initializing IR...");
    let ir_arc = std::sync::Arc::<Mutex<IR>>::new(Mutex::new(IR::new()));
    let ir_thread = IR::startup_ir_read(ir_arc.clone());
    println!("Initialized IR.");


    loop {
        let mut l = ir_arc.lock().unwrap();
        if l.state_queue.len() > 0 {
            let s_opt = l.state_queue.pop_back();
            match s_opt {
                None => todo!(),
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
