use crate::command::Command;

pub trait RedisResponse {
    fn format_response(&self) -> String;
}

impl RedisResponse for super::command::PingCommand {
    fn format_response(&self) -> String {
        self.execute()
    }
}