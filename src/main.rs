mod lexer;
mod parser;
mod errors;


pub use lexer::*;
pub use parser::*;
pub use errors::*;

fn main() {


    let text = r#"

            a = true and false;

        "#;
    
    let tokens = lex(text);

    let ast = parse(tokens);

    println!("{:#?}", ast);
}
