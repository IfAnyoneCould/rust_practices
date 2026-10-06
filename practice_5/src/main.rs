use std::fmt;

#[derive(Debug)]
enum CalcError {
    UnexpectedChar(char),
    UnexpectedToken(Token),
    MalformedToken(String),
    MissingOperand,
    UnexpectedEnd,
    DivisionByZero,
    UnpairedParentheses,
    //continue these are needed
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CalcError::UnexpectedChar(c) => write!(f, "Error: unexpected character: {c}"),
            CalcError::UnexpectedToken(t) => write!(f, "Error: unexpected token: {t}"),
            CalcError::MalformedToken(t) => write!(f, "Error: malformed token during pasrse: {t}"),
            CalcError::MissingOperand => write!(f, "Error: missing operand"),
            CalcError::UnexpectedEnd => write!(f, "Error: unexpected ending"),
            CalcError::DivisionByZero => write!(f, "Error: division by zero"),
            CalcError::UnpairedParentheses => write!(f, "Error: unpaired paranthesis"),
        }
    }
}

impl std::error::Error for CalcError {}

#[derive(Debug, Clone, PartialEq, Copy)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    LParen,
    RParen,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Token::Number(n) => write!(f, "{n}"),
            Token::Plus => write!(f, "+"),
            Token::Minus => write!(f, "-"),
            Token::Star => write!(f, "*"),
            Token::Slash => write!(f, "/"),
            Token::LParen => write!(f, "("),
            Token::RParen => write!(f, ")"),
        }
    }
}

fn print_token_list(input: &Vec<Token>) {
    for t in input {
        print!("{t} ");
    }
    println!();
}

fn tokenize(input: &str) -> Result<Vec<Token>, CalcError> {
    let mut result: Vec<Token> = Vec::new();
    let mut current = String::new();

    let clear_and_push = |current: &mut String, result: &mut Vec<Token>| -> Result<(), CalcError> {
        if !current.is_empty() {
            let num: f64 = current
                .parse()
                .map_err(|_| CalcError::MalformedToken(current.to_string()))?;
            result.push(Token::Number(num));
            current.clear();
        }
        Ok(())
    };

    for c in input.chars() {
        let token = match c {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Star),
            '/' => Some(Token::Slash),
            '(' => Some(Token::LParen),
            ')' => Some(Token::RParen),
            _ => None,
        };

        match token {
            Some(t) => {
                clear_and_push(&mut current, &mut result)?;
                result.push(t);
            }
            None => {
                if c != ' ' {
                    current.push(c)
                } else {
                    clear_and_push(&mut current, &mut result)?
                }
            }
        }
    }
    clear_and_push(&mut current, &mut result)?;
    Ok(result)
}

fn prec(op: Token) -> u8 {
    match op {
        Token::Plus | Token::Minus => 1,
        Token::Star | Token::Slash => 2,
        _ => 0,
    }
}

fn build_expr(input: &Vec<Token>) -> Result<Vec<Token>, CalcError> {
    let mut output: Vec<Token> = Vec::new();
    let mut operator: Vec<Token> = Vec::new();
    for t in input {
        match t {
            Token::Number(_) => output.push(*t),
            Token::LParen => {
                operator.push(*t);
            }
            Token::RParen => {
                let mut found_pair = false;
                while let Some(o2) = operator.pop() {
                    if o2 == Token::LParen {
                        found_pair = true;
                        break;
                    }
                    output.push(o2);
                }
                if !found_pair {
                    return Err(CalcError::UnpairedParentheses);
                }
            }
            _ => {
                while let Some(o2) = operator.last() {
                    if prec(*o2) >= prec(*t) {
                        output.push(operator.pop().unwrap());
                    } else {
                        break;
                    }
                }
                operator.push(*t);
            }
        }
    }
    for _ in 0..operator.len() {
        let t = operator.pop().unwrap();
        if t != Token::LParen {
            output.push(t);
        }
    }
    Ok(output)
}

fn evaluate_expr(input: Vec<Token>) -> Result<f64, CalcError> {
    let mut stack: Vec<f64> = Vec::new();
    for t in input {
        match t {
            Token::Number(n) => stack.push(n),
            op => {
                let r = stack.pop().ok_or(CalcError::MissingOperand)?;
                let l = stack.pop().ok_or(CalcError::MissingOperand)?;
                let result = match op {
                    Token::Plus => l + r,
                    Token::Minus => l - r,
                    Token::Star => l * r,
                    Token::Slash => {
                        if r != 0.0 {
                            l / r
                        } else {
                            return Err(CalcError::DivisionByZero);
                        }
                    }
                    e => return Err(CalcError::UnexpectedToken(e)),
                };
                stack.push(result);
            }
        }
    }
    match stack.as_slice() {
        [v] => Ok(*v),
        [] => Err(CalcError::UnexpectedEnd),
        _ => Err(CalcError::MissingOperand),
    }
}

fn main() {
    let input = "1 + (2 * 3.14 +4) / 11";
    let tokens = tokenize(input);

    match &tokens {
        Ok(v) => print_token_list(v),
        Err(e) => {
            println!("{e}");
            return;
        }
    }

    let parsed = build_expr(&tokens.unwrap());

    match &parsed {
        Ok(v) => print_token_list(v),
        Err(e) => {
            print!("{e}");
            return;
        }
    }

    let evaluated = evaluate_expr(parsed.unwrap());
    match &evaluated {
        Ok(v) => println!("{v}"),
        Err(e) => println!("{e}"),
    }
}
