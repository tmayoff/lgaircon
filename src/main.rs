fn main () {
    let ret = liblirc_client_sys::init("lgaircon", 1);
    if ret == -1 {
        println!("Initialization Failed");
    }

    let _ = liblirc_client_sys::lirc_config::new();

    loop {
        let ret_c = liblirc_client_sys::nextcode();
        if ret_c.is_err() {
            println!("failed to get next code");
        }

        println!("{}", ret_c.unwrap());
    } 

    ret = liblirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit");
    }
}