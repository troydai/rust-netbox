use std::io::Read;
use std::net::TcpStream;
use std::sync::{RwLock, Arc};
use std::thread::spawn;

// Worker handls a single connection
pub struct Worker {
    id: i32,
    state_lock: Arc::<RwLock<bool>>,    // ready vs busy
}

impl<'a> Worker {
    // Create a new Worker instance with an id, TcpStream, and SocketAddr
    pub fn new(id: i32) -> Worker {
        Worker {
            id,
            state_lock: Arc::new(RwLock::new(true)),
        }
    }

    pub fn is_ready(&self) -> bool {
        match self.state_lock.try_read() {
            Ok(state) => *state,
            Err(_) => false,
        }
    }

    // handle instructs the worker to start handling the given tcp stream
    pub fn handle(&mut self, mut socket: TcpStream) {
        let state = self.state_lock.clone();
        match state.write() {
            Ok(mut state) => { *state = false; }
            Err(_) => { print!("[{}] Failed to set worker state to busy\n", self.id); }
        }

        let id = self.id;
        spawn(move || {
            let mut buf = [0u8; 1024];
            loop {
                match socket.read(&mut buf) {
                    Ok(n) => {
                        if n == 0 {
                            println!("[{}] Connection closed", id);
                            break;
                        }
    
                        let data = trim_return(&buf[..n]);
                        print!(
                            "[{}] Received data: \"{}\"\n",
                            id,
                            String::from_utf8_lossy(data)
                        );
                    }
                    Err(e) => {
                        println!("[{}] Failed to read from stream: {}", id, e);
                        break;
                    }
                }
            }
            match state.write() {
                Ok(mut state) => {
                    *state = true;
                }
                Err(_) => {
                    print!("[{}] Failed to set worker state to ready\n", id);
                }
            }
        });

    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        println!("[{}] Worker dropped", self.id);
    }
}

fn trim_return(data: &[u8]) -> &[u8] {
    if data.ends_with(&[b'\r', b'\n']) {
        return &data[..data.len() - 2]
    } else if data.ends_with(&[b'\n']) {
        return &data[..data.len() - 1]
    }

    data
}