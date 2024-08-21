use std::net::{SocketAddr, TcpStream};

struct Worker {
    id: i32,
    stream: &mut TcpStream,
}

impl Worker {
    fn new(id: i32, stream: &mut TcpStream, addr: SocketAddr) -> Worker {
        Worker {
            id: id,
            stream: stream,
        }
    }

    fn handle(self: &Worker) {
        let resp = format!(
            "Worker {} accept connection from {}",
            self.worker_id, self.addr
        );

        match stream.write(resp.as_bytes()) {
            Ok(_) => {}
            Err(e) => {
                println!("Failed to write to stream: {}", e);
                return;
            }
        }

        let mut buf = [0u8; 1024];
        loop {
            match stream.read(&mut buf) {
                Ok(n) => {
                    if n == 0 {
                        println!("[{}] Connection closed", worker_id);
                        return;
                    }

                    let data = &buf[..n];
                    print!(
                        "[{}] Received data: {}",
                        worker_id,
                        String::from_utf8_lossy(data)
                    );
                }
                Err(e) => {
                    println!("[{}] Failed to read from stream: {}", worker_id, e);
                    return;
                }
            }
        }
    }
}
