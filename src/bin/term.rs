use color_eyre::Result;
use termx::Lexer;

#[tokio::main]
async fn main() -> Result<()> {
    let mut lex = Lexer::new("#whoami\nhelloworld\nwhoami".to_string());
    println!("{:?}", lex);
    lex.next_token();
    println!("{:?}", lex);

    lex.next_token();
    println!("{:?}", lex);

    lex.next_token();
    println!("{:?}", lex);

    Ok(())
}
