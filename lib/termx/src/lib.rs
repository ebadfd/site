/*
* language semantics.
*
* only allow defined commands to run;
* - whoami
* - cd
* - cat
* - echo
*
* Optional commands;
* - for loops
*
* Comment semantics are #
*/

use evaluator::WebEvaluator;
use lexer::Lexer;
use parser::Parser;
use registries::CommandRegistry;

pub mod errors;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod registries;

pub fn default_help_short() -> String {
    return r#"
Welcome to the Cloud Shell!

Cloud shell is a browser-based shell with bash commands pre-installed. 

- View supported commands: `help`
    "#
    .to_string();
}

pub fn run(input: String, registry: impl CommandRegistry) -> Vec<evaluator::WebEvaluatorResult> {
    let lex = Lexer::new(input);
    let mut parser = Parser::new(lex);
    let stmt = parser.parse(registry.list_available_commands()).unwrap();
    let eval = WebEvaluator::eval(stmt, registry);
    return eval;
}
