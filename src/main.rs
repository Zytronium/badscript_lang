use std::collections::HashMap;
use std::{env, io, time};
use std::fs;
use std::io::Write;
use std::process;
use std::thread::sleep;
use colored_print::ceprintln;
use colored_print::ceprint;

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    Str(String),
    Bool(bool),
    Null,
}

type BuiltinFn = fn(Vec<Value>) -> Result<Value, String>;

#[derive(Clone)]
struct Builtin {
    name: &'static str,
    arity: Option<usize>, // None means arbitrary number of args
    func: BuiltinFn,
}

fn opt_str(args: &[Value]) -> Result<Option<&str>, String> {
    match args.first() {
        None => Ok(None),
        Some(Value::Str(s)) => Ok(Some(s.as_str())),
        Some(_) => Err(String::from("expected a string argument")),
    }
}

fn builtins() -> HashMap<&'static str, Builtin> {
    let mut map = HashMap::new();

    let list = [
        Builtin { name: "print", arity: None, func: bs_fn_print },
        Builtin { name: "println", arity: None, func: bs_fn_println },
        Builtin { name: "printerr", arity: None, func: bs_fn_printerr },
        Builtin { name: "printlnerr", arity: None, func: bs_fn_printlnerr },
        Builtin { name: "noop", arity: Some(0), func: bs_fn_noop },
        Builtin { name: "sleep", arity: Some(1), func: bs_fn_sleep },
        Builtin { name: "exit", arity: None, func: bs_fn_exit },
    ];

    for b in list {
        map.insert(b.name, b);
    }

    map
}

fn bs_fn_print(args: Vec<Value>) -> Result<Value, String> {
    let msg = opt_str(&args)?;
    print!("{}", msg.unwrap_or(""));
    io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(Value::Null)
}
fn bs_fn_println(args: Vec<Value>) -> Result<Value, String> {
    let msg = opt_str(&args)?;
    println!("{}", msg.unwrap_or(""));
    Ok(Value::Null)
}

fn bs_fn_printerr(args: Vec<Value>) -> Result<Value, String> {
    let emsg = opt_str(&args)?;
    ceprint!("%R:{}", emsg.unwrap_or("Error"));
    io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(Value::Null)
}

fn bs_fn_printlnerr(args: Vec<Value>) -> Result<Value, String> {
    let emsg = opt_str(&args)?;
    ceprintln!("%R:{}", emsg.unwrap_or("Error"));
    Ok(Value::Null)
}

fn bs_fn_noop(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Null)
}

fn bs_fn_sleep(args: Vec<Value>) -> Result<Value, String> {
    let sec = match args.first() {
        None => return Err(String::from("expected 1 argument. Found 0.")),
        Some(Value::Number(n)) => *n as f64,
        Some(_) => return Err(String::from("sleep expects a number")),
    };

    sleep(time::Duration::from_secs_f64(sec));

    Ok(Value::Null)
}

fn bs_fn_exit(args: Vec<Value>) -> Result<Value, String> {
    let c = match args.first() {
        None => 0,
        Some(Value::Number(n)) => *n as i32,
        Some(_) => return Err(String::from("exit expects a number")),
    };
    // println!();
    // println!("Process exited with exit code {c}");
    process::exit(c)
}

struct Lexer {
    tokens: Vec<String>,
    current_token: String,
    prev_char: char,
    next_esc_seq: bool,
    comment_mode: bool,
    string_mode: bool,
    line: i32,
    col: i32,
}

impl Lexer {
    fn new() -> Self {
        Lexer {
            tokens: Vec::new(),
            current_token: String::new(),
            prev_char: ' ',
            next_esc_seq: false,
            comment_mode: false,
            string_mode: false,
            line: 1,
            col: 1,
        }
    }

    fn new_token(&mut self) {
        let tok = std::mem::take(&mut self.current_token).trim().to_string();
        if !tok.is_empty() {
            self.tokens.push(tok);
        }
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.col = 1;
        self.new_token();
    }
}

fn throw_error(msg: String) -> ! {
    ceprintln!("%R:Error: {msg}");
    process::exit(1);
}

fn throw_syntax_error(msg: String) -> ! {
    ceprintln!("%R:Syntax error: {msg}");
    process::exit(1);
}

