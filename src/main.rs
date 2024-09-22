#![allow(unused_imports)]
use crate::connection::{ConnectionHandler, RedisConnectionHandler};
use std::net::TcpListener;

mod command;
mod connection;
mod response;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let handler = RedisConnectionHandler::new();

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            handler.handle_client(stream);
        }
    }
}
