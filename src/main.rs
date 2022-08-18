use std::{thread, time};

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

    std::thread::spawn(|| {
        let fd = liblirc_client_sys::get_local_socket("/var/run/lirc/lircd-tx", false);
        if fd.is_err() {
            println!("\n");
            return;
        }

        loop {
            println!("Sending IR...");
            let r = liblirc_client_sys::send_one(fd.unwrap(), "LG_AC",  "AC_ON");
            if r == -1 {
                println!("Failed to send");
            }

            println!("Sent IR.");
            thread::sleep(time::Duration::from_secs(1));
        }
    });

    loop {
        println!("Receiving IR....");
        let ret_c = liblirc_client_sys::nextcode();
        if ret_c.is_err() {
            // println!("failed to get next code\n");
            break;
        }
        println!("Received IR.");

        let raw = ret_c.expect("String Failed");

        println!("{}", raw);
    } 

    let ret = liblirc_client_sys::deinit();
    if ret == -1 {
        println!("Failed to deinit\n");
    }
}