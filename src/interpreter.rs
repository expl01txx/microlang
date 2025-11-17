use crate::parser::Inst;
use core::panic;
use std::{collections::HashMap, io::Write};

#[macro_export]
macro_rules! stacklen {
    ($self:ident, $len:expr, $name:literal) => {
        if $self.stack.len() < $len {
            panic!("Stack underflow at {}", $name);
        }
    };
}

#[macro_export]
macro_rules! getarg {
    ($self:ident, $type:ident, $name:literal) => {
        (|| {
            if let Some(tmp) = $self.stack.pop() {
                match tmp {
                    MValue::$type(x) => {
                        return x;
                    }
                    _ => panic!("Incorrect arguments at {}", $name),
                }
            }
            panic!("Failed to fetch argument");
        })()
    };
}

#[derive(Clone, Debug)]
pub enum MValue {
    String(String),
    Number(i32),
    VStack(Vec<MValue>),
}

pub struct Interpreter {
    //base
    ip: usize,
    insts: Vec<Inst>,
    //stack
    pub stack: Vec<MValue>,
    return_stack: Vec<usize>,
    //functions and memory
    labels: HashMap<String, usize>,
    variables: HashMap<String, MValue>,
    functions: HashMap<String, fn(&mut Interpreter)>,
}

impl Interpreter {
    pub fn new(insts: Vec<Inst>) -> Interpreter {
        return Interpreter {
            ip: 0,
            stack: Vec::new(),
            return_stack: Vec::new(),
            labels: HashMap::new(),
            insts,
            variables: HashMap::new(),
            functions: HashMap::new(),
        };
    }

    fn find_labels(&mut self) {
        while self.ip < self.insts.len() {
            let inst = &self.insts[self.ip];
            self.ip += 1;

            match inst {
                Inst::Label(name) => _ = self.labels.insert(name.to_string(), self.ip),
                _ => {}
            }
        }
        self.ip = 0;
    }

    pub fn add_function(&mut self, name: &str, func: fn(&mut Interpreter)) {
        self.functions.insert(name.to_owned(), func);
    }

