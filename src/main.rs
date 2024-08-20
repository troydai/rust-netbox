use std::{
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
};

fn main() -> std::io::Result<()> {
    let lis = TcpListener::bind("127.0.0.1:3000")?;

    let mut worker_id = 0;
    loop {
        match lis.accept() {
            Ok((mut socket, addr)) => handle(worker_id, &mut socket, addr),
            Err(e) => {
                println!("Failed to accept connection: {}", e)
            }
        }
        worker_id += 1;
    }
}

fn handle(worker_id: i32, stream: &mut TcpStream, addr: SocketAddr) {
    let resp = format!("Worker {} accept connection from {}", worker_id, addr);

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
                print!("[{}] Received data: {}", worker_id, String::from_utf8_lossy(data));
            }
            Err(e) => {
                println!("[{}] Failed to read from stream: {}", worker_id, e);
                return;
            }
        }
    }
}
