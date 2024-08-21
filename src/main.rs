use std::net::TcpListener;

mod actors;

fn main() -> std::io::Result<()> {
    let lis = TcpListener::bind("127.0.0.1:3000")?;

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
