use std::fmt::{Display, Formatter};

use crate::evaluator::{CmdResultActionType, WebEvaluatorResult};

#[derive(Debug, Clone)]
pub struct FS {
    pub name: String,
    pub created_at: String,
    pub owner: String,
    pub group: String,
    pub summary: String,
    pub title: String,
}

impl Display for FS {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "-rw-r--r-- {:10} {:20} {:10} {:10}",
            self.owner,
            self.group,
            self.created_at.chars().take(10).collect::<String>(),
            self.name
        )
    }
}

pub type CommandFunction = fn(Vec<String>, fs: Vec<FS>) -> CommandFunctionResult;

pub struct CommandFunctionResult {
    pub result: String,
    pub is_success: bool,
}

pub struct CommandDefinition {
    pub name: String,
    pub description: String,
    pub function: CommandFunction,
    pub action: CmdResultActionType,
}

pub trait CommandRegistry {
    type State;

    fn new(sate: Self::State) -> Self
    where
        Self: Sized;
    fn register_command(&mut self, command_definition: CommandDefinition);
    fn execute_command(&self, cmd_name: &str, args: Vec<String>) -> Option<WebEvaluatorResult>;
    fn get_command_description(&self, cmd_name: &str) -> Option<&str>;
    fn list_available_commands(&self) -> Vec<&str>;
    fn get_hostname() -> String;
    fn get_user_name() -> String;
    fn help(&self) -> String;
}
