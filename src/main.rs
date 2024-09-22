use std::net::TcpListener;
use std::sync::Arc;
use crate::connection::RedisConnectionHandler;

mod command;
mod connection;
mod response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let handler = Arc::new(RedisConnectionHandler::new());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let handler_clone = Arc::clone(&handler);
                handler_clone.handle_concurrent_clients(stream);
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}