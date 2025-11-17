use logos::Lexer;

use crate::lexer::Token;

#[derive(PartialEq, Debug)]
pub enum Inst {
    //Math & stack
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    VSwap,
    Pop,
    VPop,
    Stacklen,
    VStacklen,

    //Flow
    Jmp,
    Jz,
    Jnz,
    Ret,

    //Converts And string manipulations
    Str2num,
    Strat, //Get char from string by pos
    Strlen,
    Strjoin,

    //IO
    Put,
    Endl,
    In,

    //Compares
    Cmp,
    Eof,

    //Debug stack
    Stackdmp,

    //Types & Loaders
    CustomInstruction(String),
    Label(String),
    LabelAddress(String),
    DefineVStack(String),
    DefineVar(String),
    ReadVar(String),
    WriteVar(String),
    LoadString(String),
    LoadNumber(i32),
}

pub struct Parser<'a> {
    lexer: Lexer<'a, Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<Token>) -> Parser {
        return Parser { lexer };
    }

    pub fn parse_instruction(&mut self) -> Option<Inst> {
        if let Some(token) = self.lexer.next() {
            if let Ok(token) = token {
                match token {
                    Token::Ident => {
                        let slice = self.lexer.slice();
                        match slice {
                            "add" => {
                                return Some(Inst::Add);
                            }
                            "sub" => {
                                return Some(Inst::Sub);
                            }
                            "mul" => {
                                return Some(Inst::Mul);
                            }
                            "div" => {
                                return Some(Inst::Div);
                            }
                            "dup" => {
                                return Some(Inst::Dup);
                            }
                            "swap" => {
                                return Some(Inst::Swap);
                            }
                            "vswap" => {
                                return Some(Inst::VSwap);
                            }
                            "pop" => {
                                return Some(Inst::Pop);
                            }
                            "vpop" => {
                                return Some(Inst::VPop);
                            }
                            "slen" => return Some(Inst::Stacklen),
                            "vslen" => return Some(Inst::VStacklen),
                            "s2n" => {
                                return Some(Inst::Str2num);
                            }
                            "strat" => {
                                return Some(Inst::Strat);
                            }
                            "strlen" => {
                                return Some(Inst::Strlen);
                            }
                            "strjoin" => {
                                return Some(Inst::Strjoin);
                            }
                            "put" => {
                                return Some(Inst::Put);
                            }
                            "endl" => {
                                return Some(Inst::Endl);
                            }
                            "in" => {
                                return Some(Inst::In);
                            }
                            "jmp" => {
                                return Some(Inst::Jmp);
                            }
                            "jz" => {
                                return Some(Inst::Jz);
                            }
                            "jnz" => {
                                return Some(Inst::Jnz);
                            }
                            "ret" => {
                                return Some(Inst::Ret);
                            }
                            "cmp" => {
                                return Some(Inst::Cmp);
                            }
                            "stackdmp" => {
                                return Some(Inst::Stackdmp);
                            }
                            _ => return Some(Inst::CustomInstruction(slice.to_owned())),
                        }
                    }
                    Token::Number => {
                        let slice = self.lexer.slice();
                        match i32::from_str_radix(&slice[1..], 16) {
                            Ok(num) => return Some(Inst::LoadNumber(num)),
                            Err(_) => {
                                eprintln!("Failed to parse number from token: {}", slice);
                                return None;
                            }
                        }
                    }
                    Token::String => {
                        let slice = self.lexer.slice();
                        return Some(Inst::LoadString(slice[1..slice.len() - 1].to_owned()));
                    }
                    Token::Label => {
                        let slice = self.lexer.slice();
                        return Some(Inst::Label(slice[..slice.len() - 1].to_owned()));
                    }
                    Token::LabelAdress => {
                        let slice = self.lexer.slice();
                        return Some(Inst::LabelAddress(slice[1..].to_owned()));
                    }
                    Token::Dollar => {
                        let name = self.expect_ident();
                        return Some(Inst::DefineVar(name));
                    }
                    Token::Percent => {
                        let name = self.expect_ident();
                        return Some(Inst::DefineVStack(name));
                    }
                    Token::Greater => {
                        let name = self.expect_ident();
                        return Some(Inst::ReadVar(name));
                    }
                    Token::Less => {
                        let name = self.expect_ident();
                        return Some(Inst::WriteVar(name));
                    }
                    Token::Comment => {
                        unimplemented!()
                    }
                    Token::Include => {}
                }
            }
            return None;
        }
        return Some(Inst::Eof);
    }

    fn expect_ident(&mut self) -> String {
        if let Some(tok) = self.lexer.next() {
            if let Ok(tok) = tok {
                match tok {
                    Token::Ident => {
                        let slice = self.lexer.slice();
                        return slice.to_owned();
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

    fn expect(&mut self, token: Token) -> bool {
        if let Some(tok) = self.lexer.next() {
            if let Ok(tok) = tok {
                return tok == token;
            } else {
                panic!("Failed to get token next to {:?}", token);
            }
        } else {
            panic!("Failed to get token next to {:?}", token);
        }
    }

    pub fn parse(&mut self) -> Vec<Inst> {
        let mut insts = Vec::new();
        loop {
            let inst = self.parse_instruction();
            if let Some(inst) = inst {
                if inst == Inst::Eof {
                    break;
                }
                insts.push(inst);
            }
        }
        return insts;
    }
}