    fn execute_instruction(&mut self, enable_stacktrace: bool) {
        let inst = &self.insts[self.ip];
        self.ip += 1;

        if enable_stacktrace {
            println!("{:?}:{} => {:?}", inst, self.ip, self.stack);
        }
        match inst {
            Inst::LoadString(str) => {
                self.stack.push(MValue::String(str.clone()));
            }
            Inst::LoadNumber(num) => {
                self.stack.push(MValue::Number(*num));
            }
            Inst::Add => {
                stacklen!(self, 2, "add");
                let x = getarg!(self, Number, "add");
                let y = getarg!(self, Number, "add");
                self.stack.push(MValue::Number(x + y));
            }
            Inst::Sub => {
                stacklen!(self, 2, "sub");
                let x = getarg!(self, Number, "sub");
                let y = getarg!(self, Number, "sub");
                self.stack.push(MValue::Number(x - y));
            }
            Inst::Mul => {
                stacklen!(self, 2, "mul");
                let x = getarg!(self, Number, "mul");
                let y = getarg!(self, Number, "mul");
                self.stack.push(MValue::Number(x * y));
            }
            Inst::Div => {
                stacklen!(self, 2, "mul");
                let x = getarg!(self, Number, "mul");
                let y = getarg!(self, Number, "mul");
                self.stack.push(MValue::Number(x / y));
            }
            Inst::Dup => {
                if self.stack.len() > 0 {
                    let last = self.stack.last().unwrap().clone();
                    self.stack.push(last);
                } else {
                    panic!("Stack underflow at dup.");
                }
            }
            Inst::Swap => {
                self.stack.reverse();
            }
            Inst::VSwap => {
                stacklen!(self, 1, "vswap");
                let name = getarg!(self, String, "vswap");
                let vstack = &self.variables[&name];
                match vstack {
                    MValue::VStack(data) => {
                        let mut_obj: &mut Vec<MValue> = unsafe {
                            std::mem::transmute(data as *const Vec<MValue> as *mut Vec<MValue>)
                        };
                        mut_obj.reverse();
                    }
                    _ => {
                        panic!("Cannot vswap {vstack:?}");
                    }
                }
            }
            Inst::Ret => {
                if let Some(address) = self.return_stack.pop() {
                    self.ip = address;
                } else {
                    panic!("Return stack underflow.");
                }
            }
            Inst::Pop => {
                if let Some(address) = self.stack.pop() {
                } else {
                    panic!("Return pop underflow.");
                }
            }
            Inst::VPop => {
                stacklen!(self, 1, "vpop");
                let name = getarg!(self, String, "vpop");
                let vstack = &self.variables[&name];
                match vstack {
                    MValue::VStack(data) => {
                        let mut_obj: &mut Vec<MValue> = unsafe {
                            std::mem::transmute(data as *const Vec<MValue> as *mut Vec<MValue>)
                        };
                        mut_obj.pop();
                    }
                    _ => {
                        panic!("Cannot vswap {vstack:?}");
                    }
                }
            }

            Inst::DefineVar(name) => {
                self.variables.insert(name.to_owned(), MValue::Number(0));
            }
            Inst::DefineVStack(name) => {
                self.variables
                    .insert(name.to_owned(), MValue::VStack(Vec::new()));
            }
            Inst::ReadVar(name) => {
                if self.variables.contains_key(name) {
                    let var = &self.variables[name];
                    match var {
                        MValue::VStack(data) => {
                            let mut_obj: &mut Vec<MValue> = unsafe {
                                std::mem::transmute(data as *const Vec<MValue> as *mut Vec<MValue>)
                            };
                            if let Some(value) = mut_obj.pop() {
                                self.stack.push(value);
                            } else {
                                panic!("VStack underflow at read_var")
                            }
                        }
                        _ => {
                            self.stack.push(var.clone());
                        }
                    }
                } else {
                    panic!("Variable with name {name} does not exist.")
                }
            }
            Inst::WriteVar(name) => {
                stacklen!(self, 1, "write_var");
                if let Some(value) = self.stack.pop() {
                    if self.variables.contains_key(name) {
                        let source_type = &self.variables[name];
                        match source_type {
                            MValue::VStack(data) => {
                                let mut_obj: &mut Vec<MValue> = unsafe {
                                    std::mem::transmute(
                                        data as *const Vec<MValue> as *mut Vec<MValue>,
                                    )
                                };
                                mut_obj.push(value);
                            }
                            _ => {
                                self.variables.insert(name.to_owned(), value);
                            }
                        }
                    } else {
                        panic!("Variable with name {name} does not exist.")
                    }
                }
            }
            Inst::Jmp => {
                if self.stack.len() > 0 {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            MValue::Number(addr) => {
                                self.ip = addr as usize;
                            }
                            _ => {
                                panic!("Failed to jump to address stored in a string")
                            }
                        }
                    }
                } else {
                    panic!("Stack underflow at jmp.");
                }
            }
            Inst::Jz => {
                if self.stack.len() > 1 {
                    if let Some(address) = self.stack.pop() {
                        if let Some(flag) = self.stack.pop() {
                            match address {
                                MValue::Number(addr) => match flag {
                                    MValue::Number(flag) => {
                                        if flag == 0 {
                                            self.ip = addr as usize;
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {
                                    panic!("Failed to jump to address stored in a string")
                                }
                            }
                        }
                    }
                } else {
                    panic!("Stack underflow at Jz.");
                }
            }
            Inst::Jnz => {
                if self.stack.len() > 1 {
                    if let Some(address) = self.stack.pop() {
                        if let Some(flag) = self.stack.pop() {
                            match address {
                                MValue::Number(addr) => match flag {
                                    MValue::Number(flag) => {
                                        if flag != 0 {
                                            self.ip = addr as usize;
                                        }
                                    }
                                    _ => {}
                                },
                                _ => {
                                    panic!("Failed to jump to address stored in a string")
                                }
                            }
                        }
                    }
                } else {
                    panic!("Stack underflow at Jnz.");
                }
            }
            Inst::Str2num => {
                if self.stack.len() > 0 {
                    if let Some(value) = self.stack.pop() {
                        match value {
                            MValue::String(str) => {
                                self.stack.push(MValue::Number(str.trim().parse().unwrap()))
                            }
                            MValue::Number(_) => {
                                panic!("Trying to convert number to number")
                            }
                            MValue::VStack(_) => {
                                panic!("Trying to convert VStack to number")
                            }
                        }
                    }
                } else {
                    panic!("Stack underflow at s2n.");
                }
            }
            Inst::Strat => {
                stacklen!(self, 2, "strat");
                let index = getarg!(self, Number, "strat");
                let string = getarg!(self, String, "strat");

                if let Some(c) = string.chars().nth(index as usize) {
                    self.stack.push(MValue::String(c.to_string()));
                } else {
                    panic!("Index out of bounds at strat");
                }
            }
            Inst::Strlen => {
                stacklen!(self, 1, "strlen");
                let string = getarg!(self, String, "strlen");
                self.stack.push(MValue::Number(string.len() as i32));
            }
            Inst::Strjoin => {
                stacklen!(self, 2, "strjoin");
                let x = getarg!(self, String, "strjoin");
                let y = getarg!(self, String, "strjoin");
                self.stack.push(MValue::String(y + &x))
            }
            Inst::Put => {
                stacklen!(self, 1, "put");
                if let Some(value) = self.stack.pop() {
                    match value {
                        MValue::String(str) => {
                            print!("{}", str);
                            std::io::stdout().flush().unwrap();
                        }
                        MValue::Number(num) => {
                            print!("{}", num);
                            std::io::stdout().flush().unwrap();
                        }
                        MValue::VStack(stack) => {
                            print!("{:?}", stack);
                            std::io::stdout().flush().unwrap();
                        }
                    }
                }
            }
            Inst::Endl => {
                println!("");
            }
            Inst::In => {
                let mut user_input = String::new();
                std::io::stdin()
                    .read_line(&mut user_input)
                    .expect("Failed to read line");
                self.stack
                    .push(MValue::String(user_input.trim().to_owned()));
            }
            Inst::Cmp => {
                stacklen!(self, 2, "cmp");
                if let Some(x) = self.stack.pop() {
                    if let Some(y) = self.stack.pop() {
                        match x {
                            MValue::String(x) => match y {
                                MValue::String(y) => {
                                    self.stack.push(MValue::Number(if x == y { 0 } else { 1 }))
                                }
                                MValue::Number(_) => {
                                    panic!("Trying to compare string with number")
                                }
                                MValue::VStack(_) => {
                                    panic!("Trying to compare VStack with number")
                                }
                            },
                            MValue::Number(x) => match y {
                                MValue::Number(y) => self.stack.push(MValue::Number(x - y)),
                                MValue::String(_) => {
                                    panic!("Trying to compare number with string")
                                }
                                MValue::VStack(_) => {
                                    panic!("Trying to compare Number with VStack")
                                }
                            },
                            MValue::VStack(_) => panic!("VStack cmp does not allowed"),
                        }
                    }
                }
            }
            Inst::Stackdmp => {
                println!("\n--- STACK ---\n{:?}\n--- END ---\n", self.stack);
            }
            Inst::Stacklen => {
                self.stack.push(MValue::Number(self.stack.len() as i32));
            }
            Inst::VStacklen => {
                stacklen!(self, 1, "vslen");
                let name = getarg!(self, String, "vslen");
                let vstack = &self.variables[&name];
                match vstack {
                    MValue::VStack(data) => {
                        self.stack.push(MValue::Number(data.len() as i32));
                    }
                    _ => {
                        panic!("Cannot vslen {vstack:?}");
                    }
                }
            }
            Inst::LabelAddress(name) => self.stack.push(MValue::Number(self.labels[name] as i32)),
            Inst::CustomInstruction(name) => {
                if self.functions.contains_key(name) {
                    self.functions[name](self);
                    return;
                }
                let addr = self.labels[name];
                self.return_stack.push(self.ip);
                self.ip = addr;
            }
            _ => {}
        }
    }

    pub fn execute(&mut self, enable_stacktrace: bool) {
        self.find_labels();
        if self.labels.contains_key("main") {
            let addr = self.labels["main"];
            self.return_stack.push(self.ip);
            self.ip = addr;
        } else {
            panic!("Main function doesnt found.");
        }
        while self.ip < self.insts.len() {
            self.execute_instruction(enable_stacktrace);
        }
    }
}
