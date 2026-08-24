use calculator::evaluate_expression;

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
