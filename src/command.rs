pub trait Command {
    fn execute(&self, input: Option<String>) -> String;
}

pub struct PingCommand;

impl Command for PingCommand {
    fn execute(&self, _input: Option<String>) -> String {
        "+PONG\r\n".to_string()
    }
}

pub struct EchoCommand;

impl EchoCommand {
    pub fn format_response(&self, message: Option<String>) -> String {
        match message {
            Some(msg) => format!("+{}\r\n", msg),

            None => "+ECHO\r\n".to_string(),
        }
    }
}
