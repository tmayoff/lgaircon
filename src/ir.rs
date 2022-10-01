use std::{sync::Arc, sync::Mutex, thread::JoinHandle};

use crate::lg_ac::State;

#[derive(Clone)]
pub struct IR {
    pub lirc_tx_fd: i32,
    current_state: Arc<Mutex<State>>,
}

impl IR {
    pub fn new(current_state: Arc<Mutex<State>>) -> Result<Self, String> {
        let res = rust_lirc_client_sys::get_local_socket("/var/run/lirc/lircd-tx", false);
        let fd = match res {
            Ok(_fd) => _fd,
            Err(_) => return Err(String::from("Failed to initialize IR")),
        };

        Ok(Self {
            lirc_tx_fd: fd,
            current_state,
        })
    }

    pub fn send_once(fd: i32, state: State) {
        println!("Sending IR...");

        let cmd = State::from_state(state);

        let r = rust_lirc_client_sys::send_one(fd, "LG_AC", cmd.as_str());
        if r == -1 {
            println!("Failed to send");
        }

        println!("Sent IR.");
    }

    pub fn startup_ir_read(self) -> JoinHandle<()> {
        std::thread::spawn(move || {
            let ret = rust_lirc_client_sys::init("lgaircon", 1);
            if ret == -1 {
                println!("Initialization Failed\n");
            }

            loop {
                println!("Receiving IR....");
                let ret_c = rust_lirc_client_sys::nextcode();
                if ret_c.is_err() {
                    println!("Error receiving {:?}", ret_c.err().take());
                } else {
                    println!("Received IR.");

                    if let Ok(raw) = ret_c {
                        println!("{}", raw);

                        let ret = State::from_lirc_command(&raw);
                        match ret {
                            Err(r) => println!("Failed to decode lirc command: {}", r),
                            Ok(s) => {
                                // Lock and update state
                                let l = self.current_state.lock();
                                match l {
                                    Ok(mut current_state) => {
                                        current_state.updated = true;
                                        *current_state = s;
                                    }
                                    Err(e) => {
                                        println!(
                                            "IR::ir_thread: Failed to lock current_state {}",
                                            e
                                        );
                                    }
                                }
                            }
                        }
                    }
                }
            }
        })
    }
}
