mod lexer;

pub use lexer::*;

fn main() {


    let text = r#"
    
            let p = 5;

            let s = "hello there";


        "#;
    
    let tokens = lex(text);


    println!("{:#?}", tokens);
}
