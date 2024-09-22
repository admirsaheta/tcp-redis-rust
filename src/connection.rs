use crate::command::PingCommand;
use crate::response::RedisResponse;
use std::io::{Read, Write};
use std::net::TcpStream;

pub trait ConnectionHandler {
    fn handle_client(&self, stream: TcpStream);
}

pub struct RedisConnectionHandler;

impl RedisConnectionHandler {
    pub fn new() -> Self {
        RedisConnectionHandler {}
    }
}

impl ConnectionHandler for RedisConnectionHandler {
    fn handle_client(&self, mut stream: TcpStream) {
        let command = PingCommand;
        let mut buffer = [0; 512];

        loop {
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }

                    let response = command.format_response();
                    stream.write_all(response.as_bytes()).unwrap();
                }
                Err(_) => {
                    break;
                }
            }
        }
    }
}
