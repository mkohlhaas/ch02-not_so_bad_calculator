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
        assert_eq!(evaluate_expression("3 + 4 * 2 / (1 - 5) * 2").unwrap(), "1");
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
}
