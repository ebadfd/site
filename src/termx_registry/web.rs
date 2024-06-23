use std::collections::HashMap;

use termx::{
    evaluator::{CmdResultActionType, WebEvaluatorResult},
    registries::{CommandDefinition, CommandFunction, CommandRegistry},
};

#[derive(Debug)]
pub struct WebCommandRegistry {
    pub commands: HashMap<String, CommandDefinition>,
}

impl CommandRegistry for WebCommandRegistry {
    fn new() -> Self {
        let mut registry = WebCommandRegistry {
            commands: HashMap::new(),
        };

        let echo_cmd: CommandFunction = |args| args.join(", ");
        let test_cmd: CommandFunction = |_args| "Listing files...".to_string();
        let id_cmd: CommandFunction = |_args| {
            format!(
                "uid=1000({}) gid=1000({}) groups=1000({}),998(wheel)",
                WebCommandRegistry::get_user_name(),
                WebCommandRegistry::get_user_name(),
                WebCommandRegistry::get_user_name()
            )
            .to_string()
        };
        let hostname_cmd: CommandFunction = |_args| WebCommandRegistry::get_hostname();

        registry.register_command(CommandDefinition {
            name: "echo".to_string(),
            description: "echo - display a line of text".to_string(),
            function: echo_cmd,
            action: CmdResultActionType::Display,
        });

        registry.register_command(CommandDefinition {
            name: "id".to_string(),
            description: "Print user and group information for each specified USER".to_string(),
            function: id_cmd,
            action: CmdResultActionType::Display,
        });

        registry.register_command(CommandDefinition {
            name: "hostname".to_string(),
            description: "Show or set the system's host name.".to_string(),
            function: hostname_cmd,
            action: CmdResultActionType::Display,
        });

        registry.register_command(CommandDefinition {
            name: "ls".to_string(),
            description: "Lists files".to_string(),
            function: test_cmd,
            action: CmdResultActionType::Display,
        });

        registry.register_command(CommandDefinition {
            name: "help".to_string(),
            description: "Displays help information".to_string(),
            function: test_cmd,
            action: CmdResultActionType::Display,
        });

        registry
    }

    fn register_command(&mut self, command_definition: CommandDefinition) {
        self.commands
            .insert(command_definition.name.clone(), command_definition);
    }

    fn execute_command(&self, cmd_name: &str, args: Vec<String>) -> Option<WebEvaluatorResult> {
        if let Some(command) = self.commands.get(cmd_name) {
            let result = (command.function)(args);
            match command.action {
                CmdResultActionType::Display => Some(WebEvaluatorResult::Display(result)),
                CmdResultActionType::Navigate => Some(WebEvaluatorResult::Navigate(result)),
            }
        } else {
            None
        }
    }

    fn get_command_description(&self, cmd_name: &str) -> Option<&str> {
        if let Some(command) = self.commands.get(cmd_name) {
            Some(&command.description)
        } else {
            None
        }
    }

    fn list_available_commands(&self) -> Vec<&str> {
        self.commands.keys().map(|k| k.as_str()).collect()
    }

    fn get_hostname() -> String {
        return "blog".to_string();
    }

    fn get_user_name() -> String {
        return "z9fr".to_string();
    }
}
