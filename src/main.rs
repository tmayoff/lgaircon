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

    // std::thread::spawn(|| {
    //     let fd = liblirc_client_sys::get_local_socket(None, false);
    //     if fd.is_err() {
    //         println!("Failed to get socket");
    //         return;
    //     }
        
    //     loop {
    //         println!("Sending IR...");
    //         let r = liblirc_client_sys::send_one(fd.unwrap(), String::from("LG_AC"),  String::from("AC_LOW_23"));
    //         if r == -1 {
    //             println!("Failed to send");
    //         }

    //         println!("Sent IR.");
    //         thread::sleep(time::Duration::from_secs(3)); 
    //     }
    // });
    

    loop {
        println!("Receiving IR....");
        let ret_c = liblirc_client_sys::nextcode();
        if ret_c.is_err() {
            println!("failed to get next code\n");
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