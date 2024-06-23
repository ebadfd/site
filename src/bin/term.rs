use color_eyre::Result;
use termx::{lexer::Lexer, parser::Parser};

#[tokio::main]
async fn main() -> Result<()> {
    let _function = "simpleinputs() {\necho \"This is the first argument [$1]\" \necho \"This is the second argument [$2]\"\necho \"Calling function with $# arguments\"\n}\nwhoami";
    let default_input = "#whoami\nhelloworld\nwhoami;\nid\n12;\necho\"Hello world\"";
    let lex = Lexer::new(default_input.to_string());
    let mut parser = Parser::new(lex);
    let stmt = parser.parse().unwrap();
    //let eval = WebEvaluator::eval(stmt);
    println!("{:?}", stmt);

    Ok(())
}
