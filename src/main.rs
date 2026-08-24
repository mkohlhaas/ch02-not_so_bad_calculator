#![allow(unused)]

use std::io::Write;
use std::process::exit;

enum Operand {
    Value(f64),
}

impl Operand {
    fn evaluate(&self) -> f64 {
        match self {
            Operand::Value(v) => *v,
        }
    }
}

enum Operator {
    Addition { lhs: Operand, rhs: Operand },
    Subtraction { lhs: Operand, rhs: Operand },
    Multiplication { lhs: Operand, rhs: Operand },
    Division { lhs: Operand, rhs: Operand },
    Negation { operand: Operand },
}

impl Operator {
    fn apply(&self) -> Operand {
        let inner = match self {
            Operator::Addition { lhs, rhs } => lhs.evaluate() + rhs.evaluate(),
            Operator::Subtraction { lhs, rhs } => lhs.evaluate() - rhs.evaluate(),
            Operator::Multiplication { lhs, rhs } => lhs.evaluate() * rhs.evaluate(),
            Operator::Division { lhs, rhs } => lhs.evaluate() / rhs.evaluate(),
            Operator::Negation { operand } => -operand.evaluate(),
        };
        Operand::Value(inner)
    }

    fn precedence(&self) -> u8 {
        match self {
            Operator::Addition { .. } | Operator::Subtraction { .. } => 0,
            Operator::Multiplication { .. } | Operator::Division { .. } => 1,
            Operator::Negation { .. } => 2,
        }
    }

    fn symbol(&self) -> char {
        match self {
            Operator::Addition { .. } => '+',
            Operator::Subtraction { .. } => '-',
            Operator::Multiplication { .. } => '*',
            Operator::Division { .. } => '/',
            Operator::Negation { .. } => '-',
        }
    }
}

fn skip_spaces(s: &str, i: &mut usize) {
    while *i < s.len() && s.as_bytes()[*i].is_ascii_whitespace() {
        *i += 1;
    }
}

fn parse_number(s: &str, i: &mut usize) -> Result<f64, String> {
    skip_spaces(s, i);
    let start = *i;
    while *i < s.len() {
        let b = s.as_bytes()[*i];
        if b.is_ascii_digit() || b == b'.' {
            *i += 1;
        } else {
            break;
        }
    }
    if start == *i {
        return Err("Expected number".to_string());
    }
    s[start..*i].parse::<f64>().map_err(|_| "Invalid number".to_string())
}

fn parse_expression(s: &str, i: &mut usize) -> Result<f64, String> {
    let mut lhs = parse_term(s, i)?;
    loop {
        skip_spaces(s, i);
        if *i >= s.len() {
            break;
        }
        let ch = s.as_bytes()[*i] as char;
        if ch == '+' || ch == '-' {
            *i += 1;
            let rhs = parse_term(s, i)?;
            if ch == '+' {
                lhs += rhs;
            } else {
                lhs -= rhs;
            }
        } else {
            break;
        }
    }
    Ok(lhs)
}

fn parse_term(s: &str, i: &mut usize) -> Result<f64, String> {
    let mut lhs = parse_factor(s, i)?;
    loop {
        skip_spaces(s, i);
        if *i >= s.len() {
            break;
        }
        let ch = s.as_bytes()[*i] as char;
        if ch == '*' || ch == '/' {
            *i += 1;
            let rhs = parse_factor(s, i)?;
            if ch == '*' {
                lhs *= rhs;
            } else {
                if rhs == 0.0 {
                    return Err("Division by zero".to_string());
                }
                lhs /= rhs;
            }
        } else {
            break;
        }
    }
    Ok(lhs)
}

fn parse_factor(s: &str, i: &mut usize) -> Result<f64, String> {
    skip_spaces(s, i);
    if *i < s.len() && s.as_bytes()[*i] == b'-' {
        *i += 1;
        let val = parse_factor(s, i)?;
        return Ok(-val);
    }
    if *i < s.len() && s.as_bytes()[*i] == b'(' {
        *i += 1;
        let val = parse_expression(s, i)?;
        skip_spaces(s, i);
        if *i >= s.len() || s.as_bytes()[*i] != b')' {
            return Err("Expected ')'".to_string());
        }
        *i += 1;
        return Ok(val);
    }
    parse_number(s, i)
}

fn evaluate_expression(expression: &str) -> Result<String, String> {
    let mut i = 0;
    let result = parse_expression(expression, &mut i)?;
    skip_spaces(expression, &mut i);
    if i != expression.len() {
        return Err("Unexpected characters at end".to_string());
    }
    Ok(format!("{}", result))
}

fn main() {
    let mut buf = String::new();

    loop {
        print!("> ");
        std::io::stdout().flush().unwrap();

        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();

        if buf.trim() == "exit" {
            exit(0)
        }

        match evaluate_expression(&buf) {
            Ok(result) => println!("{result}"),
            Err(error) => println!("Error: {error}"),
        }
    }
}
