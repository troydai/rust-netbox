use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};

// Worker handls a single connection
pub struct Worker<'a> {
    id: i32,
    stream: &'a mut TcpStream,
    addr: SocketAddr,
}

impl<'a> Worker<'a> {
    // Create a new Worker instance with an id, TcpStream, and SocketAddr
    pub fn new(id: i32, stream: &mut TcpStream, addr: SocketAddr) -> Worker {
        Worker { id, stream, addr }
    }

    // handle instructs the worker to start handling the given tcp stream
    pub fn handle(&mut self) {
        let resp = format!("Worker {} accept connection from {}\n", self.id, self.addr);

        match self.stream.write(resp.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to write to stream: {}", e);
                return;
            }
        }

        let mut buf = [0u8; 1024];
        loop {
            match self.stream.read(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        println!("[{}] Connection closed", self.id);
                        return;
                    }

                    let data = trim_return(&buf[..n]);
                    print!(
                        "[{}] Received data: \"{}\"\n",
                        self.id,
                        String::from_utf8_lossy(data)
                    );
                }
                Err(e) => {
                    println!("[{}] Failed to read from stream: {}", self.id, e);
                    return;
                }
            }
        }
    }
}

impl Drop for Worker<'_> {
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