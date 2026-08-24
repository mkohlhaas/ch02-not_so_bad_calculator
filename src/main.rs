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

fn evaluate_expression(expression: &str) -> Result<String, String> {
    let tokens = tokenize(expression)?;
    if tokens.is_empty() {
        return Err("Empty expression".to_string());
    }
    let rpn = shunting_yard(tokens)?;
    let result = evaluate_rpn(rpn)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addition() {
        assert_eq!(evaluate_expression("2 + 3").unwrap(), "5");
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(evaluate_expression("10 - 4").unwrap(), "6");
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(evaluate_expression("3 * 4").unwrap(), "12");
    }

    #[test]
    fn test_division() {
        assert_eq!(evaluate_expression("8 / 2").unwrap(), "4");
    }

    #[test]
    fn test_precedence_mul_over_add() {
        assert_eq!(evaluate_expression("2 + 3 * 4").unwrap(), "14");
    }

    #[test]
    fn test_precedence_div_over_sub() {
        assert_eq!(evaluate_expression("10 - 6 / 2").unwrap(), "7");
    }

    #[test]
    fn test_parentheses() {
        assert_eq!(evaluate_expression("(2 + 3) * 4").unwrap(), "20");
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(evaluate_expression("((1 + 2) * 3)").unwrap(), "9");
    }

    #[test]
    fn test_unary_negation() {
        assert_eq!(evaluate_expression("-5").unwrap(), "-5");
    }

    #[test]
    fn test_unary_negation_with_paren() {
        assert_eq!(evaluate_expression("-(3 + 2)").unwrap(), "-5");
    }

    #[test]
    fn test_unary_negation_after_op() {
        assert_eq!(evaluate_expression("2 * -3").unwrap(), "-6");
    }

    #[test]
    fn test_negative_result() {
        assert_eq!(evaluate_expression("3 - 10").unwrap(), "-7");
    }

    #[test]
    fn test_division_by_zero() {
        assert!(evaluate_expression("1 / 0").is_err());
    }

    #[test]
    fn test_division_by_zero_in_expression() {
        assert!(evaluate_expression("10 / (5 - 5)").is_err());
    }

    #[test]
    fn test_whitespace_ignored() {
        assert_eq!(evaluate_expression("  2  +  3  ").unwrap(), "5");
    }

    #[test]
    fn test_decimal_numbers() {
        assert_eq!(evaluate_expression("3.5 + 2.5").unwrap(), "6");
    }

    #[test]
    fn test_multiple_operations() {
        assert_eq!(evaluate_expression("1 + 2 + 3 + 4").unwrap(), "10");
    }

    #[test]
    fn test_mixed_precedence() {
        assert_eq!(evaluate_expression("2 * 3 + 4 * 5").unwrap(), "26");
    }

    #[test]
    fn test_zero_value() {
        assert_eq!(evaluate_expression("0").unwrap(), "0");
    }

    #[test]
    fn test_zero_dividend() {
        assert_eq!(evaluate_expression("0 / 5").unwrap(), "0");
    }

    #[test]
    fn test_negative_division() {
        assert_eq!(evaluate_expression("-10 / 2").unwrap(), "-5");
    }

    #[test]
    fn test_negative_division_negative_divisor() {
        assert_eq!(evaluate_expression("10 / -2").unwrap(), "-5");
    }

    #[test]
    fn test_subtraction_negative() {
        assert_eq!(evaluate_expression("5 - -3").unwrap(), "8");
    }

    #[test]
    fn test_invalid_characters() {
        assert!(evaluate_expression("2 + a").is_err());
    }

    #[test]
    fn test_unexpected_end() {
        assert!(evaluate_expression("2 +").is_err());
    }

    #[test]
    fn test_unexpected_characters_at_end() {
        assert!(evaluate_expression("2 2").is_err());
    }

    #[test]
    fn test_missing_closing_paren() {
        assert!(evaluate_expression("(2 + 3").is_err());
    }

    #[test]
    fn test_extra_closing_paren() {
        assert!(evaluate_expression("2 + 3)").is_err());
    }

    #[test]
    fn test_empty_string() {
        assert!(evaluate_expression("").is_err());
    }

    #[test]
    fn test_only_spaces() {
        assert!(evaluate_expression("   ").is_err());
    }

    #[test]
    fn test_complex_expression() {
        assert_eq!(evaluate_expression("3 + 4 * 2 / (1 - 5) * 2").unwrap(), "-1");
    }

    #[test]
    fn test_single_number() {
        assert_eq!(evaluate_expression("42").unwrap(), "42");
    }

    #[test]
    fn test_negative_single_number() {
        assert_eq!(evaluate_expression("-42").unwrap(), "-42");
    }

    #[test]
    fn test_paren_with_negation_inside() {
        assert_eq!(evaluate_expression("(-3 + 5) * 2").unwrap(), "4");
    }

    #[test]
    fn test_multiple_divisions() {
        assert_eq!(evaluate_expression("100 / 10 / 2").unwrap(), "5");
    }

    #[test]
    fn test_multiple_subtractions() {
        assert_eq!(evaluate_expression("10 - 3 - 2").unwrap(), "5");
    }

    #[test]
    fn test_mixed_unary_and_binary() {
        assert_eq!(evaluate_expression("-2 * -3").unwrap(), "6");
    }

    #[test]
    fn test_large_numbers() {
        assert_eq!(evaluate_expression("100000 + 200000").unwrap(), "300000");
    }

    #[test]
    fn test_fractional_division() {
        assert_eq!(evaluate_expression("5 / 2").unwrap(), "2.5");
    }

    #[test]
    fn test_negative_fractional_division() {
        assert_eq!(evaluate_expression("-5 / 2").unwrap(), "-2.5");
    }

    #[test]
    fn test_expression_with_tabs() {
        assert_eq!(evaluate_expression("2\t+\t3").unwrap(), "5");
    }

    #[test]
    fn test_expression_with_newlines() {
        assert_eq!(evaluate_expression("2 +\n3").unwrap(), "5");
    }

    #[test]
    fn test_deep_nesting() {
        assert_eq!(evaluate_expression("((2 + 3) * (4 - 1)) / 3").unwrap(), "5");
    }

    #[test]
    fn test_multiple_unary_negation() {
        assert_eq!(evaluate_expression("--5").unwrap(), "5");
    }

    #[test]
    fn test_unary_after_paren() {
        assert_eq!(evaluate_expression("(-2) * (-3)").unwrap(), "6");
    }

    #[test]
    fn test_complex_precedence_chain() {
        assert_eq!(evaluate_expression("2 + 3 * 4 - 5 / 2").unwrap(), "11.5");
    }

    #[test]
    fn test_division_chain_left_assoc() {
        assert_eq!(evaluate_expression("100 / 2 / 5").unwrap(), "10");
    }

    #[test]
    fn test_subtraction_chain_left_assoc() {
        assert_eq!(evaluate_expression("10 - 3 - 2 - 1").unwrap(), "4");
    }

    #[test]
    fn test_paren_division() {
        assert_eq!(evaluate_expression("(10 + 2) / (3 + 1)").unwrap(), "3");
    }

    #[test]
    fn test_unary_negation_decimal() {
        assert_eq!(evaluate_expression("-3.5 * 2").unwrap(), "-7");
    }

    #[test]
    fn test_nested_unary_and_binary() {
        assert_eq!(evaluate_expression("-(2 + 3) * 4").unwrap(), "-20");
    }

    #[test]
    fn test_mixed_spaces_and_tabs() {
        assert_eq!(evaluate_expression("  ( 1 + 2 ) * ( 3 + 4 )  ").unwrap(), "21");
    }

    #[test]
    fn test_negative_paren_expression() {
        assert_eq!(evaluate_expression("-(1 + 2 + 3)").unwrap(), "-6");
    }

    #[test]
    fn test_complex_mixed_operators() {
        assert_eq!(evaluate_expression("2 * 3 + 4 * 5 - 6 / 2").unwrap(), "23");
    }
}
