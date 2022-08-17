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

    loop {
        let ret_c = liblirc_client_sys::nextcode();
        if ret_c.is_err() {
            println!("failed to get next code\n");
            break;
        }

        let raw = ret_c.expect("String Failed");

        println!("{}", raw);
    } 

    let ret = liblirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit\n");
    }
}