#![allow(unused)]

#[derive(Debug, PartialEq, Clone, Copy)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Mul,
    Div,
    LParen,
    RParen,
    Neg,
}

fn precedence(t: &Token) -> u8 {
    match t {
        Token::Neg => 3,
        Token::Mul | Token::Div => 2,
        Token::Plus | Token::Minus => 1,
        _ => 0,
    }
}

fn is_left_assoc(t: &Token) -> bool {
    match t {
        Token::Neg => false,
        _ => true,
    }
}

fn is_operator(t: &Token) -> bool {
    matches!(t, Token::Plus | Token::Minus | Token::Mul | Token::Div | Token::Neg)
}

fn tokenize(s: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = s.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch.is_whitespace() {
            continue;
        }
        match ch {
            '+' => tokens.push(Token::Plus),
            '-' => {
                let is_unary = tokens.is_empty()
                    || matches!(
                        tokens.last().unwrap(),
                        Token::Plus | Token::Minus | Token::Mul | Token::Div | Token::LParen | Token::Neg
                    );
                if is_unary {
                    tokens.push(Token::Neg);
                } else {
                    tokens.push(Token::Minus);
                }
            }
            '*' => tokens.push(Token::Mul),
            '/' => tokens.push(Token::Div),
            '(' => tokens.push(Token::LParen),
            ')' => tokens.push(Token::RParen),
            '0'..='9' | '.' => {
                let mut num_str = String::new();
                num_str.push(ch);
                while let Some(&c) = chars.peek() {
                    if c.is_ascii_digit() || c == '.' {
                        num_str.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                let val = num_str.parse::<f64>()
                    .map_err(|_| format!("Invalid number: {}", num_str))?;
                tokens.push(Token::Number(val));
            }
            _ => return Err(format!("Invalid character: {}", ch)),
        }
    }
    Ok(tokens)
}

fn shunting_yard(tokens: Vec<Token>) -> Result<Vec<Token>, String> {
    let mut output = Vec::new();
    let mut op_stack: Vec<Token> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(_) => output.push(token),
            Token::Neg => {
                while let Some(top) = op_stack.last() {
                    if is_operator(top) && (precedence(top) > 3 || (precedence(top) == 3 && is_left_assoc(top))) {
                        output.push(op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(token);
            }
            Token::Plus | Token::Minus => {
                while let Some(top) = op_stack.last() {
                    if is_operator(top) && (precedence(top) > 1 || (precedence(top) == 1 && is_left_assoc(top))) {
                        output.push(op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(token);
            }
            Token::Mul | Token::Div => {
                while let Some(top) = op_stack.last() {
                    if is_operator(top) && (precedence(top) > 2 || (precedence(top) == 2 && is_left_assoc(top))) {
                        output.push(op_stack.pop().unwrap());
                    } else {
                        break;
                    }
                }
                op_stack.push(token);
            }
            Token::LParen => op_stack.push(token),
            Token::RParen => {
                let mut found = false;
                while let Some(top) = op_stack.pop() {
                    if top == Token::LParen {
                        found = true;
                        break;
                    } else {
                        output.push(top);
                    }
                }
                if !found {
                    return Err("Mismatched parentheses".to_string());
                }
            }
        }
    }

    while let Some(top) = op_stack.pop() {
        if top == Token::LParen || top == Token::RParen {
            return Err("Mismatched parentheses".to_string());
        }
        output.push(top);
    }

    Ok(output)
}

fn evaluate_rpn(tokens: Vec<Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();
    for token in tokens {
        match token {
            Token::Number(n) => stack.push(n),
            Token::Plus => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a + b);
            }
            Token::Minus => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a - b);
            }
            Token::Mul => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(a * b);
            }
            Token::Div => {
                let b = stack.pop().ok_or("Invalid expression")?;
                let a = stack.pop().ok_or("Invalid expression")?;
                if b == 0.0 {
                    return Err("Division by zero".to_string());
                }
                stack.push(a / b);
            }
            Token::Neg => {
                let a = stack.pop().ok_or("Invalid expression")?;
                stack.push(-a);
            }
            _ => return Err("Invalid token in RPN".to_string()),
        }
    }
    if stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }
    Ok(stack[0])
}

pub fn evaluate_expression(expression: &str) -> Result<String, String> {
    let tokens = tokenize(expression)?;
    if tokens.is_empty() {
        return Err("Empty expression".to_string());
    }
    let rpn = shunting_yard(tokens)?;
    let result = evaluate_rpn(rpn)?;
    Ok(format!("{}", result))
}
