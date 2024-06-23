use crate::{parser::ParserResult, registries::CommandRegistry};

#[derive(Debug)]
pub enum WebEvaluatorResult {
    Navigate(String),
    Display(String),
    Error(String),
}

#[derive(Debug)]
pub enum CmdResultActionType {
    Navigate,
    Display,
}

pub struct WebEvaluator {
    pub stmt: Vec<ParserResult>,
}

impl WebEvaluator {
    pub fn eval(
        stmt: Vec<ParserResult>,
        registory: impl CommandRegistry,
    ) -> Vec<WebEvaluatorResult> {
        let mut results = Vec::new();

        for node in stmt {
            match node {
                ParserResult::Error(data) => {
                    println!("{}", data)
                }
                ParserResult::SystemCmd(cmd, tokens) => {
                    let mut args: Vec<String> = [].to_vec();

                    tokens.iter().for_each(|f| args.push(f.value.clone()));

                    match registory.execute_command(&cmd, args) {
                        Some(data) => results.push(data),
                        _ => {}
                    };
                }
                ParserResult::FunctionCall(cmd, _) => {
                    // function calls are dissabled for the web evaluator
                    results.push(WebEvaluatorResult::Error(format!(
                        "command not found: {}",
                        cmd
                    )));
                }
                parser_results => results.push(WebEvaluatorResult::Error(format!(
                    "failed to execute the command: {:?}",
                    parser_results
                ))),
            }
        }

        results
    }
}
