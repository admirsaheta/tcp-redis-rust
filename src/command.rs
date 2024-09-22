pub trait Command {
    fn execute(&self) -> String;
}

pub struct PingCommand;

impl Command for PingCommand {
    fn execute(&self) -> String {
        "+PONG\r\n".to_string()
    }
}