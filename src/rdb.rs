use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::time::SystemTime;

#[derive(Serialize, Deserialize)]
pub struct RedisData {
    pub store: HashMap<String, String>,
    pub expiry: HashMap<String, Option<SystemTime>>,
}

pub struct RdbPersistence {
    pub file_path: Option<String>,
}

impl RdbPersistence {
    pub fn new(file_path: Option<String>) -> Self {
        RdbPersistence { file_path }
    }

    pub fn load_from_rdb(&self) -> Option<RedisData> {
        if let Some(ref path) = self.file_path {
            let file = File::open(path);
            if let Ok(file) = file {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(data) => Some(data),
                    Err(_) => {
                        println!("Failed to parse RDB file: {}", path);
                        None
                    }
                }
            } else {
                println!("No existing RDB file found: {}", path);
                None
            }
        } else {
            None
        }
    }

    pub fn save_to_rdb(&self, data: &RedisData) {
        if let Some(ref file_path) = self.file_path {
            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(file_path);

            if let Ok(file) = file {
                let writer = BufWriter::new(file);
                if let Err(e) = serde_json::to_writer(writer, data) {
                    println!("Failed to save data to RDB: {}", e);
                } else {
                    println!("Data successfully saved to RDB file: {}", file_path);
                }
            } else {
                println!("Failed to open RDB file: {}", file_path);
            }
        }
    }
}
