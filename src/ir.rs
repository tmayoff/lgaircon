use std::{thread::JoinHandle, collections::LinkedList, sync::{Arc, Mutex}};

use crate::lg_ac::State;

pub struct IR {
    pub send_fd: i32,
    running: bool,
    pub state_queue: LinkedList<State>
}

impl IR {
    pub fn new () -> Self {
        let ret = rust_lirc_client_sys::init("lgaircon", 1);
        if ret == -1 {
            println!("Initialization Failed\n");
        }

        let fd_ret = rust_lirc_client_sys::get_local_socket("/var/run/lirc/lircd-tx", false);
        if fd_ret.is_err() {
            println!("\n");
        }

        Self {
            send_fd: fd_ret.unwrap(),
            running: true,
            state_queue: LinkedList::new(),
        }
    }

    pub fn send_once (&mut self, state: State)  {
        println!("Sending IR...");
        
        let cmd = State::from_state(state);

        let r = rust_lirc_client_sys::send_one(self.send_fd, "LG_AC",  cmd.as_str());
        if r == -1 {
            println!("Failed to send");
        }

        println!("Sent IR.");
    }

    pub fn startup_ir_read(this: Arc<Mutex<Self>>) -> JoinHandle<()> {
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

                    let raw = ret_c.expect("String Failed");
                    println!("{}", raw);

                    // TODO send this somewhere
                    let ret = State::from_lirc_command(&raw);
                    match ret {
                        Err(r) => println!("Failed to decode lirc command: {}", r),
                        Ok(s) => {
                            let mut l = this.lock().unwrap();
                            l.state_queue.push_back(s);
                        }
                    }
                }
            }
        })
    }
}