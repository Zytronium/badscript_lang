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

fn value_to_string(val: &Value) -> String {
    match val {
        Value::Number(n) => n.to_string(),
        Value::Str(s) => s.clone(),
        Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Value::Null => "Null".to_string(),
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
    let msg = args.first().map(value_to_string).unwrap_or_default();
    print!("{}", msg);
    io::stdout().flush().map_err(|e| e.to_string())?;
    Ok(Value::Null)
}
fn bs_fn_println(args: Vec<Value>) -> Result<Value, String> {
    let msg = args.first().map(value_to_string).unwrap_or_default();
    println!("{}", msg);
    Ok(Value::Null)
}

fn bs_fn_printerr(args: Vec<Value>) -> Result<Value, String> {
    let emsg = args.first().map(value_to_string).unwrap_or_else(|| "Error".to_string());
    ceprint!("%R:{}", emsg);
    Ok(Value::Null)
}

fn bs_fn_printlnerr(args: Vec<Value>) -> Result<Value, String> {
    let emsg = args.first().map(value_to_string).unwrap_or_else(|| "Error".to_string());
    ceprintln!("%R:{}", emsg);
    Ok(Value::Null)
}

fn bs_fn_noop(_args: Vec<Value>) -> Result<Value, String> {
    Ok(Value::Null)
}

fn bs_fn_sleep(args: Vec<Value>) -> Result<Value, String> {
    let sec = match args.first() {
        None => return Err(String::from("expected 1 argument. Found 0.")),
        Some(Value::Number(n)) => *n,
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
    process::exit(c)
}

#[derive(Debug, Clone)]
struct Variable {
    name: String,
    value: Value,
}

impl Variable {
    fn new(name: String, value: Value) -> Self {
        Variable {
            name,
            value,
        }
    }
}

#[derive(Debug, Clone)]
struct Token {
    value: String,
    line: usize,
    col: usize, // Start column of token
}

impl Token {
    fn new(value: String, line: usize, col: usize) -> Self {
        Token { value, line, col }
    }
}

#[derive(Debug, Clone)]
struct Statement {
    tokens: Vec<Token>,
    line: usize, // starting line - can span multiple lines by escaping newlines
}

impl Statement {
    fn new(tokens: Vec<Token>, line: usize) -> Self {
        Statement { tokens, line }
    }
}

struct Lexer {
    statements: Vec<Statement>,
    current_statement: Vec<Token>,
    current_token: String,
    token_start_col: usize,
    prev_char: char,
    next_esc_seq: bool,
    comment_mode: bool,
    string_mode: bool,
    line: usize,
    col: usize,
}

impl Lexer {
    fn new() -> Self {
        Lexer {
            statements: Vec::new(),
            current_statement: Vec::new(),
            current_token: String::new(),
            token_start_col: 1,
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
            self.current_statement.push(Token::new(tok, self.line, self.token_start_col));
        }
        self.token_start_col = self.col;
    }

    fn new_statement(&mut self) {
        self.new_token();
        let stat = std::mem::take(&mut self.current_statement);
        if !stat.is_empty() {
            self.statements.push(Statement::new(stat, self.line));
        }
        self.token_start_col = self.col;
    }

    fn next_line(&mut self) {
        self.line += 1;
        self.col = 1;
        if self.prev_char != '\\' {
            self.new_statement();
        } else {
            self.new_token();
        }
    }
}

fn parse_value(raw: &str) -> Result<Value, String> {
    if let Some(s) = raw.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
        Ok(Value::Str(s.to_string()))
    } else if let Ok(n) = raw.parse::<f64>() {
        Ok(Value::Number(n))
    } else if raw == "True" {
        Ok(Value::Bool(true))
    } else if raw == "False" {
        Ok(Value::Bool(false))
    } else if raw == "Null" {
        Ok(Value::Null)
    } else {
        Err(format!("Could not parse value \"{}\"",raw))
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

    let builtins = builtins();
    let keywords = [
        "var",
        "if",
        "else",
        // "func",
    ];
    let operators = [
        "+",
        "-",
        "/",
        "*",
    ];
    let assignment_operators = [
        "=",
        "+=",
        "-=",
        "/=",
        "*=",
    ];
    let comparators = [
        "<",
        ">",
        "<=",
        ">=",
        "!=",
        "==",
    ];
    let mut variables: Vec<Variable> = Vec::new();

    let mut lx = Lexer::new();

    for char in script.chars() {
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
            // escape sequences \\
            'n' if lx.prev_char == '\\' && lx.string_mode => {
                lx.current_token.push('\n');
            }
            _ if lx.prev_char == '\\' && lx.string_mode => {
                throw_syntax_error(format!("{}:{} - Unknown escape sequence \\{}.", lx.line, lx.col - 1, char));
            }
            // newlines - ends line, token, and comment \\
            '\n' => {
                if lx.string_mode {
                    throw_syntax_error(format!("{}:{} - Unclosed string.", lx.line, lx.col));
                }
                lx.next_line();
                lx.comment_mode = false;
                if lx.prev_char == '\\' { // Don't error if last char of line is backslash
                    lx.next_esc_seq = false;
                }
                lx.prev_char = char;
                continue; // don't increment col - next_line() sets it to 1 because it also increments line
            }
            // double quotes - strings unless inside comments \\
            '"' => {
                if !lx.string_mode {
                    lx.token_start_col = lx.col;
                }
                lx.string_mode = !lx.string_mode;
                lx.current_token.push(char);
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
            // code blocks/branches \\
            '{' => { // Start code block unless inside a string - single-char token, let parser handle behavior
                if !lx.string_mode {
                    lx.new_token(); // End prev token if not empty (no-op otherwise)
                    lx.current_token.push(char); // Push this char to new token
                    lx.new_statement(); // End new token and statement
                }
            }
            '}' => { // End code block unless inside a string - single-char token, let parser handle behavior
                if !lx.string_mode {
                    lx.new_token(); // End prev token if not empty
                    lx.current_token.push(char); // Push this char to new token
                    lx.new_statement(); // End new token and statement
                }
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
    // Final touches - check for unclosed strings, end final statement
    if lx.string_mode {
        throw_syntax_error(format!("{}:{} - Unclosed string.", lx.line, lx.col));
    }
    lx.new_statement(); // End last statement if script doesn't end in a whitespace

    fn is_function_call(token: &Token) -> bool {
        let val = &token.value;
        let mut opening_pars_found = 0;
        let mut closing_pars_found = 0;
        for char in val.chars() {
            if char == '(' {
                opening_pars_found += 1;
            } else if char == ')' {
                closing_pars_found += 1;
            }
        }
        // TODO: Handle nested function calls
        opening_pars_found == closing_pars_found && opening_pars_found > 0
    }

    fn is_variable_name(token: &Token, variables: &[Variable]) -> bool {
        variables.iter().any(|v| v.name == token.value)
    }

    fn is_if_or_else_statement(token: &Token, statement: &Statement, variables: &[Variable]) -> bool {
        if !matches!(token.value.as_str(), "if" | "else") {
            return false;
        }

        let Some(last_token) = statement.tokens.last() else {
            return false;
        };

        if last_token.value != "{" {
            return false;
        }

        match token.value.as_str() {
            "if" => {
                statement.tokens.len() >= 3
                    && (
                    matches!(statement.tokens[1].value.as_str(), "True" | "False")
                        || variables.iter().any(|v|
                        v.name == statement.tokens[1].value
                            && matches!(v.value, Value::Bool(_))
                    )
                )
            }
            "else" => statement.tokens.len() == 2 && statement.tokens[1].value.as_str() == "{",
            _ => false,
        }
    }

    enum StatementType {
        NA,
        FunctionCall,
        VariableDeclaration,
        VariableReassignment,
        IfElse,
    }

    for statement in &lx.statements {
        let mut statement_type: StatementType = StatementType::NA;

        // Determine statement type by reading first token
        let first_token = &statement.tokens[0];
        if is_function_call(first_token) {
            statement_type = StatementType::FunctionCall;
        } else if first_token.value.as_str() == "var" {
            statement_type = StatementType::VariableDeclaration;
        } else if is_variable_name(first_token, &variables) && statement.tokens.len() >= 2 && statement.tokens[1].value == "=" {
            statement_type = StatementType::VariableReassignment; // Doesn't mean it's correct syntax
        } else if is_if_or_else_statement(first_token, &statement, &variables) {
            statement_type = StatementType::IfElse
        }

        match statement_type {
            StatementType::VariableDeclaration => {
                // Ensure there are enough tokens for valid syntax
                if statement.tokens.len() != 4 {
                    throw_syntax_error(format!("{}:{} - Variable definition syntax: var <name> = <value> (no more or less)", &statement.line, &first_token.col))
                }
                // Save variable name and ensure `=` sign is used after variable name
                let second_token = &statement.tokens[1];
                let third_token = &statement.tokens[2];
                let fourth_token = &statement.tokens[3];
                if third_token.value.as_str() == "=" {
                    // correct syntax
                    let new_value = match parse_value(fourth_token.value.as_str()) {
                        Ok(v) => v,
                        Err(e) => throw_error(format!("{}:{} - {}", fourth_token.line, fourth_token.col, e)),
                    };
                    let new_var = Variable::new(second_token.value.clone(), new_value);
                    variables.push(new_var);
                } else {
                    throw_syntax_error(format!("{}:{} - Variable definition syntax: var <name> = <value>", second_token.line, second_token.col))
                }
            }
            StatementType::VariableReassignment => {
                if statement.tokens.len() != 3 {
                    throw_syntax_error(format!("{}:{} - Variable re-assignment syntax: <name> = <value> (no more or less)", &statement.line, &first_token.col))
                }
                let third_token = &statement.tokens[2];

                let new_value = match parse_value(third_token.value.as_str()) {
                    Ok(v) => v,
                    Err(e) => throw_error(format!("{}:{} - {}", third_token.line, third_token.col, e)),
                };

                if let Some(var) = variables.iter_mut().find(|v| v.name == first_token.value) {
                    var.value = new_value;
                } else {
                    throw_error(format!("{}:{} - Unknown variable \"{}\"", first_token.line, first_token.col, first_token.value));
                }
            }
            StatementType::FunctionCall => {
                let Some((fn_name, rest)) = first_token.value.split_once('(') else {
                    // not a function call since there's no '('
                    throw_error(format!("{}:{} - Interpreter thought this was a function but later determined its not. This should be be possible. \"{}\"", first_token.line, first_token.col, first_token.value));
                };

                let Some(builtin) = builtins.get(fn_name) else {
                    throw_error(format!("{}:{} - Unknown function \"{}()\"", first_token.line, first_token.col, fn_name));
                };

                let Some(inner) = rest.strip_suffix(')') else {
                    throw_syntax_error(format!("{}:{} - Unclosed parentheses at function call in \"{}\"", first_token.line, first_token.col, first_token.value));
                };

                let args: Vec<Value> = if inner.is_empty() {
                    Vec::new()
                } else if let Some(s) = inner.strip_prefix('"').and_then(|s| s.strip_suffix('"')) {
                    vec![Value::Str(s.to_string())]
                } else if let Ok(n) = inner.parse::<f64>() {
                    vec![Value::Number(n)]
                } else if let Some(v) = variables.iter().find(|v| v.name == inner.trim()) {
                    vec![v.value.clone()]
                } else {
                    throw_error(format!("{}:{} - could not parse argument \"{}\"", first_token.line, first_token.col, inner));
                };

                if let Some(n) = builtin.arity {
                    if args.len() != n {
                        throw_error(format!("{}:{} - {} expects {} args; got {} instead.", first_token.line, first_token.col, builtin.name, n, args.len()));
                    }
                }

                if let Err(e) = (builtin.func)(args) {
                    throw_error(format!("{}", e));
                }
            }
            StatementType::IfElse => {
                todo!(); // TODO: Handle if-else statements
            }
            _ => {
                throw_error(format!("{}:{} - Unknown statement type", first_token.line, first_token.col))
            }
        }
    }
}