fn main() {
    let file_path = match env::args().nth(1) {
        Some(p) => p,
        None => {
            eprintln!("Usage: badscript <file>");
            process::exit(1);
        }
    };

    let script = fs::read_to_string(file_path)
        .expect("Failed to read file");

    let _builtins = builtins();

    let mut lx = Lexer::new();

    for char in script.chars() {
        // todo: don't error if char is '\\' and next char is '\n'

        // In comment mode, ignore all characters except newline, which turns off comment mode
        if lx.comment_mode && char != '\n' {
            lx.prev_char = char;
            lx.col += 1;
            continue;
        }
        if lx.next_esc_seq && !lx.string_mode && char != '\n' {
            throw_syntax_error(format!("{}:{} - Escape sequences are only allowed in strings or to break a line into two lines", lx.line, lx.col - 1));
        }
        match char {
            // newlines - ends line, token, and comment \\
            '\n' => {
                if lx.string_mode {
                    throw_syntax_error(format!("{}:{} - Unclosed string.", lx.line, lx.col));
                }
                lx.next_line();
                if lx.comment_mode {
                    lx.comment_mode = false;
                }
                if lx.prev_char == '\\' {
                    lx.next_esc_seq = false;
                }
                continue; // don't increment col (next_line() also sets prev_char so we can skip that safely too)
            }
            // double quotes - strings unless inside comments \\
            '"' => {
                if !lx.comment_mode {
                    lx.string_mode = !lx.string_mode;
                    lx.current_token.push(char);
                }
            }
            // hashtags - comments unless inside strings \\
            '#' => {
                if !lx.string_mode {
                    lx.comment_mode = true;
                } else {
                    lx.current_token.push(char);
                }
            }
            // spaces \\
            ' ' => {
                if !lx.string_mode {
                    lx.new_token();
                } else {
                    lx.current_token.push(char);
                }
            }
            // backslash - starts escape sequence inside strings \\
            '\\' => {
                if lx.next_esc_seq && lx.string_mode {
                    lx.current_token.push('\\');
                    lx.next_esc_seq = false;
                } else {
                    lx.next_esc_seq = true;
                }
            }
            // escape sequences \\
            'n' if lx.prev_char == '\\' && lx.string_mode == true => {
                lx.current_token.push('\n');
            }
            _ if lx.prev_char == '\\' && lx.string_mode == true => {
                throw_syntax_error(format!("{}:{} - Unknown escape sequence \\{}.", lx.line, lx.col - 1, char));
            }
            // all other chars \\
            _ => {
                lx.current_token.push(char);
            }
        }

        if lx.prev_char == '\\' {
            lx.next_esc_seq = false;
        }
        lx.prev_char = char;
        lx.col += 1;
    }

    for token in &lx.tokens {
        let Some((fn_name, rest)) = token.split_once('(') else {
            // not a function call since there's no '('
            throw_error(format!("{}:{} - Unknown expression \"{}\"", "line", "col", token));
        };

        let Some(builtin) = _builtins.get(fn_name) else {
            throw_error(format!("{}:{} - Unknown function \"{}()\"", "line", "col", fn_name)); // TODO: get line and col of error location. NOTE: You can do this by turning `token` into tuples of text and location (location itself would be another tuple of line and col).
        };

        let Some(inner) = rest.strip_suffix(')') else {
            throw_syntax_error(format!("{}:{} - Unclosed parentheses at function call in \"{}\"", "line", "col", token)); // TODO: get line and col of error location.
        };

        let args: Vec<Value> = if inner.is_empty() {
            Vec::new()
        } else if let Some(s) = inner.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
            vec![Value::Str(s.to_string())]
        } else if let Ok(n) = inner.parse::<f64>() {
            vec![Value::Number(n)]
        } else {
            throw_error(format!("{}:{} - could not parse argument \"{}\"", "line", "col", inner)); // TODO: get line and col of error location.
        };

        if let Some(n) = builtin.arity {
            if args.len() != n {
                throw_error(format!("{}:{} - {} expects {} args; got {} instead.", "line", "col", builtin.name, n, args.len()));
            }
        }

        if let Err(e) = (builtin.func)(args) {
            throw_error(format!("Error: {}", e));
        }
    }
}
