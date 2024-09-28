use crate::connection::RedisConnectionHandler;
use std::net::TcpListener;
use std::sync::Arc;

mod command;
mod connection;
mod rdb;
mod response;

fn main() {
    let rdb_file_path = Some("redis_data.rdb".to_string()); 
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let handler = Arc::new(RedisConnectionHandler::new(rdb_file_path));

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
