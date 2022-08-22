mod lg_ac;
mod ir;

use ir::IR;

fn main () {
    let ret = liblirc_client_sys::init("lgaircon", 1);
    if ret == -1 {
        println!("Initialization Failed\n");
        return;
    }

    let r_conf = liblirc_client_sys::readconfig(None);
    if r_conf.is_err() {
        println!("Failed to create config\n");
        return;
    }

    let mut ir_obj = IR::new();
    ir_obj.startup_ir_read();

    let ret = liblirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit\n");
    }

    ir_obj.join();
}