use std::{thread::{self, JoinHandle}, time};

use crate::lg_ac::State;

pub struct IR {
    pub send_fd: i32,
    pub send_thread: Option<JoinHandle<()>>,
    running: bool,
}

impl IR {
    pub fn new () -> Self {
        Self {
            send_fd: 0,
            send_thread: None,
            running: true
        }
    }

    pub fn send_once (&mut self, state: State)  {
        println!("Sending IR...");
        
        let cmd = State::from_state(state);

        let r = liblirc_client_sys::send_one(self.send_fd, "LG_AC",  cmd.as_str());
        if r == -1 {
            println!("Failed to send");
        }

        println!("Sent IR.");
        thread::sleep(time::Duration::from_secs(1));
    }

    pub fn startup_ir_read(&mut self) {
        let fd_ret = liblirc_client_sys::get_local_socket("/var/run/lirc/lircd-tx", true);
        if fd_ret.is_err() {
            println!("\n");
            return;
        }

        self.send_fd = fd_ret.unwrap();
        self.running = true;

        self.send_thread = Some(std::thread::spawn(move || {
            println!("Receiving IR....");
            let ret_c = liblirc_client_sys::nextcode();
            if ret_c.is_err() {
                return;
            }
            println!("Received IR.");
            
            let raw = ret_c.expect("String Failed");
            // TODO send this somewhere
            let _newState = State::from_lirc_command(&raw);

            println!("{}", raw);
        }));
    }

    pub fn join (&mut self) {
        if let Some(t) = self.send_thread.take() {
            t.join().unwrap();
        }
    }
}