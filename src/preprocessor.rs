use logos::{Lexer, Logos};

use crate::lexer::Token;

pub fn preprocess(source: String) -> String {
    let mut output = "".to_owned();
    let binding = source.clone();
    let mut lex = Token::lexer(&binding);

    while let Some(token) = lex.next() {
        if let Ok(token) = token {
            match token {
                Token::Include => {
                    let filename = expect_string(&mut lex);
                    if let Ok(content) = std::fs::read_to_string(filename.clone()) {
                        output += &preprocess(content);
                    } else {
                        panic!("File {filename} not found.")
                    }
                }
                _ => {}
            }
        }
    }

    output +="\n";
    output += &source;

    return output;
}

fn expect_string(lexer: &mut Lexer<'_, Token>) -> String {
    if let Some(tok) = lexer.next() {
        if let Ok(tok) = tok {
            match tok {
                Token::String => {
                    let slice = lexer.slice();
                    return slice[1..slice.len()-1].to_owned();
                }
                _ => {
                    panic!("Expected ident");
                }
            }
        } else {
            panic!("Expected ident");
        }
    } else {
        panic!("Expected ident");
    }
}
