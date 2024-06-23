use crate::evaluator::{CmdResultActionType, WebEvaluatorResult};

pub type CommandFunction = fn(Vec<String>) -> String;

#[derive(Debug)]
pub struct CommandDefinition {
    pub name: String,
    pub description: String,
    pub function: CommandFunction,
    pub action: CmdResultActionType,
}

pub trait CommandRegistry {
    fn new() -> Self
    where
        Self: Sized;
    fn register_command(&mut self, command_definition: CommandDefinition);
    fn execute_command(&self, cmd_name: &str, args: Vec<String>) -> Option<WebEvaluatorResult>;
    fn get_command_description(&self, cmd_name: &str) -> Option<&str>;
    fn list_available_commands(&self) -> Vec<&str>;
    fn get_hostname() -> String;
    fn get_user_name() -> String;
}
