mod lg_ac;
mod ir;

use ir::IR;

fn main () {
    let mut ir_obj = IR::new();
    let ir_thread = ir_obj.startup_ir_read();

    let ret = rust_lirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit\n");
    }

    ir_thread.join().unwrap();
}
