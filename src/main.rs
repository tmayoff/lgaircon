mod lg_ac;
mod ir;

use std::sync::Mutex;

use ir::IR;

fn main () {
    let ir_arc = std::sync::Arc::<Mutex<IR>>::new(Mutex::new(IR::new()));
    let ir_thread = IR::startup_ir_read(ir_arc.clone());

    let ret = rust_lirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit\n");
    }

    loop {
        let mut l = ir_arc.lock().unwrap();
        if l.state_queue.len() > 0 {
            let s_opt = l.state_queue.pop_back();
            match s_opt {
                None => todo!(),
                Some(s) => {
                    println!("New State");
                    todo!("Update state");
                }
            }
        }
    }

    ir_thread.join().unwrap();
}
