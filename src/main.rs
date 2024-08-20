use std::{
    io::Write,
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
    println!("Worker {} accept connection from {}", worker_id, addr);

    stream
        .write(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nHello")
        .unwrap();
}
