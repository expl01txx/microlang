# Microlang - simple stack-based interpreted programming language written in rustlang

### - Hello World:
```
main:
"Hello, World!" puts
```

### - Simple calc:
```
@main jmp

add: in s2n in s2n add put ret
sub: in s2n in s2n sub put ret
mul: in s2n in s2n mul put ret
div: in s2n in s2n div put ret

main:
    "Enter op (+-*/): " put in dup 
    "+" cmp @add jz dup
    "-" cmp @sub jz dup
    "*" cmp @mul jz
    "/" cmp @div jz
```

### - FFI

define custom function:
```rust
fn custom_print(interpreter: &mut Interpreter) {
    let value = interpreter.stack.pop();

    if let Some(value) = value {
        match value {
            MValue::VNumber(data) => {
                println!("Number: {data}");
            }
            MValue::VString(data) => {
                println!("String: {data}");
            }
            _ => {}
        }
    }
}

```

Add to interpreter
```rust
let mut interpreter = Interpreter::new(insts);
interpreter.add_function("cprint", custom_print);
```

Use in script
```
main:
#0 cprint
"Hello, World!" cprint
```

Result
```
Number: 0
String: Hello, World!
```