use std::env;
use std::net::TcpListener;

mod actors;
mod utils;

use actors::Worker;

fn main() -> std::io::Result<()> {
    let args = env::args().collect();
    let port = utils::get_port(&args);

    let addr = format!("127.0.0.1:{}", port);
    println!("Server started at {}", addr);

    let total_worker = 4;
    let mut worker_queue: Vec<Worker> = Vec::new();
    for i in 0..total_worker {
        let worker = actors::Worker::new(i);
        worker_queue.push(worker);
    }

    let lis = TcpListener::bind(addr)?;
    loop {
        match lis.accept() {
            Ok((socket, addr)) => {
                match find_ready_worker(&worker_queue) {
                    Some(idx) => {
                        println!("Connection received from: {}", addr);
                        worker_queue[idx].handle(socket);
                    }
                    None => {
                        println!("No worker available, closing connection");
                    }
                }
            }
            Err(e) => {
                println!("Failed to accept connection: {}", e)
            }
        }
    }
}

fn find_ready_worker(workers: &Vec<Worker>) -> Option<usize> {
    for i in 0..workers.len() {
        if workers[i].is_ready() {
            return Some(i)
        }
    }
    None
}