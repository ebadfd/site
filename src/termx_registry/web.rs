use std::{collections::HashMap, sync::Arc};

use termx::{
    evaluator::{CmdResultActionType, WebEvaluatorResult},
    registries::{CommandDefinition, CommandFunction, CommandFunctionResult, CommandRegistry, FS},
};

use crate::app::State;

pub struct WebCommandRegistry {
    pub commands: HashMap<String, CommandDefinition>,
    pub fs: Vec<FS>,
}

impl CommandRegistry for WebCommandRegistry {
    type State = Arc<State>;

    fn new(state: Arc<State>) -> Self {
        let mut registry = WebCommandRegistry {
            commands: HashMap::new(),
            fs: state.fs.clone(),
        };

        let echo_cmd: CommandFunction = |args, _| CommandFunctionResult {
            result: args.join(", "),
            is_success: true,
        };
        let id_cmd: CommandFunction = |_args, _| {
            let result = format!(
                "uid=1000({}) gid=1000({}) groups=1000({}),998(wheel)",
                WebCommandRegistry::get_user_name(),
                WebCommandRegistry::get_user_name(),
                WebCommandRegistry::get_user_name()
            )
            .to_string();

            CommandFunctionResult {
                result,
                is_success: true,
            }
        };
        let hostname_cmd: CommandFunction = |_args, _| CommandFunctionResult {
            result: WebCommandRegistry::get_hostname(),
            is_success: true,
        };
        let ls_cmd: CommandFunction = |args, fs| {
            let mut result = String::new();

            let all_files = args.iter().any(|arg| arg == "a");
            let detailed_output = args.iter().any(|arg| arg == "l");

            for file in fs {
                if all_files || !file.name.starts_with('.') {
                    if detailed_output {
                        result.push_str(&format!("{}\n", file));
                    } else {
                        result.push_str(&format!("{}\n", file.name));
                    }
                }
            }

            CommandFunctionResult {
                result,
                is_success: true,
            }
        };
        let cat_cmd: CommandFunction = |args, fs| {
            let mut result = String::new();

            if args.len() < 1 {
                result.push_str("Usage: cat <filename>\n");

                return CommandFunctionResult {
                    result,
                    is_success: false,
                };
            }

            let filename = &args.join("");

            for file in fs {
                if file.name == *filename {
                    result.push_str(&format!("# {}\n", file.title));
                    result.push_str(&format!("{}\n", file.summary));
                    return CommandFunctionResult {
                        result,
                        is_success: true,
                    };
                }
            }

            result.push_str("File not found\n");
            return CommandFunctionResult {
                result,
                is_success: false,
            };
        };

        let open_cmd: CommandFunction = |args, fs| {
            let mut result = String::new();

            if args.len() < 1 {
                result.push_str("Usage: open <filename>\n");

                return CommandFunctionResult {
                    result,
                    is_success: false,
                };
            }

            let filename = &args.join("");

            for file in fs {
                if file.name == *filename {
                    result.push_str(&file.name);
                    return CommandFunctionResult {
                        result,
                        is_success: true,
                    };
                }
            }

            result.push_str("File not found\n");
            return CommandFunctionResult {
                result,
                is_success: false,
            };
        };

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
            description: "list directory contents".to_string(),
            function: ls_cmd,
            action: CmdResultActionType::Display,
        });

        registry.register_command(CommandDefinition {
            name: "cat".to_string(),
            description: "concatenate files and print on the standard output".to_string(),
            function: cat_cmd,
            action: CmdResultActionType::Display,
        });

        registry.register_command(CommandDefinition {
            name: "open".to_string(),
            description: "open a file".to_string(),
            function: open_cmd,
            action: CmdResultActionType::Navigate,
        });

        registry
    }

    fn register_command(&mut self, command_definition: CommandDefinition) {
        self.commands
            .insert(command_definition.name.clone(), command_definition);
    }

    fn execute_command(&self, cmd_name: &str, args: Vec<String>) -> Option<WebEvaluatorResult> {
        if let Some(command) = self.commands.get(cmd_name) {
            let o = (command.function)(args, self.fs.clone());

            if o.is_success {
                match command.action {
                    CmdResultActionType::Display => Some(WebEvaluatorResult::Display(o.result)),
                    CmdResultActionType::Navigate => Some(WebEvaluatorResult::Navigate(o.result)),
                }
            } else {
                Some(WebEvaluatorResult::Error(o.result))
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

    fn help(&self) -> String {
        let mut output = String::new();
        self.commands.iter().for_each(|f| {
            output.push_str(&format!("  * {} - {}\n", f.1.name, f.1.description));
        });

        output
    }
}
