use std::net::{SocketAddr, TcpListener, TcpStream};

use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    let lis = TcpListener::bind("127.0.0.1:3000")?;

    let mut worker_id = 0;
    loop {
        match lis.accept() {
            Ok((mut socket, addr)) => {
                let mut _worker = Worker::new(worker_id, &mut socket, addr);
                _worker.handle();
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e)
            }
        }
        worker_id += 1;
    }
}
struct Worker<'a> {
    id: i32,
    stream: &'a mut TcpStream,
    addr: SocketAddr,
}

impl<'a> Worker<'a> {
    fn new(id: i32, stream: &mut TcpStream, addr: SocketAddr) -> Worker {
        Worker { id, stream, addr }
    }

    fn handle(&mut self) {
        let resp = format!("Worker {} accept connection from {}", self.id, self.addr);

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

                    let data = &buf[..n];
                    print!(
                        "[{}] Received data: {}",
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
