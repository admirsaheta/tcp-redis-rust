use crate::command::{EchoCommand, PingCommand};
use crate::response::RedisResponse;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;

pub trait ConnectionHandler {
    fn handle_client(&self, stream: TcpStream);
}

pub struct RedisConnectionHandler;

impl RedisConnectionHandler {
    pub fn new() -> Self {
        RedisConnectionHandler {}
    }

    pub fn handle_concurrent_clients(self: Arc<Self>, stream: TcpStream) {
        let handler = Arc::clone(&self);
        thread::spawn(move || {
            handler.handle_client(stream);
        });
    }
}

impl ConnectionHandler for RedisConnectionHandler {
    fn handle_client(&self, mut stream: TcpStream) {
        let mut buffer = [0; 512];

        loop {
            match stream.read(&mut buffer) {
                Ok(bytes_read) => {
                    if bytes_read == 0 {
                        break;
                    }

                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let mut lines = request.lines();
                    if let Some(command_line) = lines.next() {
                        let response = match command_line.trim() {
                            "PING" => {
                                let ping_command = PingCommand;
                                ping_command.format_response()
                            }
                            cmd if cmd.starts_with("ECHO") => {
                                let echo_command = EchoCommand;
                                let message = cmd.strip_prefix("ECHO ").map(|s| s.to_string());
                                echo_command.format_response(message)
                            }
                            _ => "-ERR unknown command\r\n".to_string(),
                        };
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }
                Err(_) => break,
            }
        }
    }
}
