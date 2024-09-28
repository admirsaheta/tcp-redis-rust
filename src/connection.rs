use crate::command::{EchoCommand, PingCommand};
use crate::response::RedisResponse;
use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub trait ConnectionHandler {
    fn handle_client(&self, stream: TcpStream);
}

pub struct RedisConnectionHandler {
    store: Arc<Mutex<HashMap<String, String>>>,
    expiry: Arc<Mutex<HashMap<String, Instant>>>,
}

impl RedisConnectionHandler {
    pub fn new() -> Self {
        let handler = RedisConnectionHandler {
            store: Arc::new(Mutex::new(HashMap::new())),
            expiry: Arc::new(Mutex::new(HashMap::new())),
        };
        handler.start_expiry_cleanup();
        handler
    }

    pub fn handle_concurrent_clients(self: Arc<Self>, stream: TcpStream) {
        let handler = Arc::clone(&self);
        thread::spawn(move || {
            handler.handle_client(stream);
        });
    }

    fn start_expiry_cleanup(&self) {
        let store = Arc::clone(&self.store);
        let expiry = Arc::clone(&self.expiry);

        thread::spawn(move || loop {
            thread::sleep(Duration::from_secs(1));

            let mut expiry_map = expiry.lock().unwrap();
            let mut store_map = store.lock().unwrap();

            let mut now = Instant::now();
            expiry_map.retain(|key, expiry_time| {
                if expiry_time <= &mut now {
                    store_map.remove(key);
                    false
                } else {
                    true
                }
            });
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
                        let mut parts = command_line.split_whitespace();
                        let command = parts.next().unwrap_or("");

                        let response = match command {
                            "PING" => {
                                let ping_command = PingCommand;
                                ping_command.format_response()
                            }
                            "ECHO" => {
                                let echo_command = EchoCommand;
                                let message = parts.next().map(|s| s.to_string());
                                echo_command.format_response(message)
                            }
                            "SET" => {
                                let key = parts.next();
                                let value = parts.next();
                                let expiry_time = if let (Some("EX"), Some(seconds)) =
                                    (parts.next(), parts.next())
                                {
                                    Some(seconds.parse::<u64>().unwrap())
                                } else {
                                    None
                                };

                                if let (Some(key), Some(value)) = (key, value) {
                                    let mut store = self.store.lock().unwrap();
                                    store.insert(key.to_string(), value.to_string());

                                    if let Some(expiry_seconds) = expiry_time {
                                        let mut expiry_map = self.expiry.lock().unwrap();
                                        expiry_map.insert(
                                            key.to_string(),
                                            Instant::now() + Duration::from_secs(expiry_seconds),
                                        );
                                    }

                                    "+OK\r\n".to_string()
                                } else {
                                    "-ERR wrong number of arguments for 'SET' command\r\n"
                                        .to_string()
                                }
                            }
                            "GET" => {
                                let key = parts.next();
                                if let Some(key) = key {
                                    let store = self.store.lock().unwrap();
                                    if let Some(value) = store.get(key) {
                                        format!("${}\r\n{}\r\n", value.len(), value)
                                    } else {
                                        "$-1\r\n".to_string()
                                    }
                                } else {
                                    "-ERR wrong number of arguments for 'GET' command\r\n"
                                        .to_string()
                                }
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
