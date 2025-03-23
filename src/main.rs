mod lexer;
mod parser;
mod errors;


pub use lexer::*;
pub use parser::*;
pub use errors::*;

fn main() {


    let text = r#"
    
            if (4 < 5) { hello; }
            else { something; }
            

        "#;
    
    let tokens = lex(text);

    //println!("{:#?}", tokens.clone());

    let ast = parse(tokens);

    println!("{:#?}", ast);
}
