#![allow(unused_imports)]
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

trait Command {
    fn execute(&self) -> String;
}

struct PingCommand;

impl Command for PingCommand {
    fn execute(&self) -> String {
        "+PONG\r\n".to_string()
    }
}

trait RedisResponse {
    fn format_response(&self) -> String;
}

impl RedisResponse for PingCommand {
    fn format_response(&self) -> String {
        self.execute()
    }
}

trait ConnectionHandler {
    fn handle_client(&self, stream: TcpStream);
}

struct RedisConnectionHandler;

impl RedisConnectionHandler {
    fn new() -> Self {
        RedisConnectionHandler {}
    }
}

impl ConnectionHandler for RedisConnectionHandler {
    fn handle_client(&self, mut stream: TcpStream) {
        let command = PingCommand;
        let response = command.format_response();
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn main() {
    println!("Starting Redis server on 127.0.0.1:6379...");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    let handler = RedisConnectionHandler::new();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted new connection");
                handler.handle_client(stream);
            }
            Err(e) => {
                println!("Error accepting connection: {}", e);
            }
        }
    }
}
