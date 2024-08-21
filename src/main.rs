use std::env;
use std::net::TcpListener;

mod actors;
mod utils;

fn main() -> std::io::Result<()> {
    let args = env::args().collect();
    let port = utils::get_port(&args);

    let lis = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    let mut worker_id = 0;
    loop {
        match lis.accept() {
            Ok((mut socket, addr)) => {
                let mut _worker = actors::Worker::new(worker_id, &mut socket, addr);
                _worker.handle();
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e)
            }
        }
        worker_id += 1;
    }
}

